#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};

pub struct GeneratorIteratorAdapter<G>(pub G);

impl<G> Iterator for GeneratorIteratorAdapter<G>
where
    G: Generator<Return = ()>,
{
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { self.0.resume() } {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}

pub trait GeneratorIterator: Generator<Return = ()> {
    fn iter<T: Sized>(self) -> GeneratorIteratorAdapter<Self>
    where
        Self: Generator<Yield = T, Return = ()> + Sized,
    {
        GeneratorIteratorAdapter(self)
    }
}

impl<I: Generator<Return = ()>> GeneratorIterator for I {}
