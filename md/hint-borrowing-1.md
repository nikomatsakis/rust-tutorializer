### Hint for the "Borrowing" exercise

You want to change the signature of `join_words` as follows:

```rust
fn strcat(prefix: &mut String, suffix: &String) {
    ...
}
```

Now `prefix` is a mutable reference to some `String` on the caller's
side. We need a mutable reference so we can push onto the string.

`suffix` is a shared reference. A shared reference suffices because we
only need to read from `suffix`.

Note that the return value has also changed: since we are going to be
mutating `prefix` in place, we no longer need to return anything.
