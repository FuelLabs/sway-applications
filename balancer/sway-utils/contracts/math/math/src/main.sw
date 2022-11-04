library math;

use std::{
    revert::require,
};
// Todo when signed Integers are added
// pub fn abs(a: i64) -> u64{
//     if a > 0 {
//         a
//     }else {
//         a = a - 2*a;
//     }
//     a
// }

pub fn add(a: u64, b: u64) -> u64{
    let mut c: u64 = a + b;
    require(c >= a, "Error");
    c
}

pub fn sub(a: u64, b: u64) -> u64 {
    require(b <= a, "Error" );
    let mut c: u64 = b - a;
    c
}

pub fn max(a: u64, b: u64) -> u64 {
    let mut c: u64 = 0;
    if a >= b {
        c = a
    }else {
        c = b
    }
    c
}

pub fn min(a: u64, b: u64) -> u64 {
    let mut c: u64 = 0;
    if a < b {
        c = a
    }else {
        c = b
    }
    c
}

pub fn mul(a: u64, b: u64) -> u64 {
    let mut c: u64 = a * b;
    require (a == 0 || c / a == b, "Error");
    c
}

pub fn div_down(a: u64, b: u64) -> u64{
    require(b != 0, "Error");
    return a/b
}

pub fn div_up(a: u64, b: u64) -> u64 {
    require(b != 0, "Error"); 
    let mut c: u64 = 0;
    if a == 0 {
        c
    } else {
        c = 1 + (a-1)/b;
        c
    }

}

