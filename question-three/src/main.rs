// Provides iterator adaptors for combinations & uniqueness
use itertools::Itertools;

fn main() {
    let coins = [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60];
    let mut max = 0;

    // All possible length-12 combinations of coins
    coins
        .iter()
        .combinations_with_replacement(12)
        .for_each(|p| {
            let sum = p.clone().into_iter().sum::<i32>();
            if sum > max && no_sum(&p) {
                println!("{p:?}, {sum}");
                max = sum;
            }
        });

    println!("GREATEST = {max}");
}

// Check no elements add up to 120
fn no_sum(vec: &Vec<&i32>) -> bool {
    // Check all possible unique combinations lengths 2-12 of the coins in the combination Vec
    for len in 2..=vec.len() {
        for com in vec.iter().combinations(len).unique() {
            if com.into_iter().copied().sum::<i32>() == 120 {
                return false;
            }
        }
    }
    true
}
