use picosdk_malloc::{heapref::OVERHEAD, trace, Heap, HeapRef};

pub fn heapref_new_from_heap() {
    // Allocate memory
    let heap = Heap::new(*b"Testolope").expect("failed to allocate memory");

    // Move the object into a heapref
    let heapref = HeapRef::new_from_heap(heap).expect("failed to allocate memory");
    assert_eq!(unsafe { trace::allocated() }, 9 + OVERHEAD, "invalid amount of allocated bytes");
    drop(heapref);
}

pub fn heapref_new() {
    // Allocate memory
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    assert_eq!(unsafe { trace::allocated() }, 9 + OVERHEAD, "invalid amount of allocated bytes");
    drop(heapref);
}

pub fn heapref_inner() {
    // Allocate memory and reference it
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    let value = heapref.inner();
    assert_eq!(value, b"Testolope", "invalid value on heap");
}

pub fn heapref_strong() {
    // Allocate memory
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    assert_eq!(heapref.strong(), 1, "invalid strong reference count");
}

pub fn heapref_weak() {
    // Allocate memory
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    assert_eq!(heapref.weak(), 0, "invalid weak reference count");
}

pub fn heapref_downgrade() {
    // Allocate memory
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    let weak = heapref.downgrade();
    assert_eq!(heapref.weak(), 1, "invalid weak reference count");

    // Drop the weak reference
    drop(weak);
    assert_eq!(heapref.weak(), 0, "invalid weak reference count");
}

pub fn heapref_try_unwrap_heap() {
    // Allocate memory and clone heapref
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    let clone = heapref.clone();

    // Try to unwrap the heap object and drop the clone
    let heapref = heapref.try_unwrap_heap().expect_err("no error when upgrading shared heap reference");
    drop(clone);

    // Unwrap the heap object
    let value = heapref.try_unwrap_heap().expect("failed to unwrap exclusive heap reference");
    assert_eq!(unsafe { trace::allocated() }, 9, "invalid amount of allocated bytes");
    drop(value);
}

pub fn heapref_try_unwrap() {
    // Allocate memory and clone heapref
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    let clone = heapref.clone();

    // Try to unwrap the heap object and drop the clone
    let heapref = heapref.try_unwrap().expect_err("no error when upgrading shared heap reference");
    drop(clone);

    // Unwrap the heap object
    let value = heapref.try_unwrap().expect("failed to unwrap exclusive heap reference");
    assert_eq!(unsafe { trace::allocated() }, 0, "invalid amount of allocated bytes");
    drop(value);
}

pub fn heapref_clone() {
    // Allocate memory
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    let clone = heapref.clone();
    assert_eq!(heapref.strong(), 2, "invalid strong reference count");

    // Drop clone
    drop(clone);
    assert_eq!(heapref.strong(), 1, "invalid strong reference count");
}

pub fn heaprefweak_strong() {
    // Allocate memory
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    let weak = heapref.downgrade();

    // Validate refcount
    assert_eq!(heapref.strong(), 1, "invalid strong reference count");
    drop(weak);
}

pub fn heaprefweak_weak() {
    // Allocate memory
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    let weak = heapref.downgrade();

    // Validate refcount
    assert_eq!(weak.weak(), 1, "invalid weak reference count");
}

pub fn heaprefweak_upgrade() {
    // Allocate memory
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    let weak = heapref.downgrade();
    assert_eq!(weak.strong(), 1, "invalid strong reference count");

    // Validate refcount
    let heapref2 = weak.upgrade().expect("failed to upgrade weak reference");
    assert_eq!(weak.strong(), 2, "invalid strong reference count");

    // Drop strong references
    drop(heapref);
    drop(heapref2);
    assert!(weak.upgrade().is_none(), "no error when upgrading orhpaned weak reference");
}

pub fn heaprefweak_clone() {
    // Allocate memory
    let heapref = HeapRef::new(*b"Testolope").expect("failed to allocate memory");
    let weak = heapref.downgrade();
    assert_eq!(weak.weak(), 1, "invalid weak reference count");

    // Clone the weak reference
    let clone = weak.clone();
    assert_eq!(weak.weak(), 2, "invalid weak reference count");

    // Drop clone
    drop(clone);
    assert_eq!(weak.weak(), 1, "invalid weak reference count");
}
