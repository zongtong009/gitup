fn main() {
    println!("Hello, world!");
    
    
    let a = 12;
    println!("a is {}, a again is {}", a, a); 
    println!("a is {0}, a again is {0}", a); 
    
    println!("{{}}"); 
    
    let mut a = 123;
    println!("a is {0}, a again is {0}", a); 
    a = 456;
    println!("a is {0}, a again is {0}", a); 
    
    
    /*
    let a = 123;   //重复定义是合法的,但需要用let，系统会推荐使用 _a
      //^ help: if this is intentional, prefix it with an underscore: `_a`
              //如果这是有意的，请在其前面加下划线：``u a 
    //使用 _a不会出现上方的warning
    //变量的值可以"重新绑定"，但在"重新绑定"以前不能私自被改变
    println!("a is {0}, a again is {0}", a); 
    */
    
    
    /*
    const a: i32 = 123;
    //  let a = 456;    此语句非法，因为const声明了常量
    //因为最开始定义了变量a，所以const a:i32 = 132也编译不过
    
    5  |     let a = 12;

   |         interpreted as a constant pattern, not a new variable
   |         help: introduce a variable instead: `a_var`    推荐使用a_var
20 |     const a: i32 = 123;
   |     ------------------- constant defined here
   |
   = note: the matched value is of type `i32`
   */
   
   
   //   重影（Shadowing）
   //   重影的概念与其他面向对象语言里的"重写"（Override）或"重载"（Overload）是不一样的。
   //   重影就是前面说的"重新绑定"
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    /*
    let mut s = "123";
    s = s.len();
    这段程序会出错：不能给字符串变量赋整型值,改变一个变量的类型是不被允许的
    */





    
}
