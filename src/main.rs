use rand::Rng;

fn main() {
    for n in 1..7 {
        // Run 6 times for each different stat
        let mut roll = Vec::new();
        roll.push(rand::thread_rng().gen_range(1, 7));
        roll.push(rand::thread_rng().gen_range(1, 7));
        roll.push(rand::thread_rng().gen_range(1, 7));
        roll.push(rand::thread_rng().gen_range(1, 7));

        roll.sort_unstable(); // Sort the values
        roll.reverse(); // Reverse the order since it sorts for lowest to highest
        roll.pop(); // Remove the last, lowest value
        println!("Stat {}: {}", n, roll[0] + roll[1] + roll[2]);

        drop(roll);
    }
}
