
use std::io::{self, Write};
use std::collections::HashMap;

fn main() {

    let mut array: Vec<i32> = Vec::new();
    let mut count = HashMap::new();
    let mut maxcount = 0;
    let mut mostval = 0;
    let mut total = 0;

    loop {
        match get_number() {
            Some(num) => {
                let item = count.entry(num).or_insert(0);
                *item += 1;
                if *item > maxcount {
                    maxcount = *item;
                    mostval = num;
                }
                array.push(num);
                total += num;
            }
            None => break,
        }
    }

    if count.len() > 0 {
        array.sort();
        match array.get(array.len() / 2) {
            Some(val) => println!("\n\n中位数: {}", val),
            _ => (),
        }
        println!("平均数: {}", total / (count.len() as i32));
        println!("众数: {} 数量: {}", mostval, maxcount);
    }
}

fn get_number() -> Option<i32> {

    loop {
        print!("输入一个值: ");
        io::stdout().flush().unwrap();

        let mut instr = String::new();

        io::stdin().read_line(&mut instr).expect("读取行失败");

        match instr.trim().parse() {
            Ok(num) => return Some(num),
            Err(_) => {
                if instr.trim().len() == 0 {
                    return None;
                } else {
                    println!("错误的输入");
                    continue;
                }
            }
        }
    }
}
