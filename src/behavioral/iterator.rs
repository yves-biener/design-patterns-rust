pub struct Fibonacci {
    curr: u32,
    next: u32,
}

// The iterator pattern is built into rust as a default trait
impl Iterator for Fibonacci {
    type Item = u32;

    // next can even contain buisness logic
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // this iterator always returns Some and no None (but this could be possible)
        Some(self.curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_iterator() {
        let expected = vec![3, 2, 1, 1];
        let fib = Fibonacci { curr: 0, next: 1 };
        let mut res: Vec<u32> = vec![];
        for i in fib.take(4) {
            res.insert(0, i);
        }
        assert_eq!(expected.len(), res.len());
        assert_eq!(expected, res);
    }
}
