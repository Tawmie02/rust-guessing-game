    use std::io;//handle input output
    use std::cmp::Ordering;//comparison(less,greater,equal)
    use rand::Rng;//random  number generator
    fn main() {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);//function that gives the particular random no generator
        loop{
        println!("The secret number is: {secret_number}");//relevant for testing only
        println!("Please input your guess.");

        let mut guess = String::new();//create mutable variable that is bound to a new instance of a string

        io::stdin()//call the stdin function to handlw user input
            .read_line(&mut guess)//get input from user(passing the string as the argument)(referencing/borrowing)
            .expect("Failed to read line");//handle potential failure
        let guess: u32 = match guess.trim().parse() {//parse return a Result type
            //ignore a non-number..
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {//control flow when we are selecting one of the many code blocks to be executed
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;//quit after a win
            }
       }
    }
}