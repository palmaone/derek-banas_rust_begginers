#![allow(unused)]

use std::io::{self, Read};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    /*
     * shadowing 
     * using 2 or more variables with same name
     *  but with different data types 
     */
    let age = "47";
    let mut age: u32 = age.trim().parse()
    .expect("Age wasn't assigned a number");

    age += 1;

    println!("I'm {} and I want ${}", age, ONE_MIL);
}
