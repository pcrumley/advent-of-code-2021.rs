use csv::ReaderBuilder;

fn main() {
    // the first puzzle is you are given an array and you are supposed to count how
    // many times is n+1 deeper than nth element of the array.
    // work to do to make deserialization better
    let depths_test = parse_csv(include_bytes!("depths_test.csv").as_ref());
    let depths = parse_csv(include_bytes!("depths.csv").as_ref());
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

fn parse_csv(data: &[u8]) -> Vec<u32> {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(data);
    rdr.records()
        .map(|record| {
            record
                .ok()
                .and_then(|o| o[0].to_string().parse::<u32>().ok())
                .unwrap()
        })
        .collect()
}
