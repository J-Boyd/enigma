use crate::utils;
use crate::error::{Error};
use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RotorType {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}

enum RotorDirection {
    LEFT,
    RIGHT,
}

pub struct Rotor {
    position: usize,
    ring_setting: usize,
    turnover: Vec<usize>,
    wiring: [usize; 26],
    wiring_inverse: [usize; 26],
}

impl Rotor {
    pub fn new(rotor_type: RotorType, key: char, ring_setting: usize) -> Result<Rotor> {
        let wiring = Rotor::get_rotor_wiring(&rotor_type);
        let wiring_inverse = Rotor::get_rotor_wiring_inverse(&wiring);

        if ring_setting < 1 || ring_setting > 26 {
            return Err(Error::RotorError).with_context(|| { format!("Invalid ring setting {}. Must be in the range 1 to 26 (inclusive).", ring_setting) });
        }

        let rotor = Rotor {
            position: utils::get_position_from_char(key)?,
            ring_setting: ring_setting - 1,
            turnover: match rotor_type {
                RotorType::I => vec![17], // R
                RotorType::II => vec![5], // F
                RotorType::III => vec![22], // W
                RotorType::IV => vec![10], // K
                RotorType::V => vec![0], // A
                RotorType::VI => vec![0, 13], // A, N
                RotorType::VII => vec![0, 13], // A, N
                RotorType::VIII => vec![0, 13] // A, N
            },
            wiring: wiring,
            wiring_inverse: wiring_inverse,
        };

        Ok(rotor)
    }

    pub fn get_key(&self) -> char {
        utils::get_char_from_position(self.position).unwrap()
    }

    fn get_rotor_wiring(rotor_type: &RotorType) -> [usize; 26] {
        match rotor_type {
            RotorType::I => return [4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9],
            RotorType::II => return [0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4],
            RotorType::III => return [1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18, 16, 14],
            RotorType::IV => return [4, 18, 14, 21, 15, 25, 9, 0, 24, 16, 20, 8, 17, 7, 23, 11, 13, 5, 19, 6, 10, 3, 2, 12, 22, 1],
            RotorType::V => return [21, 25, 1, 17, 6, 8, 19, 24, 20, 15, 18, 3, 13, 7, 11, 23, 0, 22, 12, 9, 16, 14, 5, 4, 2, 10],
            RotorType::VI => return [9, 15, 6, 21, 14, 20, 12, 5, 24, 16, 1, 4, 13, 7, 25, 17, 3, 10, 0, 18, 23, 11, 8, 2, 19, 22],
            RotorType::VII => return [13, 25, 9, 7, 6, 17, 2, 23, 12, 24, 18, 22, 1, 14, 20, 5, 0, 8, 21, 11, 15, 4, 10, 16, 3, 19],
            RotorType::VIII => return [5, 10, 16, 7, 19, 11, 23, 14, 2, 1, 9, 18, 15, 3, 25, 17, 0, 12, 4, 22, 13, 8, 20, 24, 6, 21],
        };
    }

    fn get_rotor_wiring_inverse(wiring: &[usize]) -> [usize; 26] {
        let mut wiring_inverse = [0; 26];

        for i in 0..26 {
            for j in 0..26 {
                if wiring[j] == i {
                    wiring_inverse[i] = j;
                    break;
                }
            }
        }

        wiring_inverse
    }

    pub fn get_rotor_type_from_string(rotor_type: &str) -> Result<RotorType> {
        let lower = rotor_type.to_ascii_lowercase();

        let t = match lower.as_str() {
            "i" => RotorType::I,
            "ii" => RotorType::II,
            "iii" => RotorType::III,
            "iv" => RotorType::IV,
            "v" => RotorType::V,
            "vi" => RotorType::VI,
            "vii" => RotorType::VII,
            "viii" => RotorType::VIII,
            _ => return Err(Error::RotorError).with_context(|| { format!("Invalid rotor type {}.", rotor_type) }),
        };

        Ok(t)
    }

    // Step the rotor and return true if the rotor to the left should also be stepped.
    pub fn step(&mut self) -> bool {
        self.position += 1;

        if self.position > 25 {
            self.position = 0;
        }

        if self.turnover.contains(&self.position) {
            return true;
        }

        false
    }

    pub fn scramble_left(&self, input: usize) -> usize {
        return self.scramble(input, RotorDirection::LEFT);
    }

    pub fn scramble_right(&self, input: usize) -> usize {
        return self.scramble(input, RotorDirection::RIGHT);
    }

    fn scramble(&self, input: usize, direction: RotorDirection) -> usize {
        let mut offset = self.position;

        if offset < self.ring_setting {
            offset += 26 - self.ring_setting;
        }
        else {
            offset -= self.ring_setting;
        }

        let mut pos = input + offset;

        if pos > 25 {
            pos -= 26;
        }

        match direction {
            RotorDirection::LEFT => {
                if self.wiring[pos] < offset {
                    return self.wiring[pos] + 26 - offset;
                }
                else {
                    return self.wiring[pos] - offset;
                }
            },
            RotorDirection::RIGHT => {
                if self.wiring_inverse[pos] < offset {
                    return self.wiring_inverse[pos] + 26 - offset;
                }
                else {
                    return self.wiring_inverse[pos] - offset;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotor_step_turnover() {
        let mut rotor = Rotor::new(RotorType::I, 'P', 1).unwrap();
        assert!(!rotor.step()); // Q - No turnover.
        assert!(rotor.step()); // R - Turnover.
        assert!(!rotor.step()); // S - No turnover.

        let mut rotor = Rotor::new(RotorType::II, 'D', 1).unwrap();
        assert!(!rotor.step()); // E - No turnover.
        assert!(rotor.step()); // F - Turnover.
        assert!(!rotor.step()); // G - No turnover.

        let mut rotor = Rotor::new(RotorType::III, 'U', 1).unwrap();
        assert!(!rotor.step()); // V - No turnover.
        assert!(rotor.step()); // W - Turnover.
        assert!(!rotor.step()); // X - No turnover.

        let mut rotor = Rotor::new(RotorType::IV, 'I', 1).unwrap();
        assert!(!rotor.step()); // J - No turnover.
        assert!(rotor.step()); // K - Turnover.
        assert!(!rotor.step()); // L - No turnover.

        let mut rotor = Rotor::new(RotorType::V, 'Y', 1).unwrap();
        assert!(!rotor.step()); // Z - No turnover.
        assert!(rotor.step()); // A - Turnover.
        assert!(!rotor.step()); // B - No turnover.

        for rotor_type in vec!(RotorType::VI, RotorType::VII, RotorType::VIII) {
            let mut rotor = Rotor::new(rotor_type, 'Y', 1).unwrap();
            assert!(!rotor.step()); // Z - No turnover.
            assert!(rotor.step()); // A - Turnover.
            assert!(!rotor.step()); // B - No turnover.

            let mut rotor = Rotor::new(rotor_type, 'L', 1).unwrap();
            assert!(!rotor.step()); // M - No turnover.
            assert!(rotor.step()); // N - Turnover.
            assert!(!rotor.step()); // O - No turnover.
        }
    }

    #[test]
    fn test_rotor_scramble_left() {
        let rotor = Rotor::new(RotorType::I, 'A', 1).unwrap();
        assert_eq!(4, rotor.scramble_left(0)); // A -> E
    }

    #[test]
    fn test_rotor_scramble_right() {
        let rotor = Rotor::new(RotorType::I, 'A', 1).unwrap();
        assert_eq!(0, rotor.scramble_right(4)); // E -> A
    }

    #[test]
    fn test_rotor_step_scramble_left() {
        let mut rotor = Rotor::new(RotorType::I, 'A', 1).unwrap();
        rotor.step();
        assert_eq!(9, rotor.scramble_left(0)); // A (B) -> K (J)
    }

    #[test]
    fn test_rotor_step_scramble_right() {
        let mut rotor = Rotor::new(RotorType::I, 'A', 1).unwrap();
        rotor.step(); 
        assert_eq!(2, rotor.scramble_right(4)); // E (F) -> D (C)
    }

    #[test]
    fn test_rotor_invalid_ring_setting_low() {
        assert!(Rotor::new(RotorType::I, 'A', 0).is_err());
    }

    #[test]
    fn test_rotor_invalid_ring_setting_high() {
        assert!(Rotor::new(RotorType::I, 'A', 27).is_err());
    }

    #[test]
    fn test_get_rotor_type_from_string() {
        assert_eq!(RotorType::I, Rotor::get_rotor_type_from_string("I").unwrap());
        assert_eq!(RotorType::I, Rotor::get_rotor_type_from_string("i").unwrap());

        assert_eq!(RotorType::II, Rotor::get_rotor_type_from_string("II").unwrap());
        assert_eq!(RotorType::II, Rotor::get_rotor_type_from_string("ii").unwrap());

        assert_eq!(RotorType::III, Rotor::get_rotor_type_from_string("III").unwrap());
        assert_eq!(RotorType::III, Rotor::get_rotor_type_from_string("iii").unwrap());

        assert_eq!(RotorType::IV, Rotor::get_rotor_type_from_string("IV").unwrap());
        assert_eq!(RotorType::IV, Rotor::get_rotor_type_from_string("iv").unwrap());

        assert_eq!(RotorType::V, Rotor::get_rotor_type_from_string("V").unwrap());
        assert_eq!(RotorType::V, Rotor::get_rotor_type_from_string("v").unwrap());

        assert_eq!(RotorType::VI, Rotor::get_rotor_type_from_string("VI").unwrap());
        assert_eq!(RotorType::VI, Rotor::get_rotor_type_from_string("vi").unwrap());

        assert_eq!(RotorType::VII, Rotor::get_rotor_type_from_string("VII").unwrap());
        assert_eq!(RotorType::VII, Rotor::get_rotor_type_from_string("vii").unwrap());

        assert_eq!(RotorType::VIII, Rotor::get_rotor_type_from_string("VIII").unwrap());
        assert_eq!(RotorType::VIII, Rotor::get_rotor_type_from_string("viii").unwrap());
    }

    #[test]
    fn test_get_rotor_type_from_string_invalid() {
        assert!(Rotor::get_rotor_type_from_string("blah").is_err());
    }
}