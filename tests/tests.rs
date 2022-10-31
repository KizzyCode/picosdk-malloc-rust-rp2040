#[cfg(not(feature = "trace"))]
compile_error!("Tests require feature `trace`");

mod _heap;
mod _heapref;

use picosdk_malloc::trace;

/// Runs all tests in sequential order
#[test]
fn all_sequential() {
    // Heap tests
    _heap::new();
    _heap::from_raw();
    _heap::into_inner();
    _heap::into_raw();
    _heap::inner();
    _heap::inner_mut();
    _heap::uninit();
    _heap::assume_init();
    _heap::new_default();
    _heap::new_from_fn();

    // HeapRef tests
    _heapref::heapref_new_from_heap();
    _heapref::heapref_new();
    _heapref::heapref_inner();
    _heapref::heapref_strong();
    _heapref::heapref_weak();
    _heapref::heapref_downgrade();
    _heapref::heapref_try_unwrap_heap();
    _heapref::heapref_try_unwrap();
    _heapref::heapref_clone();
    _heapref::heaprefweak_strong();
    _heapref::heaprefweak_weak();
    _heapref::heaprefweak_upgrade();
    _heapref::heaprefweak_clone();

    // Ensure that we have not leaked memory
    assert_eq!(unsafe { trace::allocated() }, 0, "invalid amount of allocated bytes");
}
