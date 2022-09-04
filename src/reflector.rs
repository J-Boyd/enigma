use crate::error::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ReflectorType {
    Beta,
    Gamma,
    A,
    B,
    C,
    ThinB,
    ThinC,
    ETW,
}

pub struct Reflector {
    wiring: [usize; 26]
}

impl Reflector {
    pub fn new(reflector_type: ReflectorType) -> Reflector {
        Reflector {
            wiring: Reflector::get_reflector_wiring(reflector_type),
        }
    }

    pub fn get_reflector_type_from_string(reflector_type: &str) -> Result<ReflectorType, Error> {
        let lower = reflector_type.to_ascii_lowercase();

        let t = match lower.as_str() {
            "beta" => ReflectorType::Beta,
            "gamma" => ReflectorType::Gamma,
            "a" => ReflectorType::A,
            "b" => ReflectorType::B,
            "c" => ReflectorType::C,
            "thinb" => ReflectorType::ThinB,
            "thinc" => ReflectorType::ThinC,
            "etw" => ReflectorType::ETW,
            _ => return Err(Error::ReflectorError), // format!("Couldn't convert {} to a reflector type!", reflector_type))),
        };

        Ok(t)
    }

    fn get_reflector_wiring(reflector_type: ReflectorType) -> [usize; 26] {
        match reflector_type {
            ReflectorType::Beta => return [11, 4, 24, 9, 21, 2, 13, 8, 23, 22, 15, 1, 16, 12, 3, 17, 19, 0, 10, 25, 6, 5, 20, 7, 14, 18],
            ReflectorType::Gamma => return [5, 18, 14, 10, 0, 13, 20, 4, 17, 7, 12, 1, 19, 8, 24, 2, 22, 11, 16, 15, 25, 23, 21, 6, 9, 3],
            ReflectorType::A => return [4, 9, 12, 25, 0, 11, 24, 23, 21, 1, 22, 5, 2, 17, 16, 20, 14, 13, 19, 18, 15, 8, 10, 7, 6, 3],
            ReflectorType::B => return [24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 0, 19],
            ReflectorType::C => return [5, 21, 15, 9, 8, 0, 14, 24, 4, 3, 17, 25, 23, 22, 6, 2, 19, 10, 20, 16, 18, 1, 13, 12, 7, 11],
            ReflectorType::ThinB => return [4, 13, 10, 16, 0, 20, 24, 22, 9, 8, 2, 14, 15, 1, 11, 12, 3, 23, 25, 21, 5, 19, 7, 17, 6, 18],
            ReflectorType::ThinC => return [17, 3, 14, 1, 9, 13, 19, 10, 21, 4, 7, 12, 11, 5, 2, 22, 25, 0, 23, 6, 24, 8, 15, 18, 20, 16],
            ReflectorType::ETW => return [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        }
    }

    pub fn scramble(&self, input: usize) -> usize {
        return self.wiring[input];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reflector_scramble() {
        let reflector = Reflector::new(ReflectorType::B);
    
        assert_eq!(24, reflector.scramble(0)); // A -> Y
        assert_eq!(0, reflector.scramble(24)); // Y -> A

        assert_eq!(7, reflector.scramble(3)); // D -> H
        assert_eq!(3, reflector.scramble(7)); // H -> D

        assert_eq!(14, reflector.scramble(12)); // M -> O
        assert_eq!(12, reflector.scramble(14)); // O -> M
    }

    #[test]
    fn test_get_reflector_type_from_string() {
        assert_eq!(ReflectorType::Beta, Reflector::get_reflector_type_from_string("BETA").unwrap());
        assert_eq!(ReflectorType::Beta, Reflector::get_reflector_type_from_string("beta").unwrap());

        assert_eq!(ReflectorType::Gamma, Reflector::get_reflector_type_from_string("GAMMA").unwrap());
        assert_eq!(ReflectorType::Gamma, Reflector::get_reflector_type_from_string("gamma").unwrap());

        assert_eq!(ReflectorType::A, Reflector::get_reflector_type_from_string("A").unwrap());
        assert_eq!(ReflectorType::A, Reflector::get_reflector_type_from_string("a").unwrap());

        assert_eq!(ReflectorType::B, Reflector::get_reflector_type_from_string("B").unwrap());
        assert_eq!(ReflectorType::B, Reflector::get_reflector_type_from_string("b").unwrap());

        assert_eq!(ReflectorType::C, Reflector::get_reflector_type_from_string("C").unwrap());
        assert_eq!(ReflectorType::C, Reflector::get_reflector_type_from_string("c").unwrap());

        assert_eq!(ReflectorType::ThinB, Reflector::get_reflector_type_from_string("THINB").unwrap());
        assert_eq!(ReflectorType::ThinB, Reflector::get_reflector_type_from_string("thinb").unwrap());

        assert_eq!(ReflectorType::ThinC, Reflector::get_reflector_type_from_string("THINC").unwrap());
        assert_eq!(ReflectorType::ThinC, Reflector::get_reflector_type_from_string("thinc").unwrap());

        assert_eq!(ReflectorType::ETW, Reflector::get_reflector_type_from_string("ETW").unwrap());
        assert_eq!(ReflectorType::ETW, Reflector::get_reflector_type_from_string("etw").unwrap());
    }

    #[test]
    fn test_get_reflector_type_from_string_invalid() {
        assert!(Reflector::get_reflector_type_from_string("blah").is_err());
    }
}