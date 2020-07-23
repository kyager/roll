use rand::Rng;
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
}

fn main() {
    let args = Cli::from_args();
    let die_count = args.die_count;
    let die_size = args.die_size;
    let mut total = 0;

    for _ in 1..die_count + 1 {
        let result = rand::thread_rng().gen_range(1, die_size + 1);
        total += result;
        println!("{}", result);
    }

    if args.total {
        println!("Total: {}", total);
    }
}
