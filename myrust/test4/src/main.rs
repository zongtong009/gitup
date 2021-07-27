use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number");
    println!("input your guess");
    
    
    let secret_num=rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_num);
    
    
    
    loop{
        let mut guess=String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
        //println!("you guessed :{}",guess);
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        //因这一条改变了guess的类型，所以每次loop都要重新mut guess;
        println!("you guessed :{}",guess);
        //这条语句写在上句前面会额外回车一行，因为每次输入数字都要回车
        
        //match guess.cmp(&secret_num.to_string()) {
        match guess.cmp(&secret_num) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                                },
        }
    }
    
    
}
