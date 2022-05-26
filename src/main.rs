use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    

    let secret_number:u32 = rand::thread_rng().gen_range(1,101);

    //println!("the secret number is: {}" , secret_number);


    loop{

        let mut guss = String::new();
        println!("enter your guess:");
        // get user input
        io::stdin().read_line(&mut guss).expect("error");
       
    
        let guss2:u32 = match guss.trim().parse(){
            Ok(ret) => ret,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            },
        };
    
        match guss2.cmp(&secret_number){
            Ordering::Less => println!("{}","too small".red()),
            Ordering::Greater => println!("{}" , "too big".yellow()),
            Ordering::Equal => {
                println!("{}","you win!".green());
                break

            },
        }
    
    }

    

    
    //print the input value
    //println!("{}",guss);
}
