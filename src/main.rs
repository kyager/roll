use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    die_count: u32, // The number of die to roll
    die_size: u32,  // The size of the die being rolled
}

fn main() {
    let args = Cli::from_args();
    let die_count = args.die_count;
    let die_size = args.die_size;

    for _ in 1..die_count + 1 {
        println!("{}", rand::thread_rng().gen_range(1, die_size + 1));
    }
}
