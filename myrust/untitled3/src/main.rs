fn main() {
    let greetings = ["Hello", "Hola", "Bonjour",
        "Ciao", "こんにちは", "안녕하세요",
        "Cześć", "Olá", "Здравствуйте",
        "Chào bạn", "您好"];

    // for (num, greeting) in greetings.iter().enumerate() {
    //     print!("{} : ", greeting);
    //     print!("{}", num);
    //     match num {
    //         0 => println!("This code is editable and runnable!"),
    //         1 => println!("¡Este código es editable y ejecutable!"),
    //         2 => println!("Ce code est modifiable et exécutable !"),
    //         3 => println!("Questo codice è modificabile ed eseguibile!"),
    //         4 => println!("このコードは编集して実行出来ます！"),
    //         5 => println!("여기에서 코드를 수정하고 실행할 수 있습니다!"),
    //         6 => println!("Ten kod można edytować oraz uruchomić!"),
    //         7 => println!("Esse código é editável e executável!"),
    //         8 => println!("Этот код можно отредактировать и запустить!"),
    //         9 => println!("Bạn có thể edit và run code trực tiếp!"),
    //         10 => println!("这段代码是可以编辑并且能够运行的！"),
    //         _ => {}
    //     }
    //
    // }
    // let a = 11;
    //
    // if let i = a {
    //     println!("{}", i);
    //     println!("{}", a);
    // }
    // let s = String::from("fsdgf");
    //
    // if let si = s {
    //     println!("{}", si);
    //     // println!("{}", s);
    // }
    let c: char = 'f';
    if 'f' == c {
        println!("yes");
    }
	
	
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
    //println!("{{{{{}}}}}", a);
    let x: char = { '~' };
    println!("{}", x);

    let ax: String = String::from("fdsawsg");
    let cx: &str = "friday";
    println!("{}  {} ", ax, cx);

    let mut z: String = String::new();
    z.push_str("简单教程 ");
    println!("{}", z);
    z.push_str("简单个屁");
    println!("{}", z);

    let name1: String = "简单教程 简单编程".to_string(); //原字符串对象
    let name2: String = name1.replace("程", "");
    println!("{}", name2);
}