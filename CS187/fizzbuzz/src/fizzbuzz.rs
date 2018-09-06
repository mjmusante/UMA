
struct FizzBuzz {
    fizz_number: u64,
    buzz_number: u64,
}

impl FizzBuzz {
    pub fn new(f: u64, b: u64) -> Self {
        FizzBuzz { fizz_number: f, buzz_number: b }
    }

    pub fn get_value(&self, val: u64) -> String {
        let f = (val % self.fizz_number == 0) ||
            FizzBuzz::has_digit(&val, &self.fizz_number);
        let b = (val % self.buzz_number == 0) ||
            FizzBuzz::has_digit(&val, &self.buzz_number);

        if f && b { return format!("fizzbuzz"); }

        if f { return format!("fizz"); }
        if b { return format!("buzz"); }

        format!("{}", val)
    }

    pub fn get_values(&self, low: &u64, high: &u64) -> Vec<String> {
        let mut v: Vec<String> = vec![];

        for i in *low..(*high + 1) {
            v.push(self.get_value(i));
        }

        v
    }

    fn has_digit(val: &u64, digit: &u64) -> bool {
        let mut v = *val;
        while v > 0 {
            if v % 10 == *digit { 
                return true;
            }
            v /= 10;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _r = FizzBuzz::new(3, 5);
    }

    #[test]
    fn test_not_fizz_or_buzz_trivial() {
        let three_four = FizzBuzz::new(3, 4);
        assert_eq!("1", three_four.get_value(1));
        assert_eq!("2", three_four.get_value(2));
    }

    #[test]
    fn test_fizz_or_buzz_trivial() {
        let three_four = FizzBuzz::new(3, 4);
        assert_eq!("fizz", three_four.get_value(3));
        assert_eq!("buzz", three_four.get_value(4));
    }

    #[test]
    fn test_not_fizz_or_buzz() {
        let three_four = FizzBuzz::new(3, 4);
        let two_five = FizzBuzz::new(2, 5);
        assert_eq!("59", three_four.get_value(59));
        assert_eq!("3", two_five.get_value(3));
        assert_eq!("143", two_five.get_value(143));
    }

    #[test]
    fn test_fizz_or_buzz() {
        let three_four = FizzBuzz::new(3, 4);
        assert_eq!("fizz", three_four.get_value(9));
        assert_eq!("buzz", three_four.get_value(8));
        assert_eq!("fizz", three_four.get_value(13));
        assert_eq!("buzz", three_four.get_value(116));
        assert_eq!("fizz", three_four.get_value(13125));

        let two_five = FizzBuzz::new(2, 5);
        assert_eq!("buzz", two_five.get_value(59));
        assert_eq!("fizz", two_five.get_value(29));
        assert_eq!("buzz", two_five.get_value(11511));
        assert_eq!("fizz", two_five.get_value(4342));
    }

    #[test]
    fn test_fizz_and_buzz() {
        let three_four = FizzBuzz::new(3, 4);
        let two_five = FizzBuzz::new(2, 5);

        assert_eq!("fizzbuzz", three_four.get_value(12));
        assert_eq!("fizzbuzz", three_four.get_value(12));
        assert_eq!("fizzbuzz", three_four.get_value(12));
        assert_eq!("fizzbuzz", two_five.get_value(100));
        assert_eq!("fizzbuzz", two_five.get_value(521));
        assert_eq!("fizzbuzz", three_four.get_value(12));
    }

    #[test]
    fn test_get_values_simple() {
        let three_four = FizzBuzz::new(3, 4);
        let a = three_four.get_values(&1, &5);
        let b = vec!["1".to_string(), "2".into(), "fizz".into(),
                     "buzz".into(), "5".into()];
        assert_eq!(a, b);
    }

    #[test]
    fn test_get_values_offset() {
        let three_four = FizzBuzz::new(3, 4);
        let a = three_four.get_values(&2, &6);
        let b = vec!["2".to_string(), "fizz".into(),
                     "buzz".into(), "5".into(), "fizz".into()];
        assert_eq!(a, b);
    }
}
