use std::cell::RefCell;

// Exercise #1. Make the `basic_ref_cell` test pass.
//
// Exercise #2. Modify the body of `fill_vec` (not its signature)
// so as to make the `ref_cell_ref` test pass.

#[test]
fn basic_ref_cell() {
    let data = RefCell::new(vec![]);

    let mut p = data.borrow_mut();
    p.push("Hello,");
    p.push("World!");

    // START SOLUTION
    ::std::mem::drop(p);
    // END SOLUTION

    assert_eq!(&data.borrow()[..], &["Hello,", "World!"]);
}

fn push_twice<F, G>(f: F, g: G)
    where F: FnOnce(&'static str),
          G: FnOnce(&'static str)
{
    f("Hello,");
    g("World!");
}

fn fill_vec(vec: &mut Vec<&'static str>) {
    // This won't compile, but this captures what we *want*
    // to do. Invoke `push_twice` and, for each callback,
    // push the resulting data onto `vec`:
    //
    // push_twice(|x| vec.push(x),
    //            |y| vec.push(y));

    // PROMPT unimplemented!()
    // START SOLUTION
    let cell = RefCell::new(vec);
    push_twice(|x| cell.borrow_mut().push(x),
               |y| cell.borrow_mut().push(y));
    // END SOLUTION
}

#[test]
fn ref_cell_ref() {
    let mut data = vec![];
    fill_vec(&mut data);
    assert_eq!(&data[..], &["Hello,", "World!"]);
}
