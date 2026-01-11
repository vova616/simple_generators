#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use simple_generators::*;

#[test]
fn test_macro() {
    let sum: u64 = gen_macro(10).sum();
    assert_eq!(sum, 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9);
}

#[test]
fn test_macro_lifetime() {
    let sum: u64 = Foo {
        vec: vec![10, 20, 30],
    }
    .test_gen()
    .sum();
    assert_eq!(sum, 10 + 20 + 30);
}

#[test]
fn test_iter() {
    let sum: u64 = gen_iter(10).sum();
    assert_eq!(sum, 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9);
}

#[generator]
fn gen_macro(n: u64) -> impl Iterator<Item = u64> {
    let mut num = 0;
    while num < n {
        yield num;
        num += 1;
    }
}

fn gen_iter(n: u64) -> impl Iterator<Item = u64> {
    (#[coroutine]
    move || {
        let mut num = 0;
        while num < n {
            yield num;
            num += 1;
        }
    })
    .iter()
}

fn gen_adapter(n: u64) -> impl Iterator<Item = u64> {
    GeneratorIteratorAdapter(
        #[coroutine]
        move || {
            let mut num = 0;
            while num < n {
                yield num;
                num += 1;
            }
        },
    )
}

struct Foo {
    vec: Vec<u64>,
}

impl Foo {
    #[generator]
    fn test_gen<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        for item in &self.vec {
            yield *item;
        }
    }
}
