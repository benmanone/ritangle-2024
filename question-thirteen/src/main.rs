use itertools::{repeat_n, Itertools};

const DICE: [i32; 6] = [1, 2, 3, 4, 5, 6];

fn main() {
    let (mut possible, mut total) = (0, 0);

    // Possible permutations with replacement
    for (n, p) in repeat_n(DICE.iter(), 5)
        .multi_cartesian_product()
        .enumerate()
    {
        // 3-length combinations from dice roll permutation
        println!("{}: {p:?}", n + 1);
        for q in p.iter().combinations(3) {
            let (a, b, c) = (
                ***q.first().unwrap(),
                ***q.get(1).unwrap(),
                ***q.get(2).unwrap(),
            );

            // Triangle inequality theorum
            if a + b > c && a + c > b && b + c > a {
                println!("\t SOLVED: {q:?}");
                possible += 1;
                break;
            }
        }

        total += 1;
    }

    println!(
        "{}/{total} = {}",
        total - possible,
        f64::from(total - possible) / f64::from(total)
    );
}
