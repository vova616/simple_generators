#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::ops::{Coroutine, CoroutineState};
use std::pin::*;

pub struct GeneratorIteratorAdapter<G: Coroutine + Unpin>(pub G);

impl<G: Coroutine + Unpin> Iterator for GeneratorIteratorAdapter<G> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match { Pin::new(&mut self.0).resume(()) } {
            CoroutineState::Yielded(x) => Some(x),
            CoroutineState::Complete(_) => None,
        }
    }
}

pub trait GeneratorIterator: Coroutine<Return = ()> {
    fn iter<T: Sized>(self) -> GeneratorIteratorAdapter<Self>
    where
        Self: Coroutine<Yield = T, Return = ()> + Sized + Unpin,
    {
        GeneratorIteratorAdapter(self)
    }
}

impl<I: Coroutine<Return = ()>> GeneratorIterator for I {}
