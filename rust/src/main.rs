extern crate ring;
#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;
use ring::rand;

mod blockchain;

fn main() {
    let mut selection = String::new();

    let mut miner_addr = String::new();
    let mut difficulty = String::new();

    println!("Welcome to the Rust blockchain demo!");

    print!("Input a miner address: ");
    io::stdout().flush().expect("Stdout flush failed");
    io::stdin().read_line(&mut miner_addr)
        .expect("Failed to read line");
    print!("Set the mining difficulty: ");
    io::stdout().flush().expect("Stdout flush failed");
    io::stdin().read_line(&mut difficulty)
        .expect("Failed to read line");
    let miner_addr = miner_addr.trim().to_string();
    let difficulty = difficulty.trim().parse::<u32>()
        .expect("Difficulty must be an integer");
    println!("\nGenerating genesis block...");
    let mut chain = blockchain::Chain::new(miner_addr, difficulty);
    println!("Genesis block mined.");

    loop {
        println!("\n** Main Menu **");
        println!("Enter \"q!\" to exit");
        println!("1) New Transaction");
        println!("2) Mine");
        println!("3) Change Difficulty");
        print!("Enter your choice: ");
        io::stdout().flush().expect("Stdout flush failed");
        selection.clear();
        io::stdin().read_line(&mut selection)
            .expect("Failed to read line");
        println!();

        match selection.trim().to_string().as_ref() {
            "q!" => {
                println!("Goodbye!");
                process::exit(0);
            },
            "1" => {
                let mut sender = String::new();
                let mut recipient = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush().expect("Stdout flush failed");
                io::stdin().read_line(&mut sender)
                    .expect("Failed to read line");
                print!("Enter recipient address: ");
                io::stdout().flush().expect("Stdout flush failed");
                io::stdin().read_line(&mut recipient)
                    .expect("Failed to read line");
                print!("Enter amount to send: ");
                io::stdout().flush().expect("Stdout flush failed");
                io::stdin().read_line(&mut amount)
                    .expect("Failed to read line");

                let sender = sender.trim().to_string();
                let recipient = recipient.trim().to_string();
                let amount = amount.trim().parse::<u32>()
                    .expect("Amount must be an integer");

                let result = chain.add_transaction(sender, recipient, amount);
                match result {
                    true => println!("Transaction added!"),
                    _ => println!("Transaction failed")
                }
            },
            "2" => {
                println!("Generating a new block...");
                let result = chain.generate_new_block();
                match result {
                    true => println!("Successfully added block to chain"),
                    _ => println!("Failed to generate block")
                }
            },
            "3" => {
                let mut difficulty = String::new();
                print!("Set the mining difficulty: ");
                io::stdout().flush().expect("Stdout flush failed");
                io::stdin().read_line(&mut difficulty)
                    .expect("Failed to read line");
                let difficulty = difficulty.trim().parse::<u32>().unwrap();
                let result = chain.update_difficulty(difficulty);
                match result {
                    true => println!("Difficulty updated to {}", difficulty),
                    _ => println!("Failed to update difficulty")
                }
            },
            _ => println!("Invalid option. Please try again.")
        }
    }
}
