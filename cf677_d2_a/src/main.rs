use std::{fs::read_to_string, io};

fn read_entry_file() -> (Vec<i32>, Vec<i32>){
    let entry: Vec<String> = match read_to_string("entry.txt") {
        Ok(x) => {
            x.lines()
                .map(String::from)
                .collect()
        },
        _ => std::process::exit(1)
    };

    let first_line: Vec<i32> = entry[0].split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let second_line: Vec<i32> = entry[1].split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    return (first_line, second_line);
}

fn read_cmd() -> (Vec<i32>, Vec<i32>) {
    let mut first_line = String::new();
    let mut second_line = String::new();  

    io::stdin()
        .read_line(&mut first_line)
        .unwrap();

    io::stdin()
        .read_line(&mut second_line)
        .unwrap();

    let first_line: Vec<i32> = first_line.split(" ") 
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let second_line: Vec<i32> = second_line.split(" ") 
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    return (first_line, second_line);
}

fn main() {
    let mut total_width: i32 = 0;
    // let (first_line, second_line) = read_entry_file();
    let (first_line, second_line) = read_cmd();

    for h in second_line.into_iter() {
        match h.cmp(&first_line[1]){
            std::cmp::Ordering::Greater => total_width +=2,
            _ => total_width += 1
        }
    }

    println!("{}", total_width);
}
