fn main() {
    let a = 3;
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
    
    
    //在 C 语言中 for 循环使用三元语句控制循环
        //  for (int i = 0; i < 10; i++) { // 循环体 }
    //但是 Rust 中没有这种用法，需要用 while 循环来代替
    let mut i = 0;
    while i < 10 {
        // 循环体
        i += 1;
    }
    
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }
    
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
    
    let pi = 3.141592;
    println!("Hello, world!");

    let s = format!("Pi is {a}  roughly {:.*}  ", 1, pi, a = "a");
    println!("{0}", s);

    let _tup = (500, 6.4, 1);
    let (_x, y, _z) = _tup; //未使用变量要加 _
    println!("{}", y);
}
