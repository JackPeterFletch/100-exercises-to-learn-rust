// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.


pub fn fibonacci(n: u32) -> u32 {

    let mut fib_seq: Vec<u32> = vec![];

    for i in 0..=n {

        if i == 0 {
            fib_seq.push(0)
        } else if i == 1 {
            fib_seq.push(1)
        } else {
            fib_seq.push(
                fib_seq[fib_seq.len()-1] + fib_seq[fib_seq.len()-2]
            )
        }
    }

    *fib_seq.last().unwrap()
}



#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
