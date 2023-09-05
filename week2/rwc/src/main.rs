use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead};
use std::process; // For read_file_lines()

//borrow some code from rdiff. Thanks!
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut res: Vec<String> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        res.push(line_str);
    }
    // println!("{:?}", res);
    Ok(res)
}
// run cargo run handout-b.txt  to start the program.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file = read_file_lines(filename).unwrap();
    let mut num_word: usize = 0;
    let mut num_line: usize = file.len();
    let mut num_ch: usize = 0;
    let space: char = ' ';
    for i in 0..file.len() {
        num_word += 1;
        for j in 0..file[i].len() {
            if (&file[i]).chars().nth(j).unwrap() == space {
                num_word += 1;
            } else {
                num_ch += 1;
            }
        }
    }
    println!(
        "number of lines: {}, number of words: {}, number of characters:{}.",
        num_line, num_word, num_ch
    );
}
