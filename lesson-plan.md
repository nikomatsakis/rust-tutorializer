- Hello, World Example:
    - Explain:
        - `fn` declaration
        - `println` is a macro
    - Format strings in `println!`
        - `println("Hello, {}!", world);`
        - also show `{:?}`
    - Move "world" into a local variable so we can change it
        - `let name = "fellow Rustaceans"; println("Hello, {}!", name);`
    - Abstract into a helper fn
        - `fn greet(name: String) { println("Hello, {}!", name); }`
        - What goes wrong?
            - Explain `format!`, show how you can use same helpers
            - Explain `push_str` and mutable local variables
                - `let mut name = format!("fellow "); name.push_str("Rustacean");`
    - Call helper fn twice
        - What goes wrong now?
    - Timing notes: ~30 minutes from start to here
- Borrowing Example (~5 min):
    - Show that `helper(&name)` compiles
    - Show that `name.push_str` does not
    - Create `rustify(name: &mut String)` that appends some text
      - Show that I have to modify `name` to be `let mut`
    - Show that I can do `let p = &name; helper(p);`
    - Show that I can do `let q = p; helper(p); helper(q);`
    - Show that mutable references work differently:
      - `{ let m = &mut name; rustify(m); }` is ok
      - `{ let m = &mut name; let n = m; rustify(m); }` is not
      - `{ let m = &mut name; let n = m; rustify(n); }` is OK again
    - Remove braces. Explain that you cannot have a mutable and
      immutable reference in scope at the same time. Explain that, for
      the moment, compiler does not consider that `m` is not used
      after the call to `rustify`, though we are considering changing
      that rule.

       


- Timing
  - Intro: ~5 min
  - Hello world: ~5 min (10)
  - Ownership slides: ~10 min (20)
  - Ownership example: ~10 min (30)
  - 3:58 - 4:03 (basic borrow slides: 5min)
  - Borrowing:
    
