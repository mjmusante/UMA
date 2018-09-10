
pub struct GuessMeArray {
    guess_count: usize,
    over: bool,
    prior: [u16; 20],
    valid: [bool; 9000]
}

impl GuessMeArray {

    pub fn new() -> GuessMeArray {
        GuessMeArray {
            guess_count: 0,
            over: false,
            prior: [0; 20],
            valid: [true; 9000]
        }
    }

    pub fn num_matches(aref: &u16, bref: &u16) -> u16 {
        let mut retval = 0;
        let mut a = *aref;
        let mut b = *bref;

        while a > 0 && b > 0 {
            if a % 10 == b % 10 {
                retval += 1;
            }
            a /= 10;
            b /= 10;
        }

        retval
    }

    fn reset(&mut self) {
        self.guess_count = 0;
        self.over = false;
        self.prior = [0; 20];
        self.valid = [true; 9000];
    }

    pub fn num_guesses(&self) -> usize {
        self.guess_count
    }

    pub fn is_over(&self) -> bool {
        self.over
    }

    pub fn prior_guesses(&self) -> [u16; 20] {
        self.prior
    }

    pub fn get_guess(&mut self) -> u16 {
        let mut guessnow = 0;

        for i in 1000..9999 {
            if self.valid[i - 1000] {
                guessnow = i as u16;
                break;
            }
        }

        self.prior[self.guess_count] = guessnow;
        self.guess_count += 1;

        guessnow
    }

    pub fn update_guess(&mut self, guess: u16) -> bool {
        let x = self.prior[self.guess_count - 1];
        let mut result = false;
        
        for i in 1000..9999 {
            if self.valid[i - 1000] {
                if GuessMeArray::num_matches(&x, &(i as u16)) != guess {
                    self.valid[i - 1000] = false;
                } else {
                    result = true;
                }
            }
        }
        self.over = guess == 4;
        result
    }

    fn is_prior_guess(&self, guess: u16) -> bool {
        for i in 0..self.guess_count {
            if self.prior[i] == guess {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _g = GuessMeArray::new();
    }

    #[test]
    fn test_num_matches_trivial() {
        assert_eq!(0, GuessMeArray::num_matches(&1234, &4321));
        assert_eq!(0, GuessMeArray::num_matches(&1234, &2341));
    }

    #[test]
    fn test_num_matches() {
        assert_eq!(1, GuessMeArray::num_matches(&5678, &7777));
        assert_eq!(1, GuessMeArray::num_matches(&9000, &3210));
        assert_eq!(2, GuessMeArray::num_matches(&1234, &1114));
        assert_eq!(2, GuessMeArray::num_matches(&5005, &6006));
        assert_eq!(3, GuessMeArray::num_matches(&1000, &9000));
        assert_eq!(4, GuessMeArray::num_matches(&1234, &1234));
    }

    #[test]
    fn test_reset() {
        let mut g = GuessMeArray::new();
        let none = [0; 20];

        g.reset();
        assert_eq!(0, g.num_guesses());
        assert_eq!(false, g.is_over());
        assert_eq!(none, g.prior_guesses());
    }

    #[test]
    fn test_first_guess() {
        let mut g = GuessMeArray::new();
        assert_eq!(1000, g.get_guess());
    }

    #[test]
    fn test_is_over() {
        let gamer_a = GuessMeArray::new();
        let mut gamer_b = GuessMeArray::new();

        assert_eq!(false, gamer_b.is_over());
        assert_eq!(1000, gamer_b.get_guess());
        gamer_b.update_guess(4);
        assert_eq!(true, gamer_b.is_over());
        assert_eq!(false, gamer_a.is_over());
    }

    #[test]
    fn test_is_prior_guess() {
        let mut gamer_a = GuessMeArray::new();
        let g1 = gamer_a.get_guess();
        assert_eq!(true, gamer_a.is_prior_guess(g1));
        assert_eq!(false, gamer_a.is_prior_guess(9999));
        gamer_a.update_guess(0);
        let g2 = gamer_a.get_guess();
        assert_eq!(true, gamer_a.is_prior_guess(g2));
        assert_eq!(false, gamer_a.is_prior_guess(9999));
    }

    #[test]
    fn test_num_guesses() {
        let mut gamer_b = GuessMeArray::new();
        assert_eq!(0, gamer_b.num_guesses());
        assert_eq!(0, gamer_b.num_guesses());
        gamer_b.get_guess();
        assert_eq!(1, gamer_b.num_guesses());
        assert_eq!(1, gamer_b.num_guesses());
        gamer_b.update_guess(0);
        gamer_b.get_guess();
        assert_eq!(2, gamer_b.num_guesses());
        assert_eq!(2, gamer_b.num_guesses());
        gamer_b.update_guess(4);
        gamer_b.get_guess();
        assert_eq!(3, gamer_b.num_guesses());
        assert_eq!(3, gamer_b.num_guesses());
    }

    #[test]
    fn test_update_guess_trivial() {
        let mut gamer_b = GuessMeArray::new();
        assert_eq!(1000, gamer_b.get_guess());
        assert_eq!(true, gamer_b.update_guess(4));
        assert_eq!(true, gamer_b.is_over());
    }

    #[test]
    fn test_update_guess() {
        let mut gamer_a = GuessMeArray::new();
        assert_eq!(1000, gamer_a.get_guess());
        assert_eq!(true, gamer_a.update_guess(0));

        assert_eq!(2111, gamer_a.get_guess());
        assert_eq!(true, gamer_a.update_guess(0));

        assert_eq!(3222, gamer_a.get_guess());
        assert_eq!(true, gamer_a.update_guess(3));

        assert_eq!(3223, gamer_a.get_guess());
        assert_eq!(true, gamer_a.update_guess(2));

        assert_eq!(3232, gamer_a.get_guess());
        assert_eq!(true, gamer_a.update_guess(3));

        assert_eq!(3242, gamer_a.get_guess());
        assert_eq!(true, gamer_a.update_guess(4));
        assert_eq!(true, gamer_a.is_over());
    }

    #[test]
    fn test_update_guess_error() {
        let mut gamer_a = GuessMeArray::new();
        gamer_a.get_guess();
        gamer_a.update_guess(3);
        gamer_a.get_guess();
        assert_eq!(false, gamer_a.update_guess(1));

        gamer_a.reset();
        gamer_a.get_guess();
        gamer_a.update_guess(3);
        gamer_a.get_guess();
        gamer_a.update_guess(2);
        gamer_a.get_guess();
        assert_eq!(false, gamer_a.update_guess(1));
    }

    #[test]
    fn test_prior_guesses() {
        let mut a = [0; 20];
        let mut gamer_b = GuessMeArray::new();
        assert_eq!(a, gamer_b.prior_guesses());

        a[0] = gamer_b.get_guess();
        assert_eq!(a, gamer_b.prior_guesses());
        gamer_b.update_guess(1);

        a[1] = gamer_b.get_guess();
        gamer_b.update_guess(2);

        a[2] = gamer_b.get_guess();
        gamer_b.update_guess(3);

        a[3] = gamer_b.get_guess();
        assert_eq!(a, gamer_b.prior_guesses());
        gamer_b.update_guess(4);
        assert_eq!(a, gamer_b.prior_guesses());

    }
}
