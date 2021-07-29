fn main() {
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
