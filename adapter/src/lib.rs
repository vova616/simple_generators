#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::*;

pub struct GeneratorIteratorAdapter<G: Generator + Unpin>(pub G);

impl<G: Generator + Unpin> Iterator for GeneratorIteratorAdapter<G>
{
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match { Pin::new(&mut self.0).resume() } {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}

pub trait GeneratorIterator: Generator<Return = ()> {
    fn iter<T: Sized>(self) -> GeneratorIteratorAdapter<Self>
    where
        Self: Generator<Yield = T, Return = ()> + Sized + Unpin,
    {
        GeneratorIteratorAdapter(self)
    }
}

impl<I: Generator<Return = ()>> GeneratorIterator for I {}
