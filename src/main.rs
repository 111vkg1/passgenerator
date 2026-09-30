use std::io;
use rand::Rng;

fn main()
{
    println!("Password generator on rust");
    let mut len: i32 = 16;
    let mut include_special: bool;
    loop{
        println!("Input len [4-32]: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        len = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        if len < 4 || len > 32 {
            println!("Number must be >=4 & <=32. Input again");
            continue;
        }

        break;
    }
    loop{
        println!("Include special(!@#$%^&*) [y/n]: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        input = input.trim().to_lowercase();
        if matches!(input.as_str(), "y" | "yes" | "1") {
            include_special = true;
        } else if matches!(input.as_str(), "n" | "no" | "0") {
            include_special = false;
        } else {
            println!("Not accepted format (y/n/yes/no/1/0). Try again");
            continue;
        }
        break;
    }

    let mut charset = String::from("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    if include_special {charset.push_str("~!@#$%^&*()_+[]{}|;:,.<>?")};
    let chars: Vec<char> = charset.chars().collect();
    let pass: String = (0..len)
        .map(|_| chars[rand::thread_rng().gen_range(0..chars.len())])
        .collect();
    println!("Password: {}", pass);
}
