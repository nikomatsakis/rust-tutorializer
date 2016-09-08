### Hint for the "Struct" exercise

The outline of what you want to do is this:

- create a mutable variable `sum` that is initially `0.0`
- for each name `name` in the shopping list:
  - call the `price()` method to get the (optional) price
  - if the price is `Some`, add it to `sum`
  - if the price is `None`, return `None`
- return `Some(sum)`

[Back to the exercise.](src/structs.rs)
