/// A struct that represents a single rotor in enigma
pub struct Rotor {
    arr: [char; 26],
    i: isize,
    pub name: String,
}

impl Rotor {
    pub fn new(data: &str, name: &str) -> Self {
        if data.len() != 26 {
            panic!("Unknown data format! Must be exactly 26 chars.")
        }
        let i = 0;
        let mut arr = ['A'; 26];
        for (i, c) in data.char_indices().take(26) {
            arr[i] = Rotor::validate_char(c);
        }
        let name = String::from(name);
        Self { arr, i, name }
    }

    pub fn peek_setting(&self) -> (char, char, char) {
        (self.get_ith(self.i - 1), self.get_ith(self.i), self.get_ith(self.i + 1))
    }

    pub fn encode(&self, c: char) -> char {
        let n = Rotor::validate_char(c) as u8 - 'A' as u8;
        self.get_ith(self.i + n as isize)
    }
    
    pub fn advance(&mut self) {
        self.i += 1;
        self.i %= 26;
    }

    fn validate_char(c: char) -> char {
        if !c.is_ascii_alphabetic() {
            panic!(format!("Bad char '{}', must be ascii alphabetic.", c))
        }
        c.to_ascii_uppercase()
    }

    fn get_ith(&self, i: isize) -> char {
        self.arr[Rotor::valid_i(i)]
    }

    fn valid_i(i: isize) -> usize {
        i.rem_euclid(26) as usize
    }
}
