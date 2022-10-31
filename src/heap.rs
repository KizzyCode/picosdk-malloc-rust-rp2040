//! A `malloc`/`free`-managed heap object

use crate::trace;
use core::{
    ffi::c_void,
    fmt::{self, Debug, Formatter},
    mem::{self, MaybeUninit},
    ops::{Deref, DerefMut},
};

// Bindings to `malloc` and `free`
extern "C" {
    /// Allocates some memory
    fn malloc(size: usize) -> *mut c_void;
    /// Frees some allocated memory
    fn free(ptr: *mut c_void);
}

/// A `malloc`/`free`-managed heap object
#[repr(transparent)]
pub struct Heap<T> {
    /// The heap pointer
    memory: *mut T,
}
impl<T> Heap<MaybeUninit<T>> {
    /// Creates a new uninitialized array
    pub fn new_uninit() -> Option<Self> {
        // Allocate the memory
        let memory = unsafe { malloc(Self::SIZE) as *mut MaybeUninit<T> };
        if memory.is_null() {
            return None;
        }

        // Trace the memory
        unsafe { trace::increment_allocated(Self::SIZE) };
        Some(Self { memory })
    }

    /// Assumes that the array has been initialized
    ///
    /// # Safety
    /// See
    /// [core::mem::MaybeUninit::assume_init](https://doc.rust-lang.org/stable/core/mem/union.MaybeUninit.html#method.assume_init)
    /// for more information.
    pub unsafe fn assume_init(self) -> Heap<T> {
        // Destructure and forget `self` to avoid double-free during `drop()`
        let memory = self.memory;
        mem::forget(self);

        // Create a new instance with the appropriate pointer type
        Heap { memory: memory.cast() }
    }
}
impl<T> Heap<T> {
    /// The amount of heap-allocated memory in bytes
    pub const SIZE: usize = mem::size_of::<T>();

    /// Moves `value` to the heap
    pub fn new(value: T) -> Result<Self, T> {
        // Allocate the memory
        let mut this = match Heap::new_uninit() {
            Some(this) => this,
            None => return Err(value),
        };

        // Initialize the memory
        this.write(value);
        Ok(unsafe { this.assume_init() })
    }
    /// Creates a heap object from a raw pointer that has been created with `Heap::into_raw`
    ///
    /// # Safety
    /// This function is unsafe because improper use may lead to memory problems. For example, a double-free may occur if
    /// the function is called twice on the same raw pointer.
    pub unsafe fn from_raw(memory: *mut T) -> Self {
        assert!(!memory.is_null(), "unexpected null pointer");
        Self { memory }
    }

    /// Returns the underlying element
    pub fn into_inner(self) -> T {
        // Take the element and free the allocated memory
        let element = unsafe { self.memory.read() };
        unsafe { free(self.memory as *mut c_void) };

        // Trace the memory
        let size = mem::size_of::<T>();
        unsafe { trace::decrement_allocated(size) };

        // Forget `self` to avoid double-free during `drop()`
        mem::forget(self);
        element
    }
    /// Consumes `self` and returns the underlying raw pointer
    ///
    /// # Note
    /// The resulting raw pointer is unmanaged. To release it, recreate a `Heap` object from it with `Heap::from_raw` and
    /// drop it accordingly.
    pub const fn into_raw(self) -> *mut T {
        // Destructure `self` and forget it to avoid that the memory is deallocated on drop
        let memory = self.memory;
        mem::forget(self);
        memory
    }

    /// A reference to the inner object
    pub fn inner(&self) -> &T {
        let reference = unsafe { self.memory.as_ref() };
        reference.expect("unexpected null pointer")
    }
    /// A mutable reference to the inner object
    pub fn inner_mut(&mut self) -> &mut T {
        let reference = unsafe { self.memory.as_mut() };
        reference.expect("unexpected null pointer")
    }
}
impl<const LEN: usize, T> Heap<[T; LEN]> {
    /// Allocates a new segment on the heap and initializes it with `T::default()`
    pub fn new_default() -> Option<Self>
    where
        T: Default,
    {
        Self::new_from_fn(T::default)
    }
    /// Allocates a new segment on the heap and initializes it with the return values of `generator`
    pub fn new_from_fn<F>(mut generator: F) -> Option<Self>
    where
        F: FnMut() -> T,
    {
        // Allocate the memory
        let mut this = Heap::new_uninit()?;

        // Write the elements
        let memory = this.as_mut_ptr() as *mut T;
        for nth in 0..LEN {
            let value = generator();
            let ptr = unsafe { memory.add(nth) };
            unsafe { ptr.write(value) };
        }

        // Return the new instance
        Some(unsafe { this.assume_init() })
    }
}
impl<T> Deref for Heap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}
impl<T> DerefMut for Heap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}
impl<T> AsRef<T> for Heap<T> {
    fn as_ref(&self) -> &T {
        self.inner()
    }
}
impl<T> AsMut<T> for Heap<T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}
impl<T> Debug for Heap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}
impl<T> Drop for Heap<T> {
    fn drop(&mut self) {
        // Drop the element and release the memory
        unsafe { self.memory.drop_in_place() };
        unsafe { free(self.memory as *mut c_void) }

        // Trace the memory
        let size = mem::size_of::<T>();
        unsafe { trace::decrement_allocated(size) };
    }
}
