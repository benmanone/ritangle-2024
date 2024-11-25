use is_prime::*;

fn main() {
    // Find 1-3
    for one_dn in 100..=999 {
        let one_dn_1st = 100 * nth_digit(one_dn, 0).unwrap();

        for one_ac in one_dn_1st..=(one_dn_1st + 99) {
            let one_ac_2nd = 10 * nth_digit(one_ac, 1).unwrap();

            for two_dn in one_ac_2nd..=(one_ac_2nd + 9) {
                let two_dn_2nd = if let Some(r) = nth_digit(two_dn, 1) {
                    10 * r
                } else {
                    break;
                };

                for three_ac in two_dn_2nd..=(two_dn_2nd + 9) {
                    let three_ac_2nd = if let Some(r) = nth_digit(three_ac, 1) {
                        10 * r
                    } else {
                        break;
                    };

                    for four_dn in three_ac_2nd..=(three_ac_2nd + 9) {
                        if three_ac == (four_dn - digit_sum(four_dn))
                            && two_dn == digit_product(one_ac) + digit_product(one_dn)
                            && digit_sum(one_dn) == digit_sum(one_ac)
                            && digit_sum(one_dn) == digit_sum(two_dn)
                        {
                            println!("one_dn: {one_dn}, one_ac: {one_ac}, two_dn: {two_dn}, three_ac: {three_ac}, four_dn: {four_dn}");
                        }
                    }
                }
            }
        }
    }

    println!("\n\n\n");

    // Find 4-5
    for four_dn in 50..=59 {
        for five_ac in 400..=499 {
            if f32::sqrt((five_ac - four_dn) as f32).fract() == 0.0
                && nth_digit(four_dn, 1).unwrap() == nth_digit(five_ac, 2).unwrap()
                && is_prime(&digit_sum(five_ac).to_string())
            {
                println!("4: {four_dn}, 5: {five_ac}")
            }
        }
    }
}

fn digit_sum(n: u32) -> u32 {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum()
}

fn digit_product(n: u32) -> u32 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .product()
}

fn nth_digit(int: u32, n: usize) -> Option<u32> {
    if let Some(c) = int.to_string().chars().nth(n) {
        Some(c.to_digit(10)?)
    } else {
        None
    }
}
