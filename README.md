capture.rs
==============

[![Travis-CI Status](https://travis-ci.org/Kimundi/lazy-static.rs.png?branch=master)](https://travis-ci.org/Kimundi/lazy-static.rs)

A macro for adding explicit capture clauses to a (closure-) expression.

Using this macro, it becomes possible to be explicit about what
variables a closure captures, and by which capture mode it does so.

# Syntax

```ignore
capture!($([move IDENT,]
           [ref IDENT,]
           [IDENT IDENT,])*
         in EXPR)
```

# Semantic

The macro will expand to nested block expressions with a `let` binding per capture clause:

- The `move x` clause rebinds a name by itself, which is effectively a no-op.
- The `ref x` clause rebinds a name by a shared reference to it.
- The `ref mut x` clause rebinds a name by a mutable reference to it.
- The `IDENT x` clause rebinds a name to the result of calling `.IDENT()` method on it.
  This can for example be used to capture by clone, which would look like this: `clone x`.

These bindings will be in scope for the final `EXPR` expression, which will usually be a
by-value capturing closure.

# Current limitations and future changes

- The macro syntax makes it somewhat verbose. A future version of this package
  might make use of expression-attributes instead to become more lightweight.
- The macro might become more specialized to always produce a by-value capturing closure,
  rather than allowing arbitrary expressions.

# Example

Using the macro:

```rust
#![feature(phase)]
#![feature(unboxed_closures)]

#[phase(plugin)]
extern crate capture;

fn main() {
    let (x, y, z) = (1u32, 2u32, 3u32);
    let g = capture!(move x, ref y, clone z in move |:| x + *y + z);

    assert_eq!(g(), 6);
}
```


```
