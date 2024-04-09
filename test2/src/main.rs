use std::io::stdin;
use std::time::SystemTime;

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}


fn readline_sum()
{
    let mut str_buf = String::new();
    println!("please input sum's express, eg 1 2 3");
    stdin().read_line(&mut str_buf).unwrap();
    let sp:Vec<&str> = str_buf.as_str().split(" ").collect();
    let a = sp[0].trim().parse::<i32>().unwrap();
    let b = sp[1].trim().parse::<i32>().unwrap();
    println!("{} + {} = {}", a, b, a+b);
}

fn test1() {
    println!("Hello, world!");
    println!("what's your name ?");
    for i in 0..100 {
        println!("{}", i);
    }
    let a = 12;
    let b = 12 * 13;
    println!("a {},",a);
    println!("a {0}, a again{0},",a);
    println!("a {0}, a again{0},b is {1}",a, b);
    println!("{{}}");


    let rect1 = Rectangle{width:30, height:50};
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let mut str_buf = String::new();
    println!("please input one line:");
    stdin().read_line(&mut str_buf).unwrap();
    println!("Your input line is {}", str_buf);


    readline_sum();

}

fn test3()
{
    let args = std::env::args();
    println!("args is :\n{:?}", args);
    println!("args is :\n{:#?}", args);
    for arg in args{
        println!("{}", arg);
    }
}

fn test4(){
    let mut args = std::env::args();
    args.next();
    let a = args.next().unwrap().trim().parse::<i32>().unwrap();
    let b = args.next().unwrap().trim().parse::<i32>().unwrap();
    println!("{} + {} = {}", a, b, a+ b);
}


fn test5(){
    const A_CONSTANT:i32 = 123;
    static VAR:i32 = 33;
    println!("VAR:{}", VAR);
}

fn test6(){
    static mut VAR:i32 = 123;
    unsafe {
        VAR = 456;
        println!("VAR {}", VAR);
    }
}

fn test7(){
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7/32.3;
    let remainder = 43 % 5;

}

fn test8(){
    let x:f64 = 2.0;
    println!("{}", x.sin());
    println!("{}", x.cos());
    println!("{}", x.tan());
    println!("{}", x.sqrt());
    println!("{}", x.powi(4));
    println!("{}", x.ln());
}

fn test9(){
    let a = true;
    let b:bool = false;
    println!("{} {}", a, b);
}

fn test10(){
    println!("{}", 3 > 4);
    println!("{}", 3 >= 4);
    println!("{}", 3 < 4);
    println!("{}", 3 == 4);
    println!("{}", 3 != 4);
    println!("{}", !true);
    println!("{}", true && false);
    println!("{}", true || false);
    println!("{}", true ^ true);

}

fn test11(){
    let a = 0b_1010;
    let  b= 0b_0101;
    println!("{}", !a);
    println!("{}", a &b);
    println!("{}", a | b);
    println!("{}", a ^b);
}

fn test12(){
    let en:char = 'R';
    let zh:char = '中';
    println!("{}\n{}", en, zh);
}

fn test13(){
    let strig = String::from("Some thing");
}

fn test14(){
    let mut string = String::from("");
    string.push('A');
    string.push_str("QWERYI:::");
    println!("{}", string);
    println!("len {}", String::from("Hello你好").len());
}

fn test15(){
    let a = String::from("Hello你好");
    let b = String::from("Hello你好");
    let result = a.eq(&b);
    println!("result {}", result);
}

fn test16(){
    let a = String::from("Hello你好").eq("Hello你好");
    let b = String::from("Hello你好").eq(String::from("Hello你好").as_str());
    println!("a {}", a);
    println!("b {}", b);
}

fn test17(){
    let s:String = String::from("RUNOOB");
    let ch:char = s.chars().nth(2).unwrap();
    let sub:&str = &s[0..3];
    println!("s {}",s);
    println!("ch {}", ch);
    println!("sub {}", sub);
}

fn test18(){
    let s:&str = "RUNOOB";
    println!("{} {} {} {}",
        s.len(),
        s.eq("RUNOOB"),
        s.chars().nth(2).unwrap(),
        &s[0..3]);
}

fn test19(){
    let tup = (500,6.4,1);
    let tup:(i32,f64,i32) = (500,6.4,1);
}

fn test20(){
    let tup = (500,6.4, 1);
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);

    let (x,y,z) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
}

fn test21(){
    let a = [1,2,3,4,5,];
    let b = ["January", "February", "March"];
    let c:[i32;5] = [1,2,3,4,5];
    let d = [3;5];
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
}

fn test22(){
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    println!("first:{}", first);
    println!("second:{}", second);
}
//
// fn test23(){
//     let a = [1,2,3,4,5];
//     a[0] = 0;
// }

fn test24(){
    let mut a = [1,2,3,4,5];
    a[0] = 0;
    println!("{:?}", a);
}

fn test25(){
    let array = [1,2,3,4,5];
    let length = array.len();
    println!("array:{:?}", array);
    println!("array len:{}", length);
}

fn addition(a:i32, b:i32) ->i32{
    return a + b;
}

fn addition2(a:i32, b:i32) -> i32{
    a + b
}

fn test26(){
    let sum = addition(100,23);
    println!("{}", sum);
    let sum = addition2(100,23);
    println!("{}", sum);
}

fn test27(){
    let x =4;
    let y = {
        let a = x * x *x;
        let b = 2 * x * x;
        a + b + 3
    };
    println!("y = {}", y);
}

fn function_one(){
    println!("Function one is called.")
}

fn function_two(){
    println!("Function two is called.")
}

fn test28(){
    let mut fun:fn();
    fun = function_one;
    fun();

    fun = function_two;
    fun();
}

fn test29(){
    let number = 3;
    if number < 5 {
        println!("{} < 5  true", number);
    }else{
        println!("{} < 5  false", number);
    }
}

fn test30(){
    let score = 100;
    if score > 90 {
        println!("优");
    }else if score > 60 {
        println!("及格");
    }else { println!("不及格"); }
}


fn test31(){
    let a = 3;
    let number = if a > 0 {1} else {-1};
    println!("number = {}",number);
}

fn test32(){
    let score = 85;
    let branch = if score > 90 {
        "优"
    }else if score > 80 {
        "良"
    }else if score > 70{
        "中等"
    }else { "差" };

    println!("{}", branch);
}

fn test33(){
    let op = 1;

    match op{
        0 => {
            println!("op = 0")
        },
        1|2|3|4|5 =>{
          println!("op = 1 or 2 or 3 or 4 or 5")
        },
        _=>{
            println!("op = Else number")
        }
    }


}

fn test34(){
    let mut number = 1;

    while number < 4 {
        println!("{}", number);
        number += 1;
    }

    println!("EXIT")
}


fn test35(){
    let mut i = 0;
    while  i < 10 {
        println!("{}", i);
        i+=1;
    }
}

fn test36(){
    for i in 0..5{
        println!("{}", i)
    }
}

fn test37(){
    let a  = [10,20,30,40,50];
    for i in a.iter(){
        println!("{}", i);
    }
}

fn test38(){
    let a = [10,20,30,40,50];
    for i in 0..a.len(){
        println!("a[{}] = {}", i, a[i]);
    }
}

fn test39(){
    let s = ['R','U','N','O','O','B'];
    let mut i =0;
    loop {
        let ch = s[i];
        if ch == 'B'{
            break;
        }
        print!("{}",ch);
        i+=1;
    }
    println!();
}

fn test40(){
    let s = ['R','O','O','U','N','B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch =='O'{
            break i;
        }
        i +=1;
    };

    println!("\'O\'的下标为{}", location);
}

fn test41(){
    // let x = String::from("Some data");
    // let y = x;
    // println!("{}", x);
}

fn test42(){
    let x = 1000000;
    let y = x;
    println!("{}", x);
}

fn test43(){
    let s1 = String::from("Hello");
    let s2 = &s1;
    println!("s1 is \"{}\", s2 is \"{}\"", s1, s2);
}

// fn dangele() -> &String{
//     let s = String::from("hello");
//     &s
// }
//
// fn test44(){
//     let reference_to_nothing = dangle();
// }

fn test45(){
    let s = String::from("hello");
     take_ownership(s);
     // take_ownership(s);

    let x =5;

    makes_copy(x);
    makes_copy(x);
}

fn take_ownership(some_string:String){
    println!("{}", some_string);
}

fn makes_copy(some_integer:i32){
    println!("{}", some_integer);
}

fn test46(){
    let s = String::from("hello");
    reference(&s);
    println!("来自主函数：{}",s);
}

fn reference(some_thing:&String){
    println!("来自子函数:{}", some_thing);
}


fn test47(){
    let s1 = gives_ownership();
    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    return some_string;
}

fn takes_and_gives_back(a_string:String)->String{
    return  a_string;
}

fn test48(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s:&String) -> usize{
    s.len()
}

//error
// fn test49(){
//     let mut s1 = String::from("hello;");
//     add_suffix(&s1);
//     println!("{}", s1);
// }

//error
// fn add_suffix(s:&String){
//     s.push_str("SUFFIX");
// }


fn test49(){
    let mut s1 = String::from("hello;");
    add_suffix(&mut s1);
    println!("{}", s1);
}
fn add_suffix(s:&mut String){
    s.push_str("SUFFIX");
}

fn swap(a:&mut i32, b:&mut i32){
    let t = *a;
    *a = *b;
    *b = t;
}

fn test50(){
    let mut a = 0;
    let mut b = 1;
    swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);
}


fn test51(){
    let s:String = String::from("broadcast");

    let part1:&str = &s[0..5];
    let part2:&str = &s[5..9];
    println!("{} = {} + {}", s, part1, part2);
}

fn test52(){
    let string = String::from("RUNOOB");
    let slice = string.as_str();
}

fn test53(){
    let string = "0123456789";
    let s1 = &string[1..4];
    let s2:&str = &string[5..];
    let s3:&str = &string[..4];
    let s4:&str = &string[..];

    println!("s1:{}", s1);
    println!("s2:{}", s2);
    println!("s3:{}", s3);
    println!("s4:{}", s4);
}

fn test54(){
    let arr:[i32;5] = [0,1,2,3,4];
    let part:&[i32] = &arr[1..4];
    for i in part.iter(){
        println!("{}", i);
    }
}

struct Site{
    domain:String,
    name:String,
    nation:String
}

fn test55(){
    let runoob = Site{
        domain:String::from("www.runoob.com"),
        name:String::from("菜鸟教程"),
        nation:String::from("中国")
    };

    let bing = Site{
        domain:String::from("cn.bing.com"),
        name:String::from("必应"),
        ..runoob
    };

}

fn test56(){
    let domain = String::from("www.runoob.com");
    let site = Site{
        domain,
        name:String::from("RUNOOB"),
        nation:String::from("China")
    };
    // println!("{}", domain); //have removed
}

struct Rectangle2{
    width:u32,
    height:u32,
}

impl Rectangle2{
    fn area(&self) ->u32{
        self.width * self.height
    }

    fn wider(&self, rect:&Rectangle2) ->bool{
        self.width > rect.width
    }
}

fn test57(){
    let rect1 = Rectangle2{width:30,height:50};
    println!{"rect1's area is {}", rect1.area()};
}

fn test58(){
    let rect1 = Rectangle2{width:30,height:50};
    let rect2 = Rectangle2{width:40,height:20};
    println!{"rect1's area is {}", rect1.area()};
    println!{"rect2's area is {}", rect2.area()};
    println!{"{}", rect1.wider(&rect2)};
}


fn test59(){
    struct Color(u8,u8,u8);
    struct Point(f64,f64);

    let black = Color(2,7,110);
    let origin = Point(3.0,5.0);

    println!("black = ({},{},{})",black.0,black.1,black.2);
    println!("origin = ({},{})",origin.0,origin.1);

}
fn test60(){
    struct UnitStruct;
}


fn test61(){

    #[derive(Debug)]
    enum Color{
        Red,
        Green,
        Blue
    }
let color = Color::Red;
println!("{:?}", color);

}

fn test62(){
    #[derive(Debug)]
    enum Book{
        Papery{
            index:u32
        },
        Electronic{
            url:String
        },
    }

let book = Book::Papery{index:10001};
let ebook = Book::Electronic{
    url:String::from("https://runoob.com/rust")
};
println!{"{:?}", book};
println!{"{:?}", ebook};

}

fn test63(){
    enum Book{
        Papery{index:u32},
        Electronic{url:String},
    }
    let book = Book::Papery{index:2000};

    match book{
        Book::Papery{index}=>{
            println!("Papery book{}", index);
        }
        Book::Electronic{url} =>{
            println!{"E-book {}", url};
        }
    }
}

fn test64(){
    enum Book{
        Papery{index:u32},
        Electronic{url:String},
    }

    let book = Book::Electronic { url: String::from("https://runoob.com/rust") };
    if let Book::Electronic { url } = book {
        println!("URL is {}", url)
    }


}

fn test65(){
    enum Book{
        Papery{index:u32},
        Electronic{url:String},
    }

    let book = Book::Electronic { url: String::from("url") };

    if let Book::Papery { index } = book {
        println!("Papery {}", index);
    }
    else{
        println!("Not papery book");
    }
}

fn test66(){
    #[derive(Debug)]
    enum Signal {
        Red,
        Yellow,
        Green,
    }

    impl Signal {
        fn red(&mut self){
            *self = Signal::Red;
        }

        fn yellow(&mut self){
            *self = Signal::Yellow;
        }

        fn green(&mut self){
            *self = Signal::Green;
        }
    }

    let mut signal = Signal::Red;
    println!("{:?}", signal);
    signal.yellow();
    println!("{:?}", signal);
    signal.green();
    println!("{:?}", signal);
    signal.red();
    println!("{:?}", signal);


}
fn get_lasti(array:&[i32]) ->i32{
    array[array.len() - 1]
}

fn test67(){
    let  a = [2,3,5,6,7,8];
    println!("a的最后一个元素是{}", get_lasti(&a));
}

fn get_last<T>(array:&[T])->&T{
    &array[array.len() - 1]
}

fn test68(){
    let a = ["Ada","Bert","C","CPP"];
    println!("a的最后一个元素是 {}",get_last::<&str>(&a));
}


fn test69(){
    #[derive(Debug)]
    struct Point<T>{
        x:T,
        y:T,
    }

    let point = Point::<i32>{
        x:1,
        y:2,
    };

    let point2 = Point::<f64>{
        x:1.0,
        y:5.6,
    };
    println!("{:?}",point);
    println!("{:?}",point2);

    let p1 = Point{x:1,y:3};
    let p2 = Point{x:1.0,y:3.0};
    println!("{:?}",p1);
    println!("{:?}",p2);
}

fn test70(){
    #[derive(Debug)]
    struct Point<T1,T2>{
        x:T1,
        y:T2,
    }

    let p = Point{x:1,y:2.0};
    println!("{:?}",p);
}

fn test71(){
    #[derive(Debug)]
    enum Shape<T> {
        Rectangle(T,T),
        Cube(T,T,T)
    }

    let s1 = Shape::Rectangle(1,2);
    let s2 = Shape::Cube(1.0, 2.0, 3.0);

    let s3:Shape<i32> = Shape::Rectangle(1, 2);
    let s4:Shape<f64> = Shape::<f64>::Cube(1.0, 2.0, 3.0);

    println!("{:?}",s1);
    println!("{:?}",s2);
    println!("{:?}",s3);
    println!("{:?}",s4);

}

fn test72(){
    struct Point<T>{
        x:T,
        y:T
    }

    impl <T> Point<T> {
        fn get_x(&self)->&T{
            &self.x
        }

        fn get_y(&self)->&T{
            &self.y
        }
    }

    let point = Point{x:3.0, y:4.0};
    println!("point is ({}, {})", point.get_x(), point.get_y());
}

fn main() {
    // test66();
    // test67();
    // test68();
    // test69();
    // test70();
    // test71();
    test72();


    // test51();
    // test52();
    // test53();
    // test54();
    // test55();
    // test56();
    // test57();
    // test58();
    // test59();
    // test60();
    // test61();
    // test62();
    // test63();
    // test64();
    // test65();




    // test41();
    // test42();
    // test43();
    // test45();
    // test46();
    // test47();
    // test48();
    // test49();
    // test50();



    // test31();
    // test32();
    // test33();
    // test34();
    // test35();
    // test36();
    // test37();
    // test38();
    // test39();
    // test40();

    //old！！！！
    //old！！！！
    //old！！！！

    // test3();
    // test4();
    // test5();
    // test6();
    // test7();
    // test8();
    // test9();
    // test10();
    // test11();
    // test12();
    // test13();
    // test14();
    // test15();
    // test16();
    // test17();
    // test18();
    // test19();
    // test20();
    // test21();
    // test22();
    // test23();
    // test24();
    // test25();
    // test26();
    // test27();
    // test28();
    // test29();
    // test30();

}
