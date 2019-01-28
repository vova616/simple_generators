#![feature(generators, generator_trait)]

use simple_generators::*;

#[test]
fn test_macros() {
    let sum: u64 = test_macro(10).sum();

    for var in test_macro(5) {
        println!("{}", var);
    }
}

#[generator]
fn test_macro(n: u64) -> impl Iterator<Item = u64> {
    let mut num = 0;
    while num < n {
        yield num;
        num += 1;
    }
}

fn test_iter(n: u64) -> impl Iterator<Item = u64> {
    (move || {
        let mut num = 0;
        while num < n {
            yield num;
            num += 1;
        }
    })
    .iter()
}

fn test_adapter(n: u64) -> impl Iterator<Item = u64> {
    GeneratorIteratorAdapter(move || {
        let mut num = 0;
        while num < n {
            yield num;
            num += 1;
        }
    })
}
