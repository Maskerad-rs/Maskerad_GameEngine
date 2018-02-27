// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use allocators::stacks::{MemStack, DoubleMemStack};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MemStackMap(HashMap<String, MemStack>);

impl Default for MemStackMap {
    fn default() -> Self {
        MemStackMap(HashMap::new())
    }
}

impl MemStackMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_mem_stack<S, M>(&mut self, key: S, stack: M) -> Option<MemStack> where
        S: Into<String>,
        M: Into<MemStack>,
    {
        self.0.insert(key.into(), stack.into())
    }

    pub fn get_stack<S>(&self, key: S) -> Option<&MemStack> where
        S: AsRef<str>
    {
        self.0.get(key.as_ref())
    }

    pub fn get_stack_mut<S>(&mut self, key: S) -> Option<&mut MemStack> where
        S: AsRef<str>
    {
        self.0.get_mut(key.as_ref())
    }
}

#[derive(Debug)]
pub struct DoubleMemStackMap(HashMap<String, DoubleMemStack>);

impl Default for DoubleMemStackMap {
    fn default() -> Self {
        DoubleMemStackMap(HashMap::new())
    }
}

impl DoubleMemStackMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_double_mem_stack<S, M>(&mut self, key: S, double_stack: M) -> Option<DoubleMemStack> where
        S: Into<String>,
        M: Into<DoubleMemStack>,
    {
        self.0.insert(key.into(), double_stack.into())
    }

    pub fn get_double_stack<S>(&self, key: S) -> Option<&DoubleMemStack> where
        S: AsRef<str>
    {
        self.0.get(key.as_ref())
    }

    pub fn get_double_stack_mut<S>(&mut self, key: S) -> Option<&mut DoubleMemStack> where
        S: AsRef<str>
    {
        self.0.get_mut(key.as_ref())
    }
}