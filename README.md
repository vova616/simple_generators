# simple_generators

A library that contains a macro for a simpler generator->iterator creation

## Usage

Use latest nightly
Didn't yet publish to crates.io so

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple_generators = {version="0.1", git = "https://github.com/vova616/simple_generators"}
```

example:

```rust
#![feature(generators, generator_trait)]

use simple_generators::*;

fn main() {
    println!("{}", test_macro(10).sum::<u64>());

    let foo = Foo {
        vec: vec![10, 20, 30],
    };

    for e in foo.test_macro() {
        println!("{}", e);
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

struct Foo {
    vec: Vec<u64>,
}

impl Foo {
    #[generator]
    fn test_macro<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        for item in &self.vec {
            yield *item;
        }
    }
}
```
