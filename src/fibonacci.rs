use num::BigInt;

/// Struct for tracking and managing a Fibonacci sequence.
#[derive(Debug)]
pub struct Fibonacci {
    full: Vec<BigInt>,
    count: usize
}

impl Fibonacci {
    /// Returns an initialized instance of Fibonacci.
    pub fn new() -> Fibonacci {
        Fibonacci {
            full: vec!(BigInt::from(0)),
            count: 0
        }
    }

    /// Returns the previous item in the sequence and steps the sequence back so that a subsequent
    /// call to next will a give a repeat value.
    ///
    /// Example:
    ///
    /// ```
    /// let fibonacci = Fibonacci::new();
    /// fibonacci.next(); // 0
    /// fibonacci.next(); // 1
    /// fibonacci.next(); // 1
    /// fibonacci.next(); // 2
    /// fibonacci.previous(); // 1
    /// fibonacci.next(); // 2
    /// ```
    pub fn previous(&mut self) -> Option<BigInt> {
        // A desired side-effect of this is that running previous on a Fibonacci that has been
        // previoused to zero fully resets the next funtion to previous as well.
        if self.count == 0 {
            Some(BigInt::from(0))
        } else {
            self.count -= 1;
            match self.full.len() {
                // If there are 3 items in the full Vector we are either in the normal case or in a
                // [0,1,1] case.
                3 => {
                    // Safe to unwrap because we have already validated a length of 3.
                    match self.full.get(0).unwrap().to_string().as_str() {
                        // If we have 3 items and the one at index 0 is 0, we should have [0,1,1].
                        // This is a special case, as usual.
                        "0" => {
                            self.full = vec!(BigInt::from(0), BigInt::from(1));
                            // Because of the way the the initial numbers are nexted through, we
                            // could either be coming from count 2 to count 1 OR from count 3 to
                            // count 2 in this case. We should hanlde both for the user even if
                            // it's an inelegant edge case.
                            if self.count == 1 {
                                Some(BigInt::from(0))
                            } else {
                                Some(BigInt::from(1))
                            }
                        },
                        // If the item at index 0 is anything but 0, we are in a normal case.
                        _ => {
                            let target_last = self.full.get(1).unwrap().clone();
                            let target_middle = self.full.get(0).unwrap().clone();
                            // The first item in the new Vector is the actial previous value at index
                            // 1 less the further previous value at index 0. This approach saves the
                            // memory overhead of keeping the entire iteration history.
                            let target_first = &target_last - &target_middle;
                            let new_vec = vec!(target_first, target_middle, target_last);
                            self.full = new_vec;
                            match self.full.last() {
                                Some(val) => Some(val.clone()),
                                None => None
                            }
                        }
                    }
                },
                // If there are only 2 items in the full Vector we are in a [0,1] case, making the
                // desired previous value 0.
                2 => {
                    self.full.pop();
                    Some(BigInt::from(0))
                },
                // If there is only 1 item in the full Vector we are in a [0] case. We should never
                // go lower than this.
                1 => Some(BigInt::from(0)),
                _ => None
            }
        }
    }

    /// Returns the current fibonacci value without changing it. Generally, this is achieved by
    /// taking the last item in the `full` Vector, which contains the last 3 items in the sequence
    /// in most cases. See code for exceptions.
    pub fn current(&self) -> Option<BigInt> {
        // The first call to next adds a 1 to the sequence even though it returns 0 for the sake of
        // completeness. We have to manually ignore this.
        if self.full.len() == 2 {
            Some(BigInt::from(0))
        }
        // On the second iteration the Vector is full, but is still in the weird state caused by
        // the first few numbers. One last hard coded value.
        else if self.full.len() == 3 && self.count == 2 {
            Some(BigInt::from(1))
        }
        // After count: 2 we can use the last item in the full Vector.
        else {
            match self.full.last() {
                Some(val) => Some(val.clone()),
                None => Some(BigInt::from(0))
            }
        }
    }
}

impl Iterator for Fibonacci {
    type Item = BigInt;

    /// Return the next number in the Fibonacci sequence. This function has no call limit and will
    /// return accurate values as long as there are resources to hold them.
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        // Because we bootstrap with a single 0 in the Vector, we need to account for the first
        // couple of iterations where we need to fill it to our intended maximum. Remember that the
        // first values of the sequence are, irritatingly, [0, 1, 1, 2].
        if self.full.len() == 1 {
            self.full.push(BigInt::from(1));
            // Because of the initial divergence between count and the number of next calls, we could
            // either be coming from count 0 to count 1 OR from count 1 to count 2 in this case. We
            // should hanlde both for the user even if it's an inelegant edge case.
            if self.count == 1 {
                Some(BigInt::from(0))
            } else {
                Some(BigInt::from(1))
            }
        } else if self.full.len() == 2 {
            let next_val: BigInt = self.full.iter().sum();
            self.full.push(next_val.clone());
            Some(next_val)
        } else if self.full.len() == 3 && self.count == 3 {
            Some(BigInt::from(1))
        } else {
            // If we have a fully initialized set it is time to actually do the fibonacci math with
            // the contents of the Vector. Get the last 2 items in the Vector, sum them. Here we
            // have some additional guards for unexpected Vector and subslice states that handle
            // unlikely cases.
            let length = self.full.len();
            match self.full.get(length - 2..length) {
                Some(subslice) => {
                    let next_num: BigInt = subslice.iter().sum();
                    let mut new_vec = subslice.to_vec();
                    new_vec.push(next_num.clone());
                    self.full = new_vec;
                    Some(next_num)
                },
                None => Some(BigInt::from(0))
            }
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_first_10_nexts() {
        let mut fibonacci = Fibonacci::new();
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(0));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(2));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(3));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(5));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(8));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(13));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(21));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(34));
    }

    #[test]
    fn verify_1000th() {
        let mut fibonacci = Fibonacci::new();
        let mut count = 0;
        loop {
            count += 1;

            fibonacci.next();

            if count == 1000 {
                assert_eq!(
                    fibonacci.current()
                        .unwrap()
                        .clone()
                        .to_string(),
                        String::from("26863810024485359386146727202142923967616609318986952340123175997617981700247881689338369654483356564191827856161443356312976673642210350324634850410377680367334151172899169723197082763985615764450078474174626")
                );
                break;
            }
        }
    }

    #[test]
    fn verify_first_10_currents() {
        let mut fibonacci = Fibonacci::new();
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(0));
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(1));
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(1));
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(2));
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(3));
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(5));
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(8));
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(13));
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(21));
        fibonacci.next();
        assert_eq!(fibonacci.current().unwrap(), BigInt::from(34));
    }

    #[test]
    fn verify_first_10_previouses() {
        let mut fibonacci = Fibonacci::new();
        let mut count = 0;
        loop {
            count +=1;
            fibonacci.next();
            if count == 11 {break;}
        }
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(34));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(21));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(13));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(8));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(5));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(3));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(2));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(0));
        // Should never go below 0
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(0));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(0));
    }

    #[test]
    fn verify_early_forward_and_back() {
        let mut fibonacci = Fibonacci::new();
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(0));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(0));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(0));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.next().unwrap(), BigInt::from(2));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(1));
        assert_eq!(fibonacci.previous().unwrap(), BigInt::from(0));
    }
}
