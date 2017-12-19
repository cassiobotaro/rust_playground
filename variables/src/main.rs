fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

// error[E0384]: re-assignment of immutable variable `x`
// fn main() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }
//
// ok
//fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
// }
//
// shadowing
//
// fn main() {
    // let x = 5;

    // let x = x + 1;

    // let x = x * 2;

    // println!("The value of x is: {}", x);
// }

//error[E0308]: mismatched types
// let spaces = "   ";
// let spaces = spaces.len();
