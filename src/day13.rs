use std::collections::HashSet;

pub fn part1() {
    let mut lines = crate::util::data_lines(13);
    let time = <usize as std::str::FromStr>::from_str(&lines.next().unwrap()).unwrap();
    let busses = lines.next().unwrap();
    let busses: HashSet::<_> = busses.split(',').filter(|bus| *bus != "x").map(<usize as std::str::FromStr>::from_str).map(Result::unwrap).collect();
    let (id, to_wait) = busses.iter().map(|id| (id, id - (time % id))).min_by_key(|(_id, to_wait)| *to_wait).unwrap();
    println!("Day 13, Part 1: {}", id * to_wait);
}

// If `extended_euclidean_algorithm(a, b) = (gcd, bezout)` then `gcd = left * a + right * b`
struct Bezout {
    left: i128,
    right: i128,
}

fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, Bezout) {
    let remainder = a % b;
    let quotient = a / b;
    if remainder == 0 {
        // Base case, b divides a:
        //   `GCD(a, b) = b`
        //   `b = 0 * a + 1 * b`
        (b, Bezout { left: 0, right: 1})
    } else {
        // We have `a = b * quotient + remainder` and remainder is non-zero
        let (gcd, bezout) = extended_euclidean_algorithm(b, remainder);
        // We have gcd = left_coeff * b + right_coeff * remainder
        //  => gcd = left_coeff * b + right_coeff * (a - b * quotient)
        //  => gcd = a * right_coeff + b * (left_coeff - quotient * right_coeff)
        (gcd, Bezout { left: bezout.right, right: bezout.left - quotient * bezout.right })
    }
}

#[test]
fn test_eea_240_46() {
    let (gcd, bezout) = extended_euclidean_algorithm(240, 46);
    assert!(gcd == 2);
    assert!(bezout.left == -9);
    assert!(bezout.right == 47);
}

// Find the inverse of `x` in `Z_m`
fn modulus_inverse(x: i128, m: i128) -> i128 {
    let (gcd, bezout) = extended_euclidean_algorithm(m, x);
    assert!(gcd == 1);
    assert!(m * bezout.left + x * bezout.right == gcd);
    // we have `1 == left * m + right * x` thus `1 === right.x (mod m)` so `right` is the inverse
    // of x in `Z_m`
    ((bezout.right % m) + m) % m
}

pub fn part2() {
    let mut lines = crate::util::data_lines(13);
    let _ = lines.next().unwrap(); // Ignore the first line
    let busses = lines.next().unwrap();
    let busses = busses.split(',').map(|id| -> Option<i128> {
        if id == "x" {
            None
        } else {
            Some(std::str::FromStr::from_str(id).unwrap())
        }
    });
    let mut product_of_moduli = 1;
    let busses = busses.enumerate().filter_map(|(offset, id)| {
        match id {
            Some(id) => {
                product_of_moduli *= id;
                let offset = id - offset as i128;
                Some((offset, id))
            }
            None => None
        }
    }).collect::<Vec<_>>();

    let earliest_time = busses.into_iter().map(move |(offset, id)| {
        let prod = product_of_moduli / id;
        prod * offset * modulus_inverse(prod, id)
    }).sum::<i128>();

    let earliest_time = ((earliest_time % product_of_moduli) + product_of_moduli) % product_of_moduli;

    println!("Day 13, Part 2: {}", earliest_time);
}
