use anyhow::Result;

fn main() -> Result<()> {
    // the first puzzle is you are given an array and you are supposed to count how
    // many times is n+1 deeper than nth element of the array.
    // work to do to make deserialization better
    let depths_test = parse_file(include_str!("depths_test.csv"))?;
    let depths = parse_file(include_str!("depths.csv"))?;
    assert_eq!(7, num_increasing(depths_test.iter().cloned()));
    println!("{}", num_increasing(depths.iter().cloned()));

    assert_eq!(
        5,
        num_increasing(depths_test.windows(3).map(|o| o.iter().sum::<u32>()))
    );

    println!(
        "{}",
        num_increasing(depths.windows(3).map(|o| o.iter().sum::<u32>()))
    );
    Ok(())
}

fn num_increasing<I>(depths: I) -> u32
where
    I: Iterator<Item = u32>,
{
    depths
        .fold((0, u32::MAX), |(sum, prev), cur| {
            if cur.cmp(&prev).is_gt() {
                (sum + 1, cur)
            } else {
                (sum, cur)
            }
        })
        .0
}

fn parse_file(data: &str) -> Result<Vec<u32>> {
    data.lines()
        .map(|o| o.parse::<u32>().map_err(anyhow::Error::from))
        .collect()
}
