use rand::Rng;
use std::io;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "1")] // The number of die to roll, defaulted to 1
    die_count: u32,
    #[structopt(default_value = "20")] // The size of the die being rolled, defaulted to 20
    die_size: u32,
    #[structopt(
        short = "t",
        long = "total",
        help = "Show total result of the die rolled"
    )]
    // Shows the total amount rolled
    total: bool,
    #[structopt(short = "m", long = "min", help = "Show the minimum possible result")]
    // Shows the total amount rolled
    min: bool,
    #[structopt(short = "M", long = "max", help = "Show the maximum possible result")]
    // Shows the total amount rolled
    max: bool,
}

fn main() {
    let args = Cli::from_args();
    let mut total = 0;

    for _ in 1..args.die_count + 1 {
        let result = rand::thread_rng().gen_range(1, args.die_size + 1);
        total += result;
        print!("{} ", result);
        io::stdout().flush().unwrap();
    }

    println!("");

    if args.min {
        println!("Minimum: {}", args.die_count);
    }

    if args.max {
        println!("Maximum: {}", args.die_count * args.die_size);
    }

    if args.total {
        println!("Total: {}", total);
    }
}
