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

// fn main() {
//     // let x = 5;
//     // let x = x + 1;
//     // let x = x * 2;
//     // println!("The value of x is: {}", x);

//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     // tup.0 等于 500
//     // tup.1 等于 6.4
//     // tup.2 等于 1
//     let (x, y, z) = tup;
//     // y 等于 6.4
//     println!("x is {}", x);
//     println!("y is {}", y);
//     println!("z is {}", z);
// }

// /// Adds one to the number given.
// ///
// /// # Examples
// ///
// /// ```
// /// let x = add(1, 2);
// ///
// /// ```

// fn add(a: i32, b: i32) -> i32 {
//     return a + b;
// }

// fn main() {
//     println!("{}", add(2, 3));
// }

// fn main() {
//     println!("Hello, world!");
//     another_function();
// }

// fn another_function() {
//     println!("Hello, runoob!");
// }

// fn main() {
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("x 的值为 : {}", x);
//     println!("y 的值为 : {}", y);
// }

fn main() {
    assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    assert_eq!(3 + 4 + 5, (3..6).sum());
    assert_eq!(3 + 4 + 5 + 6, (3..=6).sum());
    for i in 1..5 {
        println!("{}", i);
    }
    for i in 1..=5 {
        println!("{}", i);
    }
}
