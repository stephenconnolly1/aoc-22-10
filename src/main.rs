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
    let mut distance: i32;
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
fn check_register(cycle:  &i32, register: &i32) -> bool {
    let pixel: bool =   (cycle - 1) % 40 == *register - 1 || 
                        (cycle - 1) % 40 == *register ||
                        (cycle - 1) % 40 == *register + 1;
    // println!("Cycle: {}, Register: {}, Pixel: {}", cycle, register, pixel);

    return pixel;
}

fn process_instructions(file_path: &String) -> Vec<bool>{
    let mut cycle: i32 = 1;
    let mut register: i32 = 1;
    // let mut sum_strength: i32 = 0;
    let mut crt: Vec<bool> = Vec::new();
    let mut pixel : bool;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {
                // println! ("{:?}", l);
                let i: Instruction = parse_line(&l);
                match i {
                    Instruction::Addx(arg) => {
                        pixel = check_register(&cycle, &register);
                        crt.push (pixel);

                        cycle = cycle + 1;            
                        // Check _during_ the cycle, i.e. before the operation is committed             
                        pixel = check_register(&cycle, &register);
                        crt.push (pixel);
                        register = register + arg;
                    },
                    Instruction::Noop => {
                        pixel = check_register(&cycle, &register);
                        crt.push (pixel);
                    }
                }
                cycle = cycle + 1;    
            }
        }
    }
    return crt;
}
fn print_crt(crt: &Vec<bool>) {
    let mut c: i32 = 0;
    for p in crt.iter() {
        if *p == true {
            print! ("#");
        } else {
            print!(".");
        }
        c = c + 1;
        if c % 40 == 0 { println!("");} 

    }
}
/* TESTS */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let file_path = "./src/test3.txt".to_string();
        let crt: Vec<bool> = process_instructions(&file_path);
        print_crt(&crt);
        assert_eq!(true,true) ;
    }
}   