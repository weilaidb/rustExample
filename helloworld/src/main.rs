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

// fn move_coords(x:(i32, i32)) -> (i32, i32)
// {
//     (x.0 + 1, x.1+1)
// }

// fn main() {
//     let tuple: (&'static str, i32, char) = ("hello", 5, 'c');
//     assert_eq!(tuple.0, "hello");
//     assert_eq!(tuple.1, 5);
//     assert_eq!(tuple.2, 'c');

//     let coords = (0,1);
//     let result = move_coords(coords);
//     assert_eq!(result, (1,2));
//     let (x, y) = move_coords(coords);
//     assert_eq!(x, 1);
//     assert_eq!(y, 2);

// }

// #[derive(Debug, PartialEq)]
// struct People {
//     name: &'static str,
//     gender: u32,
// }

// impl People {
//     fn new(name: &'static str, gender: u32) -> Self {
//         return People {
//             name: name,
//             gender: gender,
//         };
//     }
//     fn name(&self) {
//         println!("name:{:?}", self.name);
//     }

//     fn set_name(&mut self, name: &'static str) {
//         self.name = name;
//     }

//     fn gender(&self) {
//         let gender = if (self.gender == 1) { "boy" } else { "girl" };
//         println!("gender:{:?}", gender);
//     }
// }

// fn main() {
//     let alex = People::new("Alex", 1);
//     alex.name();
//     alex.gender();
//     assert_eq!(
//         alex,
//         People {
//             name: "Alex",
//             gender: 1
//         }
//     );
//     let mut alice = People::new("Alice", 0);
//     alice.name();
//     alice.gender();
//     assert_eq!(
//         alice,
//         People {
//             name: "Alice",
//             gender: 0
//         }
//     );
//     alice.set_name("Rose");
//     alice.name();
//     assert_eq!(
//         alice,
//         People {
//             name: "Rose",
//             gender: 0
//         }
//     );
// }

// struct Color(i32, i32, i32);
// fn main() {
//     let color = Color(0, 1, 2);
//     assert_eq!(color.0, 0);
//     assert_eq!(color.1, 1);
//     assert_eq!(color.2, 2);
// }

// struct Integer(u32);
// type Int = i32;
// fn main() {
//     let int = Integer(10);
//     assert_eq!(int.0, 10);

//     let int:Int = 10;
//     assert_eq!(int, 10);
// }

// struct Empty;
// fn main() {
//     let x = Empty;
//     println!("{:p}", &x);

//     let y = x;
//     println!("{:p}", &y);

//     let z = Empty;
//     println!("{:p}", &z);

//     assert_eq!((..), std::ops::RangeFull);
// }

// enum Number {
//     Zero,
//     One,
//     Two,
// }

// fn main()
// {
//     let a = Number::One;
//     match a {
//         Number::Zero => println!("0"),
//         Number::One => println!("1"),
//         Number::Two => println!("2"),
//     }
// }

// enum Color {
//     Red = 0xff0000,
//     Green = 0x00ff00,
//     Blue = 0x0000ff,
// }

// fn main() {
//     println!("roses are #{:06x}", Color::Red as i32);
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
//     let y: fn(String) -> IpAddr = IpAddr::V6;
//     let home = IpAddr::V4(127, 0, 0, 1);
// }

// enum Option {
//     Some(i32),
//     None,
// }

// fn main() {
//     let s = Some(42);
//     let num = s.unwrap();
//     match s {
//         Some(n) => println!("num is :{}", n),
//         None => (),
//     }
// }

// fn main()
// {
//     let s:&Option<String> = &Some("Hello".to_string());
//     match s {
//         Some(ref s) => println!("s is :{}", s),
//         _ => (),
//     }
// }

// fn main()
// {
//     let mut v1 = vec![];
//     v1.push(1);
//     v1.push(2);
//     v1.push(3);
//     assert_eq!(v1, [1,2,3]);
//     assert_eq!(v1[0], 1);
//     assert_eq!(v1[1], 2);
//     assert_eq!(v1[2], 3);

//     let mut v2 = vec![0;10];
//     let mut v3 = Vec::new();
//     v3.push(4);
//     v3.push(5);
//     v3.push(6);
//     // v3[4];

// }

// use std::collections::VecDeque;
// fn main()
// {
//     let mut buf: VecDeque<i32> = VecDeque::new();
//     buf.push_front(1);
//     buf.push_front(2);
//     assert_eq!(buf.get(0), Some(&2));
//     assert_eq!(buf.get(1), Some(&1));
//     buf.push_back(3);
//     buf.push_back(4);
//     buf.push_back(5);
//     assert_eq!(buf.get(2), Some(&3));
//     assert_eq!(buf.get(3), Some(&4));
//     assert_eq!(buf.get(4), Some(&5));
// }

// use std::collections::LinkedList;
// fn main() {
//     let mut list1 = LinkedList::new();
//     list1.push_back('a');
//     let mut list2 = LinkedList::new();
//     list2.push_back('b');
//     list2.push_back('c');
//     list1.append(&mut list2);
//     println!("{:?}", list1);
//     println!("{:?}", list2);
//     list1.pop_front();
//     println!("{:?}", list1);
//     list1.push_front('e');
//     println!("{:?}", list1);
//     list2.push_front('f');
//     list2.push_front('h');
//     list2.push_front('h');
//     println!("{:?}", list2);

// }

// use std::collections::BTreeMap;
// use std::collections::HashMap;
// fn main() {
//     let mut hmap = HashMap::new();
//     let mut bmap = BTreeMap::new();
//     hmap.insert(3, "c");
//     hmap.insert(1, "a");
//     hmap.insert(2, "b");
//     hmap.insert(5, "e");
//     hmap.insert(4, "f");
//     bmap.insert(3, "c");
//     bmap.insert(2, "b");
//     bmap.insert(1, "a");
//     bmap.insert(5, "e");
//     bmap.insert(4, "d");
//     println!("{:?}", hmap);
//     println!("{:?}", bmap);
// }

// use std::collections::BTreeSet;
// use std::collections::HashSet;

// fn main() {
//     let mut hbooks = HashSet::new();
//     let mut bbooks = BTreeSet::new();
//     hbooks.insert("A Song of Ice and Fire");
//     hbooks.insert("The Emerald City");
//     hbooks.insert("The Odyssey");
//     if !hbooks.contains("The Emerald City") {
//         println!(
//             "We have {} books, but The Emerald City ain't one.",
//             hbooks.len()
//         );
//     }
//     println!("{:?}", hbooks);
//     bbooks.insert("A Song of Ice and Fire");
//     bbooks.insert("The Emerald City");
//     bbooks.insert("The Odyssey");
//     println!("{:?}", bbooks);
// }

// use std::collections::BinaryHeap;
// fn main() {
//     let mut heap = BinaryHeap::new();
//     assert_eq!(heap.peek(), None);
//     let arr = [93, 94, 99, 80, 89, 23, 1, 2, 3, 4, 5, 7, 76];
//     for &i in arr.iter() {
//         heap.push(i);
//     }
//     assert_eq!(heap.peek(), Some(&99));
//     println!("{:?}", heap);
// }

// fn main() {
//     #[derive(PartialEq)]
//     #[derive(Debug)]
//     struct Point {
//         x: f64,
//         y: f64,
//     }
//     let box_point = Box::new(Point { x: 0.0, y: 0.0 });
//     let unboxed_point = *box_point;
//     assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 });
// }

// std::option::Option;
// enum Option<T> {
//     Some(T),
//     None,
// }

// use std::fmt::Debug;
// fn match_option<T: Debug>(o: Option<T>) {
//     match o {
//         Some(i) => println!("{:?}", i),
//         None => println!("nothing"),
//     }
// }
// fn main() {
//     let a:Option<i32> = Some(3);
//     let b:Option<&str> = Some("hello");
//     let c:Option<char> = Some('A');
//     let d:Option<u32> = None;

//     match_option(a);
//     match_option(b);
//     match_option(c);
//     match_option(d);
// }

// struct Duck;
// struct Pig;
// trait Fly {
//     fn fly(&self) -> bool;
// }
// impl Fly for Duck {
//     fn fly(&self) -> bool {
//         return true;
//     }
// }
// impl Fly for Pig {
//     fn fly(&self) -> bool {
//         return false;
//     }
// }
// fn fly_static<T: Fly>(s: T) -> bool {
//     s.fly()
// }
// /// .
// fn fly_dyn(s: &dyn Fly) -> bool {
//     s.fly()
// }
// fn main() {
//     let pig = Pig;
//     assert_eq!(fly_static::<Pig>(pig), false);
//     let duck = Duck;
//     assert_eq!(fly_static::<Duck>(duck), true);
//     assert_eq!(fly_dyn(&Duck), true);
// }

use std::fmt::*;

struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 10, y: 0 };
    println!("The origin is :{:?}", origin);
}
