use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Diagnostics {
    gamma: Vec<u8>,
    epsilon: Vec<u8>,
    oxygen: Vec<u8>,
    co2: Vec<u8>,
}

pub enum LifeSupportRating {
    Oxygen,
    CO2,
}

impl Diagnostics {
    pub fn power_consumption(&self) -> u32 {
        Diagnostics::to_number(&self.gamma) * Diagnostics::to_number(&self.epsilon)
    }

    pub fn life_support_rating(&self) -> u32 {
        Diagnostics::to_number(&self.oxygen) * Diagnostics::to_number(&self.co2)
    }

    fn to_number(bit_vec: &[u8]) -> u32 {
        //assuming bits are in little_endian
        bit_vec
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (place, &bit)| {
                if bit > 1 {
                    panic!("invalid bit")
                } else {
                    acc + (bit as u32).rotate_left(place as u32)
                }
            })
    }

    pub fn new(data: &str) -> Result<Diagnostics> {
        let line_len = match data.lines().next() {
            Some(x) => x.len(),
            None => return Err(anyhow!("Data must contain one line")),
        };
        let gamma = (0..line_len)
            .map(|i| {
                Diagnostics::most_common_bit(data.lines(), i).and_then(Diagnostics::char_to_u8)
            })
            .collect::<Result<Vec<_>>>()?;
        let epsilon = (0..line_len)
            .map(|i| {
                Diagnostics::least_common_bit(data.lines(), i).and_then(Diagnostics::char_to_u8)
            })
            .collect::<Result<Vec<_>>>()?;
        let oxygen = Diagnostics::get_life_rating(data, LifeSupportRating::Oxygen)?;
        let co2 = Diagnostics::get_life_rating(data, LifeSupportRating::CO2)?;
        Ok(Diagnostics {
            gamma,
            epsilon,
            oxygen,
            co2,
        })
    }

    fn char_to_u8(c: char) -> Result<u8> {
        match c {
            '1' => Ok(1),
            '0' => Ok(0),
            _ => Err(anyhow!("can only be 1 or 0")),
        }
    }

    fn most_common_bit<'a, I>(data: I, place: usize) -> Result<char>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut len = 0;
        let mut acc = 0;
        for line in data {
            len += 1;
            acc += match line.chars().nth(place) {
                Some('1') => 1,
                Some('0') => 0,
                Some(_) => return Err(anyhow!("input must only contain 1 or 0")),
                None => return Err(anyhow!("trying to access bit larger than input")),
            }
        }
        Ok(if (2 * acc).cmp(&len).is_ge() {
            '1'
        } else {
            '0'
        })
    }

    fn least_common_bit<'a, I>(data: I, place: usize) -> Result<char>
    where
        I: Iterator<Item = &'a str>,
    {
        Diagnostics::most_common_bit(data, place).map(|o| match o {
            '1' => '0',
            '0' => '1',
            _ => unreachable!(),
        })
    }

    fn get_life_rating(data: &str, life_support_rating: LifeSupportRating) -> Result<Vec<u8>> {
        let mut numbers = data.lines().map(String::from).collect::<Vec<_>>();
        let mut place: usize = 0;
        while numbers.len() > 1 {
            let matcher = match life_support_rating {
                LifeSupportRating::Oxygen => {
                    Diagnostics::most_common_bit(numbers.iter().map(|o| o.as_str()), place)?
                }
                LifeSupportRating::CO2 => {
                    Diagnostics::least_common_bit(numbers.iter().map(|o| o.as_str()), place)?
                }
            };
            numbers = numbers
                .into_iter()
                .filter(|o| {
                    o.chars()
                        .nth(place)
                        .map(|c| c == matcher)
                        .unwrap_or_default()
                })
                .collect();

            place += 1;
        }
        match numbers.pop() {
            Some(s) => s
                .chars()
                .map(Diagnostics::char_to_u8)
                .collect::<Result<Vec<_>>>(),
            None => Err(anyhow!("empty set while calculating life support rating")),
        }
    }
}

fn main() -> Result<()> {
    let diagnostics = Diagnostics::new(include_str!("data.txt"))?;
    println!("{}", diagnostics.power_consumption());
    println!("{}", diagnostics.life_support_rating());
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let expected_gam = vec![1, 0, 1, 1, 0u8];
        let expected_eps = vec![0, 1, 0, 0, 1u8];
        let diagnostics = Diagnostics::new(include_str!("test.txt")).unwrap();
        for (res, expected) in diagnostics.gamma.iter().zip(expected_gam.iter()) {
            assert_eq!(res, expected);
        }
        for (res, expected) in diagnostics.epsilon.iter().zip(expected_eps.iter()) {
            assert_eq!(res, expected);
        }

        assert_eq!(Diagnostics::to_number(&diagnostics.gamma), 22);
        assert_eq!(Diagnostics::to_number(&diagnostics.epsilon), 9);
        assert_eq!(Diagnostics::to_number(&diagnostics.oxygen), 23);
        assert_eq!(Diagnostics::to_number(&diagnostics.co2), 10);
        assert_eq!(diagnostics.power_consumption(), 198);
        assert_eq!(diagnostics.life_support_rating(), 230);
    }
}
