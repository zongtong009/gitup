fn main() {
/*
    整数型（Integer）
整数型简称整型，按照比特位长度和有无符号分为一下种类：

    位长度	    有符号	无符号
    8-bit	    i8	    u8
    16-bit	    i16	    u16
    32-bit	    i32	    u32
    64-bit	    i64	    u64
    128-bit	    i128	u128
    arch	    isize	usize


    let x = 2.1; // f64
    let y: f32 = 3.43; // f32

    println!("{} {}",x,y);
    println!("{} {}",x,y);


    //元组用一对 ( ) 包括的一组数据，可以包含    不同种类    的数据：
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    let (x, y, z) = tup;
    // y 等于 6.4
 
*/ 
    //数组用一对 [ ] 包括的     同类型     数据。
    let a = [1, 2, 3, 4, 5];
    println!("{:?} ",a);
    let b = ["January", "February", "March"];

    let c: [i32; 5] = [1, 2, 3, 4, 5];

    let d = [3; 5]; // 等同于 let d = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];  // 数组访问

    //a[0] = 123; // 错误：数组 a 不可变
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确
    println!("{:?} ",a);
    
    
    another_function(5, 6);
    
    fn five() -> i32 {
        5
    }
    println!("five() 的值为: {}", five());

}

fn another_function(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}