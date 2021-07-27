fn main() {
    println!("Hello, world!");
    let a=[0;a_len()];
    println!("{:?}",a);
    println!("{:?}",std::f32::INFINITY);
    println!("{:?}",std::f32::NEG_INFINITY);
    println!("{:?}",std::f32::NAN);
    println!("{:?}",std::f32::MIN);
    println!("{:?}",std::f32::MAX);
    println!("////////////////////////////////////////");
    //let a=;
    let a=std::f32::MIN.to_string();
    let b=std::f32::MAX.to_string();
    let c=std::f64::MIN.to_string();
    let d=std::f64::MAX.to_string();
    println!("{}    {}  {}  {}",a.len(),b.len(),c.len(),d.len());
    println!("{:?}",std::f64::INFINITY);
    println!("{:?}",std::f64::NEG_INFINITY);
    println!("{:?}",std::f64::NAN);
    println!("{:?}",std::f64::MIN);
    println!("{:?}",std::f64::MAX);
}
    const fn a_len() -> usize {
        return 5+11;
    }