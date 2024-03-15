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

// fn main() {
//     assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
//     assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
//     assert_eq!(3 + 4 + 5, (3..6).sum());
//     assert_eq!(3 + 4 + 5 + 6, (3..=6).sum());
//     for i in 1..5 {
//         println!("{}", i);
//     }
//     for i in 1..=5 {
//         println!("{}", i);
//     }
// }

// fn main()
// {
//     let arr = [1,2,3,4,5];
//     assert_eq!(&arr, &[1,2,3,4,5]);
//     assert_eq!(&arr[1..], &[2,3,4,5]);
//     assert_eq!(&arr.len(), &5);
//     assert_eq!(&arr.is_empty(), &false);

//     let arrx = &mut [1,2,3];
//     arrx[1] = 7;
//     assert_eq!(arrx,&[1,7,3]);

//     let vec = vec![1,2,3];
//     assert_eq!(&vec[..],[1,2,3]);
// }

// fn main() {
// // 定义静态字符串truth
//     let truth: &'static str = "Rust是一门优雅的语言";

// // 获取指向字符串的指针
//     let ptr = truth.as_ptr();

// // 获取字符串的长度
//     let len = truth.len();

// // 断言字符串的长度为28
//     assert_eq!(28, len);

// // 使用unsafe块创建字符串切片，并将其转换为UTF-8编码的字符串
//     let s = unsafe {
//         let slice = std::slice::from_raw_parts(ptr, len);
//         std::str::from_utf8(slice).unwrap_or_else(|_| panic!("Invalid UTF-8 sequence"));
//     };

// // 断言转换后的字符串与原字符串相等
//     // assert_eq!(Ok(s), Ok(truth));
// }

fn move_coords(x:(i32, i32)) -> (i32, i32)
{
    (x.0 + 1, x.1+1)
}

fn main() {
    let tuple: (&'static str, i32, char) = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    let coords = (0,1);
    let result = move_coords(coords);
    assert_eq!(result, (1,2));
    let (x, y) = move_coords(coords);
    assert_eq!(x, 1);
    assert_eq!(y, 2);


}
