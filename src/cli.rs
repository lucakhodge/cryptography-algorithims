use std::io;
use crate::euclidean;
use crate::chinese;

pub fn use_menu() {
    loop {
        let mut input = String::new();
        println!("Select one of the following (or quit)");
        println!("1 - GCD (Euclidean)");
        println!("2 - GCD and the coefficients (Extended Euclidean)");
        println!("3 - Chinese Remainder Theorem");
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() as &str {
            "1" => {use_euclidean();},
            "2" => {use_extended_euclidean();},
            "3" => {use_chinese();},
            "quit" | "q" => {return;},
            _ => {println!("Invalid input");},
        };
        println!();
    }
}

fn use_euclidean() {
    println!("Enter your first integer");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    println!("Enter your second integer");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    match (input1.trim().parse::<i128>(), input2.trim().parse::<i128>()) {
        (Ok(x1), Ok(x2)) => {
            println!("The GCD of {} and {} is {}", x1, x2, euclidean::euclidean_algorithim(x1, x2));
        },
        _ => {println!("Invalid input");},
    };
}
fn use_extended_euclidean() {
    println!("Enter your first integer");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    println!("Enter your second integer");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    match (input1.trim().parse::<i128>(), input2.trim().parse::<i128>()) {
        (Ok(x1), Ok(x2)) => {
            let (gcd, b1, b2) = euclidean::extended_euclidean_algorithim(x1, x2);
            println!("The GCD of {} and {} is {}", x1, x2, euclidean::euclidean_algorithim(x1, x2));
            println!("({}) * ({}) + ({}) * ({}) = {} ", x1, b1, x2, b2, gcd);
        },
        _ => {println!("Invalid input");},
    };
}

fn use_chinese() {
    println!("Enter two integers seperate by a space to be the value and modulus.");
    println!("Enter as many with coprime modulus as you want, then enter nothing to coninue.");
    let mut values : Vec<(i128, i128)> = vec!();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "" => {
                let (num, modulus) = chinese::chinese_remainder_list(values);
                println!("The value is: {} mod {}", num, modulus);
                return;
            },
            s => {
                let words : Vec<&str> = s.split(" ").collect();
                if words.len() != 2 {
                    println!("Invalid input: you must enter two numbers");
                }
                match (words[0].parse::<i128>(), words[1].parse::<i128>()){
                    (Ok(x1), Ok(x2)) => {
                        values.push((x1 , x2));
                        println!("Added {} mod {}", x1, x2);
                    },
                    _ => {
                        println!("Invalid input: you must enter two numbers");
                    }, 
                }
            },
        }
    }
}
