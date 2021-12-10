pub enum ParserType {
    Corruption,
    Completion,
}
trait ParseScorer {
    fn is_open(&self) -> bool;
    fn is_close(&self) -> bool;
    fn get_partner(&self) -> Self;
    fn get_score(&self, parse_type: ParserType) -> u64;
}
impl ParseScorer for char {
    fn is_open(&self) -> bool {
        match *self {
            '{' | '<' | '[' | '(' => true,
            _ => false,
        }
    }

    fn is_close(&self) -> bool {
        match *self {
            '}' | '>' | ']' | ')' => true,
            _ => false,
        }
    }

    fn get_partner(&self) -> char {
        match *self {
            '{' => '}',
            '<' => '>',
            '[' => ']',
            '(' => ')',
            '}' => '{',
            '>' => '<',
            ']' => '[',
            ')' => '(',
            _ => unreachable!(),
        }
    }

    fn get_score(&self, parse_type: ParserType) -> u64 {
        match parse_type {
            ParserType::Corruption => match *self {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            },
            ParserType::Completion => match *self {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            },
        }
    }
}

fn is_corrupted(line: &str) -> Option<u64> {
    let mut stack = Vec::with_capacity(line.len());
    for c in line.chars() {
        if c.is_open() {
            stack.push(c);
        } else if c.is_close() {
            match stack.last() {
                Some(o) => {
                    if c.get_partner() == *o {
                        stack.pop();
                    } else {
                        return Some(c.get_score(ParserType::Corruption));
                    }
                }
                None => return Some(c.get_score(ParserType::Corruption)),
            }
        }
    }
    None
}

fn is_unfinished(line: &str) -> Option<u64> {
    let mut stack = Vec::with_capacity(line.len());
    for c in line.chars() {
        if c.is_open() {
            stack.push(c);
        } else if c.is_close() {
            match stack.last() {
                Some(o) => {
                    if c.get_partner() == *o {
                        stack.pop();
                    } else {
                        // is corrupted not unfinished
                        return None;
                    }
                }
                None => return None,
            }
        }
    }
    if stack.len() == 0 {
        None
    } else {
        Some(
            stack
                .iter()
                .rev()
                .map(|c| c.get_partner())
                .fold(0, |acc, c| (acc * 5) + c.get_score(ParserType::Completion)),
        )
    }
}

fn main() {
    let lines = include_str!("../data/main.txt");
    println!(
        "{}",
        lines
            .lines()
            .filter_map(|line| is_corrupted(line))
            .sum::<u64>()
    );
    let mut scores: Vec<_> = lines
        .lines()
        .filter_map(|line| is_unfinished(line))
        .collect();
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_corruption_score() {
        assert_eq!(
            include_str!("../data/test.txt")
                .lines()
                .filter_map(|line| is_corrupted(line))
                .sum::<u64>(),
            26397
        )
    }

    #[test]
    fn test_completion_score() {
        let mut scores: Vec<_> = include_str!("../data/test.txt")
            .lines()
            .filter_map(|line| is_unfinished(line))
            .collect();
        scores.sort();
        println!("{:?}", scores);
        assert_eq!(288957, scores[scores.len() / 2]);
    }
}
