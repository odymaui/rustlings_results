// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    /*
    let result;

    if num <= 1 {
        result = 1;
    } else {
        result = num * factorial(num - 1);
    }
    
    result
    */

    let mut amt = 1;
    for cnt in (1 ..= num) {
        amt = amt * cnt;
    }

    amt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_3() {
        assert_eq!(6, factorial(3));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn factorial_of_5() {
        assert_eq!(120, factorial(5));
    }
}