use clap::Parser;
use rand::Rng;

#[derive(Parser)]
#[command(name = "passgen")]
struct Args {
    #[arg(short, long, default_value_t = 16)]
    len: usize,

    #[arg(short, long)]
    special: bool,
}

fn main()
{
    let args = Args::parse();
    let mut charset = String::from("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    if args.special {charset.push_str("~!@#$%^&*()_+[]{}|;:,.<>?")};
    let chars: Vec<char> = charset.chars().collect();
    let pass: String = (0..args.len)
        .map(|_| chars[rand::thread_rng().gen_range(0..chars.len())])
        .collect();
    println!("{}", pass);
}

