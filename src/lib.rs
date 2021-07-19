mod rotor;
mod reflector;
mod plugboard;
mod utils;
mod error;

pub use crate::rotor::Rotor;
pub use crate::rotor::RotorType;
pub use crate::reflector::Reflector;
pub use crate::reflector::ReflectorType;
pub use crate::plugboard::Plugboard;
pub use crate::error::*;

pub struct Enigma {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(reflector_type: ReflectorType, rotors: Vec<Rotor>, plugboard: Plugboard) -> Enigma {
        Enigma {
            rotors,
            reflector: Reflector::new(reflector_type),
            plugboard,
        }
    }

    pub fn encrypt(&mut self, input: &str) -> Result<String, EnigmaError> {
        if !input.is_ascii() {
            return Err(EnigmaError::new(ErrorKind::InputError(), String::from("Input is not ASCII")));
        }

        let mut result = String::with_capacity(input.len());

        for c in input.chars() {
            if c.is_ascii_whitespace() {
                result.push(c);
                continue
            }

            // Move the right hand rotor 1 position, and subsequent rotors if they are in the correct position.
            if self.rotors[2].step() {
                if self.rotors[1].step() {
                    self.rotors[0].step();
                }
            }

            let mut pos = utils::get_position_from_char(c)?;

            pos = self.plugboard.scramble(pos);
            pos = self.rotor_scramble(pos);
            pos = self.plugboard.scramble(pos);

            result.push(utils::get_char_from_position(pos)?);
        }

        Ok(result)
    }

    fn rotor_scramble(&mut self, input: usize) -> usize {
        let mut result = self.rotors[2].scramble_left(input);
        result = self.rotors[1].scramble_left(result);
        result = self.rotors[0].scramble_left(result);

        result = self.reflector.scramble(result);
    
        result = self.rotors[0].scramble_right(result);
        result = self.rotors[1].scramble_right(result);
        result = self.rotors[2].scramble_right(result);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rotor::Rotor;
    use crate::rotor::RotorType;

    #[test]
    fn test_enigma_encrypt() {
        let input = String::from("AAAAA");
        let expected = String::from("BDZGO");

        let mut rotors = Vec::new();

        rotors.push(Rotor::new(RotorType::I, 'A', 1).unwrap());
        rotors.push(Rotor::new(RotorType::II, 'A', 1).unwrap());
        rotors.push(Rotor::new(RotorType::III, 'A', 1).unwrap());

        let plugboard = Plugboard::new(&vec![]).unwrap();
        let mut enigma = Enigma::new(ReflectorType::B, rotors, plugboard);

        let output = enigma.encrypt(&input).expect("Failed to encrypt");

        assert_eq!(&expected, &output);
    }

    #[test]
    fn test_enigma_encrypt_ring_setting() {
        let input = String::from("AAAAA");
        let expected = String::from("EWTYX");

        let mut rotors = Vec::new();

        rotors.push(Rotor::new(RotorType::I, 'A', 2).unwrap());
        rotors.push(Rotor::new(RotorType::II, 'A', 2).unwrap());
        rotors.push(Rotor::new(RotorType::III, 'A', 2).unwrap());

        let plugboard = Plugboard::new(&vec![]).unwrap();
        let mut enigma = Enigma::new(ReflectorType::B, rotors, plugboard);

        let output = enigma.encrypt(&input).expect("Failed to encrypt");

        assert_eq!(&expected, &output);
    }

    #[test]
    fn test_enigma_encrypt_decrypt() {
        let input = String::from("AAAAA");
        let mut output;

        {
            let mut rotors = Vec::new();

            rotors.push(Rotor::new(RotorType::I, 'A', 1).unwrap());
            rotors.push(Rotor::new(RotorType::II, 'A', 1).unwrap());
            rotors.push(Rotor::new(RotorType::III, 'A', 1).unwrap());

            let plugboard = Plugboard::new(&vec![]).unwrap();
            let mut enigma = Enigma::new(ReflectorType::B, rotors, plugboard);

            output = enigma.encrypt(&input).expect("Failed to encrypt");
        }

        {
            let mut rotors = Vec::new();

            rotors.push(Rotor::new(RotorType::I, 'A', 1).unwrap());
            rotors.push(Rotor::new(RotorType::II, 'A', 1).unwrap());
            rotors.push(Rotor::new(RotorType::III, 'A', 1).unwrap());

            let plugboard = Plugboard::new(&vec![]).unwrap();
            let mut enigma = Enigma::new(ReflectorType::B, rotors, plugboard);

            output = enigma.encrypt(&output).expect("Failed to encrypt");
        }

        assert_eq!(&input, &output);
    }


    #[test]
    fn test_enigma_encrypt_decrypt_with_plugboard() {
        let input = String::from("AAAAA");
        let mut output;

        let plugs = vec![('A', 'Z'), ('B', 'Y'), ('C', 'X'), ('D', 'W'), ('E', 'V'), ('F', 'U'), ('G', 'T'), ('H', 'S'), ('I', 'R'), ('J', 'Q')];

        {
            let mut rotors = Vec::new();

            rotors.push(Rotor::new(RotorType::I, 'A', 1).unwrap());
            rotors.push(Rotor::new(RotorType::II, 'A', 1).unwrap());
            rotors.push(Rotor::new(RotorType::III, 'A', 1).unwrap());
            
            let plugboard = Plugboard::new(&plugs).unwrap();
            let mut enigma = Enigma::new(ReflectorType::B, rotors, plugboard);

            output = enigma.encrypt(&input).expect("Failed to encrypt");
        }

        {
            let mut rotors = Vec::new();

            rotors.push(Rotor::new(RotorType::I, 'A', 1).unwrap());
            rotors.push(Rotor::new(RotorType::II, 'A', 1).unwrap());
            rotors.push(Rotor::new(RotorType::III, 'A', 1).unwrap());

            let plugboard = Plugboard::new(&plugs).unwrap();
            let mut enigma = Enigma::new(ReflectorType::B, rotors, plugboard);

            output = enigma.encrypt(&output).expect("Failed to encrypt");
        }

        assert_eq!(&input, &output);
    }
}