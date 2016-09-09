## Rust Tutorial

Thank you for working through these tutorials! I hope you like
them. Before you finish, please let me know what you thought at
[this Google form](http://goo.gl/forms/TTjkyPcF6i) -- your feedback
helps me to improve the tutorials for others. Thanks. --nmatsakis

FYI: This page is at <http://home.url>.

### Beginner tutorial: Ownership and Borrowing

- [Hello World](src/hello_world.rs):
    - Time: 3 minutes
- [Ownership](src/ownership.rs):
    - Extra credit: Can you do it without copying any data?
    - Time: 10 minutes
- [Shared borrows](src/shared-borrow.rs):
    - Time: 10 minutes
- [Mutable borrows](src/mutable-borrow.rs):
    - [Hint:](hint-mutable-borrow-1.html) Getting the syntax just right can
      be a bit tricky if you've never done any Rust
      before. [If you need a hint, click here](hint-mutable-borrow-1.html).
    - Time: 10 minutes
- [Structs](src/structs.rs)
    - [Hint: Here is an outline of what
      the function should do, if you get stuck.](hint-struct-1.html)
    - Time: 10 minutes

### Intermediate tutorial: Traits and Threads

- [Traits](src/traits.rs)
- [Defaults](src/defaults.rs)
- [Layering](src/layering.rs)
- [Threads](src/threads.rs)
    - Extra credit #1: use channels instead
    - Extra credit #2: or, instead of channels, use a mutex to compute the best price in
      the parallel threads themselves
    - Time: 10 minutes

### Advanced tutorial: More lifetimes

- Ensure you are using **Nightly builds** for the best error messages.
- Exercises are listed in the source code.

- [Named lifetime parameters](src/named_lifetime_parameters.rs)
- [Lifetimes as part of the type](src/lifetimes_as_part_of_type.rs)
- [Successful borrowing](src/successful_borrowing.rs)
- [Lifetimes in structs](src/entry.rs)

