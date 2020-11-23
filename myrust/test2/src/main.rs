/*
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    //return s.len();
}
*/


fn main() {
    let re = dangle();
    println!("The length of '{}' .", re);
}

//错误
//伴随着 dangle 函数的结束，其局部变量的值本身没有被当作返回值，被释放了。
//但它的引用却被返回，这个引用所指向的值已经不能确定的存在，故不允许其出现
//fn dangle() -> &String {
//   let s = String::from("hello");
//    &s    }

fn dangle() -> String {
    let s = String::from("hello");
    let s1= s;
    s1
}


