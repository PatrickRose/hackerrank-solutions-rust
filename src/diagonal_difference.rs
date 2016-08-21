use std;

// Given a square matrix of size N, calculate the absolute difference between the sums of its diagonals.
// Input Format
// The first line contains a single integer, N. The next lines denote the matrix's rows, with each line containing space-separated integers describing the columns.
// Output Format
// Print the absolute difference between the two sums of the matrix's diagonals as a single integer.
#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_it_calculates_the_first_diagonal() {
        let matrix = Matrix::new(String::from("11 2 4
4 5 6
10 8 -12"));
        assert_eq!(4, matrix.calculate_first_diagonal());
    }

    #[test]
    fn test_it_calculates_the_second_diagonal() {
        let matrix = Matrix::new(String::from("11 2 4
4 5 6
10 8 -12"));
        assert_eq!(19, matrix.calculate_second_diagonal());
    }

    #[test]
    fn test_it_calculates_the_difference_correctly() {
        let matrix = Matrix::new(String::from("11 2 4
4 5 6
10 8 -12"));
        assert_eq!(15, matrix.calculate_diagonal_difference());
    }

}

struct Matrix {
    num_lines: u32,
    lines: Vec<String>,
}

impl Matrix {

    fn new(input: String) -> Matrix {
        let mut lines: Vec<String> = Vec::new();
        let mut num_lines: u32 = 0;

        for line in input.lines() {
            num_lines += 1;
            lines.push(String::from(line));
        }
        
        return Matrix { lines: lines, num_lines: num_lines };
    }

    fn calculate_first_diagonal(&self) -> i64 {
        let mut answer: i64 = 0;
        for i in 0..self.num_lines {
            let ref line: String = self.lines[i as usize];
            let line_content: Vec<&str> = line.split_whitespace().collect();
            answer += parse_string(String::from(line_content[i as usize]));
        }
        
        return answer; 
    }

    fn calculate_second_diagonal(&self) -> i64 {
        let mut answer: i64 = 0;
        for i in 0..self.num_lines {
            let ref line: String = self.lines[i as usize];
            let line_content: Vec<&str> = line.split_whitespace().collect();
            answer += parse_string(String::from(line_content[(self.num_lines - i - 1) as usize]));
        }
        
        return answer; 
    }

    fn calculate_diagonal_difference(&self) -> i64 {
        let answer: i64 = self.calculate_first_diagonal() - self.calculate_second_diagonal();

        if answer < 0 {
            return answer * -1;
        }

        return answer;
    }
}

fn parse_string(s: String) -> i64 {
    let num = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not parse")
    };
     
   return num;
}
    
fn read_string() -> String {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).expect("Could not read stdin!");
   return input;
}
