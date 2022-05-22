# Cast and Overflow utilities

This utility lib helps for type casting and integer operation overflow checking. The following code block shows examples:

```rust
let a: u64 = 10;
let b: i64 = a.numeric_cast();
```

```rust
let a = 1.overflow_add(1);
```

For the first example, `as` conversion is not perfect for slicently lossy conversion while `try_from` and `try_into` are better. However they're too verbose in most cases, so we wrap it in the `cast` method and make it panic while these `try_xxx` methods failed.

For the second example, rust std lib provides overflow checking methods such as `overflowing_add`. The methods provided in this lib are one step futher, panicing when any overflow happens.
