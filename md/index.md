# RustConf 2017 Tutorial Exercises

FYI: This page is at <http://home.url>.

## Beginner Tutorial

### Ownership

[Slides.](http://home.url/PDF/10-Ownership.pdf)

- [Hello World](src/hello_world.rs):
    - Time: 3 minutes
- [Ownership](src/ownership.rs):
    - Extra credit: Can you do it without copying any data?
    - Time: 10 minutes

### Shared borrows

[Slides.](http://home.url/PDF/20-Sharing.pdf)

- [Shared borrows](src/shared-borrow.rs):
    - Time: 10 minutes

### Mutable borrows

[Slides.](http://home.url/PDF/25-Mutable%20borrows.pdf)

- [Mutable borrows](src/mutable-borrow.rs):
    - [Hint:](hint-mutable-borrow-1.html) Getting the syntax just right can
      be a bit tricky if you've never done any Rust
      before. [If you need a hint, click here](hint-mutable-borrow-1.html).
    - Time: 10 minutes

### Structs and such

[Slides.](http://home.url/PDF/30-Structs-and-such.pdf)

- [Structs](src/structs.rs)
    - [Hint: Here is an outline of what
      the function should do, if you get stuck.](hint-struct-1.html)
    - Time: 10 minutes
    
## Intermediate Tutorial    

### Traits

[Slides.](http://home.url/PDF/40-Traits.pdf)

- [Traits](src/traits.rs)
- [Defaults](src/defaults.rs)
- [Layering](src/layering.rs)

### Intro to threading

[Slides.](http://home.url/PDF/50-Intro-to-threading.pdf)

- [Threads](src/threads.rs)
    - Extra credit #1: use channels instead
    - Extra credit #2: or, instead of channels, use a mutex to compute the best price in
      the parallel threads themselves
    - Time: 10 minutes

## Advanced tutorial

[Slides.](http://home.url/PDF/60-Advanced-Lifetimes.pdf)

- [Named lifetime parameters](src/named_lifetime_parameters.rs)
- [Lifetimes as part of the type](src/lifetimes_as_part_of_type.rs)
- [Successful borrowing](src/successful_borrowing.rs)
- [Lifetimes in structs](src/entry.rs)

