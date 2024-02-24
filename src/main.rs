use std::collections::HashMap;

fn get_prime_factors(num: u32) -> Vec<(u32, u32)> {
    let mut current = num;
    let mut result = vec![];
    let mut i = 2;

    loop {
        if current % i == 0 {
            let mut power = 0;
            while current % i == 0 {
                current /= i;
                power += 1
            }
            result.push((i, power))
        }

        i += 1;
        if i > current {
            break;
        }
    }

    result
}

fn main() {
    let mut prime_factors = HashMap::<u32, u32>::new();

    for i in 1..=20 {
        for (base, new_power) in get_prime_factors(i) {
            if let Some(old_power) = prime_factors.get_mut(&base) {
                *old_power = (*old_power).max(new_power)
            } else {
                prime_factors.insert(base, new_power);
            }
        }
    }

    let result = prime_factors
        .into_iter()
        .fold(1, |acc, (base, power)| acc * base.pow(power));
    println!("{}", result)
}
