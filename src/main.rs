use rug::Integer;
use std::io::{self, Write};

fn main() {
    print!(
        r#"
                    █████╗ ██╗  ██╗███████╗
                    ██╔══██╗██║ ██╔╝██╔════╝
                    ███████║█████╔╝ ███████╗
                    ██╔══██║██╔═██╗ ╚════██║
                    ██║  ██║██║  ██╗███████║
                    ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝
                         - algorithm - 

                    by Álvaro Buj 
                    @buj8 on GitHub
        "#
    );
    print!("\n\tBased on the AKS primality test algorithm developed in 2002 by \n");
    print!("\t- Manindra Agrawal\n");
    print!("\t- Neeraj Kayal\n");
    print!("\t- Nitin Saxena\n\n");
    loop {
        print!("Enter a number to check if it is prime (0 to exit)\n> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Integer = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please enter a valid number.\n\n");
                continue;
            }
        };
        if input.is_zero() {
            break;
        }
        let is_prime = aks_rs::aks(&input);
        if is_prime {
            print!("{} is a prime number.\n\n", input);
        } else {
            print!("{} is not a prime number.\n\n", input);
        }
    }
}
