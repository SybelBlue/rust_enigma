pub mod rotor;

#[cfg(test)]
mod test {
    use crate::rotor::Rotor;

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
}
