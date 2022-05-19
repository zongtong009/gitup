fn main() {
    println!("Hello, world!");
    #[derive(Debug)]
    enum Sen<E> {
        Error(E),

    }
    fn m() -> Sen<i32> {
        Sen::Error(1)
    }
    match  { }
}
