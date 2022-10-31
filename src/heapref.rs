//! A `malloc`/`free`-managed reference-counted heap object

use crate::heap::Heap;
use core::{
    fmt::{self, Debug, Formatter},
    mem,
    ops::Deref,
};

/// A shared reference counter
struct RefCounter {
    /// The amount of strong references
    pub strong: usize,
    /// The amount of weak references
    pub weak: usize,
}

/// The memory overhead for the reference counters in bytes
pub const OVERHEAD: usize = mem::size_of::<RefCounter>();

/// A reference counted heap object
pub struct HeapRef<T> {
    /// The referenced value
    value: *mut T,
    /// The reference counter
    refctr: *mut RefCounter,
}
impl<T> HeapRef<T> {
    /// The amount of heap-allocated memory in bytes
    pub const SIZE: usize = mem::size_of::<T>() + OVERHEAD;

    /// Creates a new reference counted heap object from the given heap object
    pub fn new_from_heap(value: Heap<T>) -> Result<Self, Heap<T>> {
        // Create a reference counter that resembles one strong reference
        let refctr = RefCounter { strong: 1, weak: 0 };

        // Move the reference counter to the heap
        let refctr = match Heap::new(refctr) {
            Ok(refctr) => refctr,
            Err(_) => return Err(value),
        };

        Ok(Self { value: value.into_raw(), refctr: refctr.into_raw() })
    }
    /// Creates a new reference counted heap object with the given value
    pub fn new(value: T) -> Result<Self, T> {
        // Move the value to the heap
        let value = Heap::new(value)?;

        // Create the reference counted heap object
        match Self::new_from_heap(value) {
            Ok(this) => Ok(this),
            Err(value) => Err(value.into_inner()),
        }
    }

    /// A reference to the underlying value
    pub fn inner(&self) -> &T {
        let reference = unsafe { self.value.as_ref() };
        reference.expect("unexpected null pointer")
    }

    /// The amount of strong references to the underlying value
    pub fn strong(&self) -> usize {
        unsafe { (*self.refctr).strong }
    }
    /// The amount of weak references to the underlying value
    pub fn weak(&self) -> usize {
        unsafe { (*self.refctr).weak }
    }

    /// Creates a weak reference to the heap allocated object
    pub fn downgrade(&self) -> HeapRefWeak<T> {
        unsafe { (*self.refctr).weak += 1 };
        HeapRefWeak { value: self.value, refctr: self.refctr }
    }

    /// Returns the underlying element as heap-object
    pub fn try_unwrap_heap(self) -> Result<Heap<T>, Self> {
        // Ensure that we are the last strong reference
        if self.strong() > 1 {
            return Err(self);
        }

        // Take the value and set the reference counter to zero
        let value = unsafe { Heap::from_raw(self.value) };
        unsafe { (*self.refctr).strong = 0 };

        // Deallocate the reference counter if there are no weak references left
        if self.weak() == 0 {
            let refctr = unsafe { Heap::from_raw(self.refctr) };
            drop(refctr);
        }

        // Forget `self` to avoid double-free during `drop()`
        mem::forget(self);
        Ok(value)
    }
    /// Returns the underlying element
    pub fn try_unwrap(self) -> Result<T, Self> {
        let value = self.try_unwrap_heap()?;
        Ok(value.into_inner())
    }
}
impl<T> Deref for HeapRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}
impl<T> AsRef<T> for HeapRef<T> {
    fn as_ref(&self) -> &T {
        self.inner()
    }
}
impl<T> Debug for HeapRef<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.inner().fmt(f)
    }
}
impl<T> Clone for HeapRef<T> {
    fn clone(&self) -> Self {
        unsafe { (*self.refctr).strong += 1 };
        Self { value: self.value, refctr: self.refctr }
    }
}
impl<T> Drop for HeapRef<T> {
    fn drop(&mut self) {
        // Decrement the reference counter
        unsafe { (*self.refctr).strong -= 1 };

        // Deallocate the value if we are the last strong reference
        if self.strong() == 0 {
            let value = unsafe { Heap::from_raw(self.value) };
            drop(value);
        }

        // Deallocate the reference counter if we are the last reference
        if self.strong() == 0 && self.weak() == 0 {
            let refctr = unsafe { Heap::from_raw(self.refctr) };
            drop(refctr);
        }
    }
}

/// A weak reference to a reference counted heap object
pub struct HeapRefWeak<T> {
    /// The referenced value
    value: *mut T,
    /// The reference counter
    refctr: *mut RefCounter,
}
impl<T> HeapRefWeak<T> {
    /// The amount of strong references to the underlying value
    pub fn strong(&self) -> usize {
        unsafe { (*self.refctr).strong }
    }
    /// The amount of weak references to the underlying value
    pub fn weak(&self) -> usize {
        unsafe { (*self.refctr).weak }
    }

    /// Tries to create a strong reference to the heap object
    pub fn upgrade(&self) -> Option<HeapRef<T>> {
        // Ensure that there is at least one strong reference left
        if self.strong() == 0 {
            return None;
        }

        // Update the reference counter and create the reference
        unsafe { (*self.refctr).strong += 1 };
        Some(HeapRef { value: self.value, refctr: self.refctr })
    }
}
impl<T> Clone for HeapRefWeak<T> {
    fn clone(&self) -> Self {
        unsafe { (*self.refctr).weak += 1 };
        Self { value: self.value, refctr: self.refctr }
    }
}
impl<T> Drop for HeapRefWeak<T> {
    fn drop(&mut self) {
        // Decrement the reference counter
        unsafe { (*self.refctr).weak -= 1 };

        // Deallocate the reference counter if we are the last reference
        if self.strong() == 0 && self.weak() == 0 {
            let refctr = unsafe { Heap::from_raw(self.refctr) };
            drop(refctr);
        }
    }
}
