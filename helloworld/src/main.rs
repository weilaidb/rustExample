// fn testforloop() {
//     let numbers = [1, 2, 3, 4, 5];

//     for num in numbers.iter() {
//         println!("{}", num);
//     }
// }

// fn main() {
//     println!("Hello, world!");
//     println!("Hello, world!");
//     println!("Hello, world!");
//     println!("Hello, world!");

//     testforloop();
// }

// fn main() {
//     let a = 12;
//     println!("a is {}", a);
//     // a = 10;

//     // #[warn(unused_mut)]
//     let b = 123;
//     // b = 456;
//     println!("b is {}", b);

//     // let a = 123;
//     // let a = 456;
// }

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    let (x, y, z) = tup;
    // y 等于 6.4
}
