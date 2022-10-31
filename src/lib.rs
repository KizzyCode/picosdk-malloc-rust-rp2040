#![no_std]
#![doc = include_str!("../README.md")]

pub mod heap;
pub mod heapref;
pub mod trace;

pub use crate::{
    heap::Heap,
    heapref::{HeapRef, HeapRefWeak},
};
