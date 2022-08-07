use std::io;

fn main() {
    println!("Let's discover fibonacci sequence!");
    println!("Which position do you want to know the value?");
    loop {
        let mut position = String::new();
        io::stdin()
            .read_line(&mut position)
            .expect("Failed to read line");

        let position: u32 = position.trim().parse().expect("Please type a number!");
        let result = fibonacci(position);
        println!("The value of the {} position is {}", position, result);

        println!("Do you want to know another value? (y/n)");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        let answer = answer.trim();
        if answer == "n" {
            break;
        }
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
