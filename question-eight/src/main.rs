use itertools::Itertools;

fn main() {
    let mut sum = 0;

    ['R', 'I', 'T', 'A', 'N', 'G', 'L', 'E']
        .iter()
        .permutations(8)
        .for_each(|p| {
            if adjacent(&p) {
                sum += 1;
            }
        });

    println!("Sum: {sum}");
}

fn adjacent(p: &[&char]) -> bool {
    for (i, c) in p.iter().enumerate() {
        if let Some(d) = p.get(i + 1) {
            if ['I', 'A', 'E'].contains(c) && ['I', 'A', 'E'].contains(d) {
                return true;
            }
        }
    }
    false
}
