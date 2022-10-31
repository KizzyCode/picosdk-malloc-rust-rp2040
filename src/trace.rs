//! Traces the amount of heap-allocated memoy via this crate
//!
//! # Safety
//! Because the pico does not support atomics, the trace counter __IS NOT__ multicore-safe.

/// The amount of allocated bytes
#[cfg(feature = "trace")]
static mut ALLOCATED_BYTES: usize = 0;

/// The current amount of heap-allocated bytes
///
/// # Safety
/// Because the pico does not support atomics, this function __IS NOT__ multicore-safe.
#[cfg(feature = "trace")]
pub unsafe fn allocated() -> usize {
    ALLOCATED_BYTES
}

/// Increases the allocated-bytes counter by `bytes`
///
/// # Safety
/// Because the pico does not support atomics, this function __IS NOT__ multicore-safe.
#[allow(unused_variables)]
#[inline(always)]
pub(crate) unsafe fn increment_allocated(bytes: usize) {
    #[cfg(feature = "trace")]
    {
        // Is optimized away if `trace` is disabled
        ALLOCATED_BYTES += bytes;
    }
}

/// Increases the allocated-bytes counter by `bytes`
///
/// # Safety
/// Because the pico does not support atomics, this function __IS NOT__ multicore-safe.
#[allow(unused_variables)]
#[inline(always)]
pub(crate) unsafe fn decrement_allocated(bytes: usize) {
    #[cfg(feature = "trace")]
    {
        // Is optimized away if `trace` is disabled
        ALLOCATED_BYTES -= bytes;
    }
}
