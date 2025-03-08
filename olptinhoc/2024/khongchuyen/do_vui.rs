// Solution for https://oj.vnoi.info/problem/olp_kc24_string

/*
    Explanation:
    - If the length is even, there is no solution
    - If the length is odd, we can split the string into two parts: s1 and s2, where s1 is the first half of the string and s2 is the second half of the string (excluding the middle character)
    - We can then concatenate(duplicate) s1 with itself and s2 with itself to form t1 and t2
    - If t1 is a subsequence of z, then s1 is the original string
    - If t2 is a subsequence of z, then s2 is the original string
    - If both t1 and t2 are subsequences of z, then there are multiple solutions (unless s1 == s2)
    - If neither t1 nor t2 is a subsequence of z, then there is no solution
*/
use std::{io, usize};

fn _input() -> (usize, Vec<char>) {
    let mut input = String::new();

    // Read n
    io::stdin().read_line(&mut input).expect("Failed to read n");
    let n: usize = input.trim().parse().expect("Failed to parse n");
    
    // Read z
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read z");
    let z: Vec<char> = input.trim().chars().collect();

    (n, z)
}

// Check if s2 is a subsequence of s1
fn is_sub_seq(s1: &[char], s2: &[char]) -> bool {
    let mut p1 = 0;
    let mut p2 = 0;
    let l1 = s1.len();
    let l2 = s2.len();

    // If s2 is longer than s1, return false
    if l2 > l1 {
        return false;
    }

    // Navigating in s2
    while p2 < l2 {
        // Navigating in s1 until we find a match or reach the end
        while p1 < l1 && s1[p1] != s2[p2] {
            p1 += 1;
        }

        // If we reach the end of s1, return false
        if p1 >= l1 {
            return false;
        }

        // Move to the next character in s1 and s2
        p1 += 1;
        p2 += 1;
    }

    p2 == l2
}

fn original_string(length: usize, z: Vec<char>) -> (String, String) {
    let mut s1 = String::new();
    let mut s2 = String::new();

    for i in 0..length/2 {
        s1.push(z[i]);
    }
    for i in length/2+1..length {
        s2.push(z[i]);
    }

    let t1: Vec<char> = s1.chars().chain(s1.chars()).collect();
    let t2: Vec<char> = s2.chars().chain(s2.chars()).collect();

    if !is_sub_seq(&z, &t1) {
        s1.clear();
    }
    if !is_sub_seq(&z, &t2) {
        s2.clear();
    }

    (s1, s2)
}

// Main function
fn do_vui(n: usize, z: Vec<char>) {
    if n % 2 == 0 {
        println!("No Solution");
        return;
    }

    let (s1, s2) = original_string(n, z);

    if s1.is_empty() && s2.is_empty() {
        println!("No Solution");
        return;
    }
    
    if !s1.is_empty() && !s2.is_empty() {
        if s1 == s2 {
            println!("{}", s1);
            return;
        }
        println!("Multiple Solutions");
        return;
    }

    if !s1.is_empty() {
        println!("{}", s1);
    } else {
        println!("{}", s2);
    }
}

fn main() {
    let (n, z): (usize, Vec<char>) = _input();
    do_vui(n, z);
}
