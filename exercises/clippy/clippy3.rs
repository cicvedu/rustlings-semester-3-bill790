// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#![allow(unused_variables, unused_assignments)]
//#[allow(clippy::needless_pass_by_value)]
use std::mem;

fn main() {
    let my_option: Option<()> = None;
    match my_option {
        Some(_) => println!("my_option has a value"),
        None => println!("my_option has no value"),
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

   // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 0);
   let mut my_vec = vec![1, 2, 3, 4, 5];
   my_vec.clear();
   let my_empty_vec = my_vec;
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
