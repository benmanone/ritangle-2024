#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Bucket {
    n: i32,
    c: char,
}

impl Bucket {
    pub fn new(n: i32, c: char) -> Self {
        Self { n, c }
    }
}

fn main() {
    let mut triangles: Vec<Bucket> = Vec::new();
    let mut factors: Vec<Bucket> = Vec::new();

    let mut tn = 0;

    for i in 0..100000 {
        if i != 0 && ((f64::sqrt((8.0 * i as f64) + 1.0) - 1.0) / 2.0).fract() == 0.0 {
            triangles.push(Bucket::new(
                i,
                ['T', 'R', 'I', 'A', 'N', 'G', 'L', 'E'][tn % 8],
            ));
            tn += 1;
        }
    }

    for (n, i) in (0..100000).step_by(90).enumerate() {
        if i != 0 {
            factors.push(Bucket::new(
                i,
                ['R', 'I', 'T', 'A', 'N', 'G', 'L', 'E'][(n - 1) % 8],
            ));
        }
    }

    for f in factors {
        for t in &triangles {
            if &f == t {
                println!("{f:?}")
            }
        }
    }
}
