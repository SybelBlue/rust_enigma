/// A struct that represents a single rotor in enigma
pub struct Rotor {
    arr: [char; 26],
    i: usize,
    name: String,
}

impl Rotor {
    pub fn new(data: &str, name: &str) -> Self {
        let i = 0;
        let arr = ['A'; 26];
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2, 1 + 1)
    }
}
