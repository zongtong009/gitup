fn main() {
    println!("Hello, world!");
    
    let es = 0.0;
    let fact=1.0;
    let i=1;
    for (i=1;;fact*=((double)i++)){
        es+=(1.0/fact);
    }
         
    return 0;
}
