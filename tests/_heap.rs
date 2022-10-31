use core::{default::Default, mem::MaybeUninit};
use picosdk_malloc::{trace, Heap};

pub fn uninit() {
    // Allocate memory
    let heap: Heap<MaybeUninit<[u8; 9]>> = Heap::new_uninit().expect("failed to allocate memory");
    assert_eq!(unsafe { trace::allocated() }, 9, "invalid amount of allocated bytes");
    drop(heap);
}

pub fn assume_init() {
    // Allocate memory
    let mut heap: Heap<MaybeUninit<[u8; 9]>> = Heap::new_uninit().expect("failed to allocate memory");
    heap.write(*b"Testolope");

    // Validate value
    let heap = unsafe { heap.assume_init() };
    assert_eq!(heap.inner(), b"Testolope", "invalid value on heap");
}

pub fn new() {
    // Allocate memory
    let heap = Heap::new(*b"Testolope").expect("failed to allocate memory");
    assert_eq!(heap.inner(), b"Testolope", "invalid value on heap");
    assert_eq!(unsafe { trace::allocated() }, 9, "invalid amount of allocated bytes");

    // Deallocate memory
    drop(heap);
}

pub fn from_raw() {
    // Create raw pointer
    let heap = Heap::new(*b"Testolope").expect("failed to allocate memory");
    let memory = heap.into_raw();

    // Load raw pointer
    let heap = unsafe { Heap::from_raw(memory) };
    assert_eq!(heap.inner(), b"Testolope", "invalid value on heap");
    assert_eq!(unsafe { trace::allocated() }, 9, "invalid amount of allocated bytes");
}

pub fn into_inner() {
    // Allocate memory
    let heap = Heap::new(*b"Testolope").expect("failed to allocate memory");

    // Take the value
    let value = heap.into_inner();
    assert_eq!(value, *b"Testolope", "invalid value on heap");
    assert_eq!(unsafe { trace::allocated() }, 0, "invalid amount of allocated bytes");
}

pub fn into_raw() {
    // Create raw pointer
    let heap = Heap::new(*b"Testolope").expect("failed to allocate memory");
    let memory = heap.into_raw();
    assert_eq!(unsafe { trace::allocated() }, 9, "invalid amount of allocated bytes");

    // Compare the bytes
    let value = unsafe { memory.read() };
    assert_eq!(value, *b"Testolope", "invalid value on heap");

    // Drop memory
    let heap = unsafe { Heap::from_raw(memory) };
    drop(heap);
}

pub fn inner() {
    // Allocate memory and reference it
    let heap = Heap::new(*b"Testolope").expect("failed to allocate memory");
    let value = heap.inner();

    // Validate value
    assert_eq!(value, b"Testolope", "invalid value on heap");
}

pub fn inner_mut() {
    // Allocate memory
    let mut heap = Heap::new(*b"Testolope").expect("failed to allocate memory");

    // Modify and validate value
    let value = heap.inner_mut();
    value.make_ascii_uppercase();
    assert_eq!(value, b"TESTOLOPE", "invalid value on heap");
}

pub fn new_default() {
    /// A testing struct with a non-`0` default value
    struct Testolope(u8);
    impl Default for Testolope {
        fn default() -> Self {
            Self(0x07)
        }
    }

    // Allocate memory
    let heap: Heap<[Testolope; 9]> = Heap::new_default().expect("failed to allocate memory");
    let all_0x07 = heap.iter().all(|value| value.0 == 0x07);
    assert!(all_0x07, "invalid value on heap");
}

pub fn new_from_fn() {
    // Create the init function
    let mut iterator = b"Testolope".into_iter();
    let generator = || *iterator.next().expect("init function is exhausted");

    // Allocate memory
    let heap: Heap<[_; 9]> = Heap::new_from_fn(generator).expect("failed to allocate memory");
    assert_eq!(heap.inner(), b"Testolope", "invalid value on heap");
}
