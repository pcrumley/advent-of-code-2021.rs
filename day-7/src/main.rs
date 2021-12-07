// Part one is the same thing as minimizng L1 norm.
// I could write a fancy optimizer but for now I will just
// brute force it

fn l1_error(val: &[i32], guess: i32) -> u32 {
    return val
        .iter()
        .map(|o| (o - guess).checked_abs().unwrap() as u32)
        .sum::<u32>();
}

fn crab_walk(val: &[i32], guess: i32) -> u32 {
    return val
        .iter()
        .map(|o| (0..=(o - guess).checked_abs().unwrap() as u32).sum::<u32>())
        .sum();
}
fn min_cost(val: &[i32], cost_fn: fn(&[i32], i32) -> u32) -> u32 {
    let min = *val.iter().min().unwrap();
    let max = *val.iter().max().unwrap();
    (min..max + 1)
        .map(|guess| cost_fn(val, guess))
        .min()
        .unwrap()
}

fn main() {
    let data = include_str!("../data/full.txt")
        .split(",")
        .map(|o| o.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    println!("parsed input");
    println!(
        "The min l1 norm of constant guess is: {}",
        min_cost(&data, l1_error)
    );
    println!(
        "The min of that weirder crab walk cost fn is: {}",
        min_cost(&data, crab_walk)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_l1() {
        let data = include_str!("../data/test.txt")
            .split(",")
            .map(|o| o.trim().parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(min_cost(&data, l1_error), 37);
        assert_eq!(min_cost(&data, crab_walk), 168);
    }
}
