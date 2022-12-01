use anyhow::Result;
use colored::Colorize;

enum CalorieLogLine {
    CalorieCount(usize),
    ChangeOfElf,
}

fn main() -> Result<()> {
    let log = include_str!("../../day_01_01/input.txt");

    // This time I tried to use a more functional style than the solution in day_01_01. I think the
    // imperative counterpart was easier to write, and is also shorter. It also looks like this one
    // uses less `mut` variables but if you inspect the `fold` call below you'll see mutability and
    // dereferencing.
    //
    // One good thing about this solution is that I could use types to represent each line, giving
    // the program a sort of semantic DSL.
    //
    // I'm sure this can be made shorter and nicer...
    let mut sums: Vec<usize> = log
        .split("\n")
        .map(|line| match line {
            "" => Some(CalorieLogLine::ChangeOfElf),
            _ => {
                let count = line.parse::<usize>().ok()?;
                Some(CalorieLogLine::CalorieCount(count))
            }
        })
        // NB: fold allows different types in the accumulator and in the item, but reduce doesn't;
        // i.e. if your items are T you must also reduce into T.
        .fold(vec![], |mut calorie_counts, line| {
            match line {
                Some(CalorieLogLine::CalorieCount(count)) => {
                    if let Some(last_sum) = calorie_counts.last_mut() {
                        *last_sum += count;
                    } else {
                        calorie_counts.push(count);
                    };
                }
                Some(CalorieLogLine::ChangeOfElf) => {
                    calorie_counts.push(0);
                }
                _ => {}
            }
            calorie_counts
        });

    sums.sort();

    println!(
        "The sum of the three biggest is is 🥁: {}",
        sums.iter().rev().take(3).sum::<usize>().to_string().green()
    );

    Ok(())
}
