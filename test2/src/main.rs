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


fn main() {
    // test31();
    test32();

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
