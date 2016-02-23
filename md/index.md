## Rust Tutorial

Thank you for working through these tutorials! I hope you like
them. Before you finish, please let me know what you thought at
[this Google form](http://goo.gl/forms/TTjkyPcF6i) -- your feedback
helps me to improve the tutorials for others. Thanks. --nmatsakis

FYI: This page is at <http://home.url>.

- [Hello World](src/hello_world.rs):
    - Goal: make it greet you by name
    - Time: 3 minutes
- [Ownership](src/ownership.rs):
    - Goal #1: Get code to compile
    - Goal #2: Convert so that it prints the original string before removing vowels
      as well as the "devowelized" version.
    - Extra credit: Can you do it without copying any data?
    - Time: 10 minutes
- [Borrowing](src/borrowing.rs):
    - Goal: convert `strcat` function so that it uses borrowing, not ownership.
    - [Hint:](hint-borrowing-1.html) Getting the syntax just right can
      be a bit tricky if you've never done any Rust
      before. [If you need a hint, click here](hint-borrowing-1.html).
    - Time: 10 minutes
- [Structs](src/structs.rs)
    - Goal: implement `total_price` method
    - Time: 10 minutes
- [Threads](src/threads.rs)
    - Goal: join the threads and print out the store with the best price
    - Extra credit #1: use channels instead
    - Extra credit #2: or, instead of channels, use a mutex to compute the best price in
      the parallel threads themselves
    - Time: 10 minutes

