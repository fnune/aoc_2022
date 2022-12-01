use anyhow::Result;
use colored::Colorize;
use log::{info, debug};

fn main() -> Result<()> {
    env_logger::init();

    let log = include_str!("../input.txt");

    let mut current_sum = 0;
    let mut current_max = 0;

    for line in log.split("\n") {
        debug!("Reading line: {}", line.trim());

        match line.trim() {
            "" => {
                debug!("🧝 New elf...");
                current_sum = 0;
            }
            _ => {
                let calories = line.parse::<usize>()?;
                current_sum += calories;
            }
        }

        if current_sum > current_max {
            current_max = current_sum;
            info!("👑 New winner: {}", current_max);
        }
    }

    println!("The maximum is 🥁: {}", current_max.to_string().green());

    Ok(())
}
