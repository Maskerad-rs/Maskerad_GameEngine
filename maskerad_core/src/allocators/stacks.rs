// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use maskerad_memory_allocators::StackAllocator;
use maskerad_memory_allocators::DoubleBufferedAllocator;
use allocators::errors::{AllocationError, AllocationResult};

#[derive(Debug)]
pub struct MemStack(StackAllocator);

impl MemStack {
    pub fn with_capacity(capacity: usize, capacity_copy: usize) -> Self {
        MemStack(StackAllocator::with_capacity(capacity, capacity_copy))
    }

    pub fn allocate_mutable<T, F>(&self, op: F) -> AllocationResult<&mut T> where
        F: FnOnce() -> T
    {
        self.0.alloc_mut(op).map_err(|maskerad_mem_error| {
            AllocationError::from(maskerad_mem_error)
        })
    }

    pub fn allocate_mutable_unchecked<T, F>(&self, op: F) -> &mut T where
        F: FnOnce() -> T
    {
        self.0.alloc_mut_unchecked(op)
    }

    pub fn allocate<T, F>(&self, op: F) -> AllocationResult<&T> where
        F: FnOnce() -> T
    {
        self.0.alloc(op).map_err(|maskerad_mem_error| {
            AllocationError::from(maskerad_mem_error)
        })
    }

    pub fn allocate_unchecked<T, F>(&self, op: F) -> &T where
        F: FnOnce() -> T
    {
        self.0.alloc_unchecked(op)
    }

    pub fn marker(&self) -> usize {
        self.0.marker()
    }

    pub fn marker_copy(&self) -> usize {
        self.0.marker_copy()
    }

    pub fn reset(&self) {
        self.0.reset()
    }

    pub fn reset_copy(&self) {
        self.0.reset_copy()
    }

    pub fn reset_to_marker(&self, marker: usize) {
        self.0.reset_to_marker(marker)
    }

    pub fn reset_to_marker_copy(&self, marker: usize) {
        self.0.reset_to_marker_copy(marker)
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn capacity_copy(&self) -> usize {
        self.0.capacity_copy()
    }
}

#[derive(Debug)]
pub struct DoubleMemStack(DoubleBufferedAllocator);

impl DoubleMemStack {
    pub fn with_capacity(capacity: usize, capacity_copy: usize) -> Self {
        DoubleMemStack(DoubleBufferedAllocator::with_capacity(capacity, capacity_copy))
    }

    pub fn allocate_mutable<T, F>(&self, op: F) -> AllocationResult<&mut T> where
        F: FnOnce() -> T
    {
        self.0.alloc_mut(op).map_err(|maskerad_mem_error| {
            AllocationError::from(maskerad_mem_error)
        })
    }

    pub fn allocate_mutable_unchecked<T, F>(&self, op: F) -> &mut T where
        F: FnOnce() -> T
    {
        self.0.alloc_mut_unchecked(op)
    }

    pub fn allocate<T, F>(&self, op: F) -> AllocationResult<&T> where
        F: FnOnce() -> T
    {
        self.0.alloc(op).map_err(|maskerad_mem_error| {
            AllocationError::from(maskerad_mem_error)
        })
    }

    pub fn allocate_unchecked<T, F>(&self, op: F) -> &T where
        F: FnOnce() -> T
    {
        self.0.alloc_unchecked(op)
    }

    pub fn reset(&self) {
        self.0.reset()
    }

    pub fn reset_copy(&self) {
        self.0.reset_copy()
    }

    pub fn reset_to_marker(&self, marker: usize) {
        self.0.reset_to_marker(marker)
    }

    pub fn reset_to_marker_copy(&self, marker: usize) {
        self.0.reset_to_marker_copy(marker)
    }

    pub fn marker(&self) -> usize {
        self.0.marker()
    }

    pub fn marker_copy(&self) -> usize {
        self.0.marker_copy()
    }

    pub fn swap_buffers(&mut self) {
        self.0.swap_buffers()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn capacity_copy(&self) -> usize {
        self.0.capacity_copy()
    }
}