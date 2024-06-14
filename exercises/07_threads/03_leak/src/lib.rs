// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {

    let static_ref: &'static mut [i32] = v.leak();

    let (left, right) = static_ref.split_at(static_ref.len() / 2);

    let left_total = thread_sum(left);
    let right_total = thread_sum(right);

    left_total + right_total
}

pub fn thread_sum(v: &'static [i32]) -> i32 {
    let handle = thread::spawn(|| {
        v.iter().sum()
    });

    return handle.join().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
