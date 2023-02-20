#![allow(unused)]

use std::io::{self, Read};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);

    let mut my_age = 47;
    
    let can_vote = if my_age >= 18 {
        true //return true
    } else { false };

    println!("Can vote:{}", can_vote)
    
}
