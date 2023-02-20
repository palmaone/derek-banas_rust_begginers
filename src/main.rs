#![allow(unused)]

use std::io::{self, Read};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);

    let age: i32 = 8;
    if (age >= 1) && (age <=18) {
        println!("Impórtant Bday!");
    } else if (age == 21) || (age == 50) {
        println!("Impórtant Bday!");
    } else if age >= 65 {
        println!("Impórtant Bday!");
    }else{
        println!("NOT AN Impórtant Bday!");
    }
    
}
