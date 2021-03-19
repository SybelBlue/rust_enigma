pub mod rotor;

#[cfg(test)]
mod test {
    use crate::rotor::{Rotor, RotorSeq, std_set};

    #[test]
    fn valid_i() {
        assert_eq!(Rotor::valid_i(-2), 24);
        assert_eq!(Rotor::valid_i(2), 2);
        assert_eq!(Rotor::valid_i(28), 2);
    }

    #[test]
    fn test_rotor() {
        let data = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut r = Rotor::new(data, "Test r");
        let mut s = Rotor::new(data, "Test s");
        s.advance();

        for (i , c) in data.char_indices() {
            assert_eq!(c, r.encode(c));
            assert_eq!(((i + 1) as u8 % 26 + 65) as char, s.encode(c));
        }

        for (i, c) in data.char_indices() {
            assert_eq!(((c as u8 + i as u8 - 65) % 26 + 65) as char, r.encode(c));
            r.advance();
        }
    }

    #[test]
    fn test_valid_m3() {
        std_set();
    }

    #[test]
    fn test_rotor_seq() {
        let mut rs = RotorSeq::new_m3(0, 1, 2);
        assert_eq!(String::from("AAAAA").chars().map(|c| rs.encode(c)).collect::<String>(), String::from("BDZGO"));
    }
}
