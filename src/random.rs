use rand::{thread_rng, Rng};

pub fn random_range(min: usize, max:usize) -> usize {
    thread_rng().gen_range(min..max)
}

// make a test for random range
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn random_range_test() {
        //import my funciton        // loop that executes 1000 times
        for _ in 0..1000 {
            // generate random number between 0 and 10
            let random_number = random_range(0, 10);
            // assert that the number is less than 10
            assert!(random_number <= 10);
        }
    }
}