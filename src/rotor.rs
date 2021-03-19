/// A struct that represents a single rotor in enigma
#[derive(Debug, Clone)]
pub struct Rotor {
    arr: [char; 26],
    i: isize,
    pub name: String,
}

pub type Reflector = Rotor;

impl Rotor {
    pub fn new(data: &str, name: &str) -> Self {
        if data.len() != 26 {
            panic!("Unknown data format! Must be exactly 26 chars.")
        }
        let i = 0;
        let mut arr = ['A'; 26];
        let mut tracker = [false; 26];
        for (i, c) in data.char_indices().take(26) {
            let c = Rotor::validate_char(c);
            arr[i] = c;
            let i = c as usize - 'A' as usize;
            if tracker[i] {
                panic!(format!("{} has already been mapped to!", c));
            }
            tracker[i] = true;
        }
        let name = String::from(name);
        Self { arr, i, name }
    }

    pub fn peek(&self) -> char {
        (65u8 + self.i as u8) as char
    }

    pub fn encode(&self, c: char) -> char {
        let n = Rotor::validate_char(c) as u8 - 'A' as u8;
        self.get_ith(self.i + n as isize)
    }
    
    pub fn advance(&mut self) -> bool {
        self.i += 1;
        if self.i == 26 {
            self.i = 0;
            true
        } else {
            false
        }
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

    pub(crate) fn valid_i(i: isize) -> usize {
        i.rem_euclid(26) as usize
    }
}

pub fn std_set() -> ([Rotor; 8], Reflector) {
    ( [ Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "I")
      , Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", "II")
      , Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", "III")
      , Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", "IV")
      , Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", "V")
      , Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", "VI")
      , Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", "VII")
      , Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", "VIII")
      ]
    , Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", "UKW-B")
    )
}

pub struct RotorSeq {
    rotors: [Rotor; 3],
    fourth: Option<Rotor>, 
    reflector: Reflector,
}

impl RotorSeq {
    pub fn new_m3(r1: usize, r2: usize, r3: usize) -> Self {
        let (m3, reflector) = std_set();
        Self { rotors: [m3[r1].clone(), m3[r2].clone(), m3[r3].clone()], fourth: None, reflector }
    }

    pub fn new_3(pos1: Rotor, pos2: Rotor, pos3: Rotor, reflector: Reflector) -> Self {
        Self { rotors: [pos1, pos2, pos3], fourth: None, reflector }
    }

    pub fn new_4(pos1: Rotor, pos2: Rotor, pos3: Rotor, pos4: Rotor, reflector: Reflector) -> Self {
        Self { rotors: [pos1, pos2, pos3], fourth: Some(pos4), reflector }
    }

    /// encodes `c` and advances the rotors
    pub fn encode(&mut self, c: char) -> char {
        print!("{:?} ", self.peeks());

        let mut c = c;
        print!("{}", c);

        for r in self.rotors.iter() {
            c = r.encode(c);
            print!("{}", c);
        }
        if let Some(r) = &self.fourth {
            c = r.encode(c);
            print!("{}", c);
        }

        c = self.reflector.encode(c);
        print!("{}", c);
        // don't ever advance the reflector!
        
        if let Some(r) = &mut self.fourth {
            c = r.encode(c);
            print!("{}", c);
        }
        for r in self.rotors.iter().rev() {
            c = r.encode(c);
            print!("{}", c);
        }

        if self.rotors[0].advance() {
            if self.rotors[1].advance() {
                if self.rotors[2].advance() {
                    if let Some(r) = &mut self.fourth {
                        r.advance();
                    }
                }
            }
        }

        println!(" {:?}", self.peeks());
        // println!("{:?}", self.rotors.iter().map(|r| r.name.clone()).collect::<Vec<String>>());
        c
    }

    /// returns the peek on all rotors in forward order
    pub fn peeks(&self) -> Vec<char> {
        self.rotors.iter().map(Rotor::peek).rev().collect()
    }
}