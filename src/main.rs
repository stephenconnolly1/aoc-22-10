use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

enum Instruction {
    Addx(i32),
    Noop,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len()==2, "You must provide a file argument");
    let file_path = &args[1];
    process_instructions(file_path);   
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Instruction {
    let mut toks = line.split(' ').fuse();
    let first = toks.next();
    let mut distance: i32 = 0;
    match first {
        Some("noop") => {
//            println!("noop");
            Instruction::Noop
        },
        Some("addx") => {
//            println!("addx");
            match toks.next() {
                Some(num) => {
                    distance = num.parse::<i32>().unwrap();
                    Instruction::Addx(distance)
                },
                _ => panic!("Bad number format"), 
            }
        },
        _ => panic!("Bad line format"),
    }    
}
fn check_register(cycle:  &i32, register: &i32) -> i32 {
    println!("Cycle: {}, Register: {}, Signal Strength: {}", cycle, register, cycle * register);
    match cycle {
        20|60|100|140|180|220=> {
            cycle * register
        }, 
        _ => 0,
    }
}

fn process_instructions(file_path: &String) -> i32{
    let mut cycle: i32 = 1;
    let mut register: i32 = 1;
    let mut sum_strength: i32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {
                println! ("{:?}", l);
                let i: Instruction = parse_line(&l);
                match i {
                    Instruction::Addx(arg) => {
                        sum_strength = sum_strength + check_register(&cycle, &register);

                        cycle = cycle + 1;                        
                        sum_strength = sum_strength + check_register(&cycle, &register);
                        register = register + arg;
                    },
                    Instruction::Noop => {
                        sum_strength = sum_strength + check_register(&cycle, &register);
                    }
                }
                cycle = cycle + 1;
    
            }
        }
    }
    return sum_strength;
}
/* TESTS */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let file_path = "./src/test1.txt".to_string();
        let sum: i32 = process_instructions(&file_path);
        assert_eq!(sum, 13140) ;
    }
    
    #[test]
    fn t3() {
        let file_path = "./src/test3.txt".to_string();
        let sum: i32 = process_instructions(&file_path);
        assert_eq!(sum, 13140) ;
    }
}   