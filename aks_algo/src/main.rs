use std::io::Write;

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
    print!("\n\tBased on the AKS primality test algorithm developed in 2002 by \n\t- Manindra Agrawal\n\t- Neeraj Kayal\n\t- Nitin Saxena\n\n");
    loop {
        print!("Enter a number to check if it is prime (0 to exit)\n> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please enter a valid number.\n\n");
                continue;
            }
        };
        if input == 0 {
            break;
        }
        let is_prime = aks_algo::aks(input);
        if is_prime {
            print!("{} is a prime number.\n\n", input);
        } else {
            print!("{} is not a prime number.\n\n", input);
        }
    }
}
