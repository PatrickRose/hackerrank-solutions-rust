// Given an array of integers, calculate which fraction of its elements are positive,
// which fraction of its elements are negative, and which fraction of its elements are zeroes, 
// respectively. Print the decimal value of each fraction on a new line.

use std;

#[cfg(test)]
mod tests {

    use super::Counter;
    
    #[test]
    fn test_it_calculates_the_fraction_of_positives() {
        let counter = Counter::new(6, String::from("-4 3 -9 0 4 1"));

        assert_eq!("0.500000", counter.positives());
    }
    
    #[test]
    fn test_it_calculates_the_fraction_of_negatives() {
        let counter = Counter::new(6, String::from("-4 3 -9 0 4 1"));

        assert_eq!("0.333333", counter.negatives());
    }
    
    #[test]
    fn test_it_calculates_the_fraction_of_zeros() {
        let counter = Counter::new(6, String::from("-4 3 -9 0 4 1"));

        assert_eq!("0.166667", counter.zeros());
    }
}

#[allow(dead_code)]
struct Counter {
    num_elements: u64,
    positive: u32,
    negative: u32,
    zeros: u32
}

#[allow(dead_code)]
impl Counter {

    fn new(num_elements: u64, input: String) -> Counter {
        let mut positive: u32 = 0;
        let mut negative: u32 = 0;
        let mut zeros: u32 = 0;
        let zero: i64 = 0;
        
        for number in input.split_whitespace() {
            let num: i64 = parse_string(String::from(number));

            match num.cmp(&zero) {
                std::cmp::Ordering::Less => negative+=1,
                std::cmp::Ordering::Equal => zeros+=1,
                std::cmp::Ordering::Greater => positive+=1,
            }
        }
        
        return Counter {
            num_elements: num_elements,
            positive: positive,
            negative: negative,
            zeros: zeros
        }
    }

    fn positives(&self) -> String {
        let answer: f64 = self.positive as f64 / self.num_elements as f64;
        return format!("{:.6}", answer);
    }

    fn negatives(&self) -> String {
        let answer: f64 = self.negative as f64 / self.num_elements as f64;
        return format!("{:.6}", answer);
    }

    fn zeros(&self) -> String {
        let answer: f64 = self.zeros as f64 / self.num_elements as f64;
        return format!("{:.6}", answer);
    }
    
}

#[allow(dead_code)]
fn parse_string(s: String) -> i64 {
    let num = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not parse")
    };
     
   return num;
}

#[allow(dead_code)]    
fn read_string() -> String {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).expect("Could not read stdin!");
   return input;
}

#[allow(dead_code)]
fn main() {
    let num_elements = read_string();
    let num_elements = parse_string(num_elements);
    
    let input = read_string();

    let counter = Counter::new(num_elements as u64, input);

    println!("{}", counter.positives());
    println!("{}", counter.negatives());
    println!("{}", counter.zeros());
}
