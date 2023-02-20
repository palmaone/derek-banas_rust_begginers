#![allow(unused)]

use std::io::{self, Read};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    
    let age = 0;
    match age {
        1..=18 => println!("Important Bday!"),
        21 | 50 => println!("Important Bday!"),
        65..=i32::MAX => println!("Important Bday!"),
        _ => println!("NOT AN Important Bday!"),//Default Bday
    }
    
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {//.cmp is the compare function
        Ordering::Less => println!("NO, Can't vote!"),
        Ordering::Greater => println!("YES, Can vote!"),
        Ordering::Equal => println!("YES, Congrats NOW YOU Can vote!"),
    }
}
