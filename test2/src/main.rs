use std::io::stdin;

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


fn main() {
    // test3();
    // test4();
    // test5();
    // test6();
    // test7();
    // test8();
    // test9();
    // test10();
    test11();

}
