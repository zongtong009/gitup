fn main() {
    println!("Hello, world!");

    let mut n = 0;
    while n < 10 {
        println!("{}", n);
        n += 1;
    }
    let _f64_a = 1.0;

    let a = 1;
    match a {
        1 => println!("add"),
        2 => println!("add"),
        3 => println!("add"),
        9 => println!("add"),
        10 => println!("add"),
        _ => {},
    }
}
