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

fn main() {
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
