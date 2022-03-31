/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::{collections::HashSet, hash::Hash};

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in v.iter() {
        result.push(i + n)
    };
    result
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    let length = v.len();
    for i in 0..length {
        v[i] = v[i] + n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut s:HashSet<i32> = HashSet::new();
    let length = v.len();
    let mut indexes: Vec<usize> = Vec::new();
    for i in 0..length {
        if (s.contains(&v[i])) {
            indexes.push(i);
        } else {
            s.insert(v[i]);
        }
    }
    for index in indexes.iter().rev() {
        v.remove(*index);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
