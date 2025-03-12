use std::collections::HashMap;

pub fn fib(n: u128) -> Option<u128> {
    //  nothing to process if zero
    if n == 0 {
        return Some(0);
    }

    //  storing previously calculated results
    let mut memo: HashMap<u128, u128> = HashMap::<u128, u128>::new();

    for i in 1..n + 1 {
        memo.insert(
            i,
            if i <= 2 {
                1
            } else {
                let Some(a) = memo.get(&(i - 1)) else {
                    return None;
                };
                let Some(b) = memo.get(&(i - 2)) else {
                    return None;
                };
                a + b
            },
        );
    }

    if let Some(result) = memo.get(&n) {
        Some(*result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero() {
        let result = fib(0);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn returns_one() {
        let result = fib(1);
        assert_eq!(result, Some(1));

        let result = fib(2);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn returns_small() {
        let result = fib(6);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn returns_large() {
        let result = fib(180);
        assert_eq!(result, Some(18547707689471986212190138521399707760));
    }
}
