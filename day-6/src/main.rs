use anyhow::{anyhow, Result};

pub struct Ocean {
    fish: [u64; 9],
}

impl Ocean {
    pub fn build(data: &str) -> Result<Self> {
        let mut fish = [0; 9];
        for elm in data.trim().split(",") {
            if let Ok(x) = elm.parse::<usize>() {
                // let it panic
                fish[x] += 1;
            } else {
                return Err(anyhow!("invalid input"));
            }
        }
        Ok(Ocean { fish })
    }

    pub fn step_time(&mut self) {
        let mut next_fish = [0; 9];
        for i in 1..self.fish.len() {
            next_fish[i - 1] += self.fish[i];
        }
        next_fish[8] += self.fish[0];
        next_fish[6] += self.fish[0];
        self.fish = next_fish;
    }
}
fn main() {
    let data = include_str!("data.txt");
    let mut ocean = Ocean::build(data).unwrap();

    for _ in 0..80 {
        ocean.step_time()
    }
    println!(
        "number of fish at 80 days: {}",
        ocean.fish.iter().sum::<u64>()
    );
    for _ in 80..256 {
        ocean.step_time()
    }
    println!(
        "number of fish at 256 days: {}",
        ocean.fish.iter().sum::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let test_data = include_str!("test.txt");
        let mut ocean = Ocean::build(test_data).unwrap();

        for _ in 0..18 {
            ocean.step_time()
        }
        assert_eq!(ocean.fish.iter().sum::<u64>(), 26);

        for _ in 18..80 {
            ocean.step_time()
        }
        assert_eq!(ocean.fish.iter().sum::<u64>(), 5934);
        for _ in 80..256 {
            ocean.step_time()
        }
        assert_eq!(ocean.fish.iter().sum::<u64>(), 26984457539);
    }
}
