#![allow(unused)]

use std::io::{self, Read};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    //Unsigned integers: u8, u16, u32, u64, u128, usize
    //Signed integers: i8, i16, i32, i64, i128, isize
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max f32 : {}", f32::MAX);

    let _is_true = true; //allow unused by prepending an underscore
    let is_true = true;
    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    println!("f32:{}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f64:{}", num_2 + 0.111111111111111);
    
}
