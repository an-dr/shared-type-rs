# shared-type

![GitHub Release](https://img.shields.io/github/v/release/an-dr/shared-type-rs?style=flat-square)


Shared type alias and several traits to simplify working with Arc&lt;Mutex&lt;T>>

- [shared-type](#shared-type)
    - [TL;DR](#tldr)
    - [Description](#description)
    - [How to use](#how-to-use)

## TL;DR

| With the crate                           | Without the crate                                   |
| ---------------------------------------- | --------------------------------------------------- |
| `var.into_shared()`                      | `Arc::new(Mutex::new(var))`                         |
| `let r = var.with_inner(\|v\|{...})`     | `let r = var.lock().ok().map(\|mut v\| {...});`     |
| `let r = var.try_with_inner(\|v\|{...})` | `let r = var.try_lock().ok().map(\|mut v\| {...});` |

## Description

The crate provides a shared type alias `Shared<T>` which is a shorthand for `Arc<Mutex<T>>`. It also provides following traits:

- IntoShared<T> - To convert a value into `Shared<T>` (`Arc<Mutex<T>>`)
- WithSharedInner<T> - Closure to unwrap the value from `Shared<T>`

## How to use

Add the following to your `Cargo.toml`:

```toml
shared-type = "1.0.0"
```
Code!

```rust
let vec = vec![1, 2, 3];
let vec_shared = vec.into_shared(); // Arc<Mutex<Vec<i32>>>

// The simple example of using the shared value that will wait for
// the lock to become available and then call the closure
vec_shared.with_inner(|vec| {
    vec.push(4);
});

// The same example but with the return value
let new_len = vec_shared.with_inner(|vec| {
    vec.push(5);
    vec.len()
});

// Or we can use the `try_with` to non-blocking access
let newer_len = vec_shared.try_with_inner(|vec| {
    vec.push(6);
    vec.len()
});
```

Find the full example in the [examples](examples) directory.
