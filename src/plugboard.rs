use crate::utils;
use crate::error::{EnigmaError, ErrorKind};

pub struct Plugboard {
    plugs: Vec<(usize, usize)>,
}

impl Plugboard {
    pub fn new(plugs: &[(char, char)]) -> Result<Plugboard, EnigmaError> {
        let mut plug_positions: Vec<(usize, usize)> = Vec::with_capacity(plugs.len());

        for plug in plugs.iter() {
            if plug.0 == plug.1 {
                return Err(EnigmaError::new(ErrorKind::PlugboardError(), format!("Cannot connect plug {} to {}!", plug.0, plug.1)));
            }

            let p = (utils::get_position_from_char(plug.0)?, utils::get_position_from_char(plug.1)?);

            for positions in plug_positions.iter() {
                if p.0 == positions.0 || p.0 == positions.1 {
                    return Err(EnigmaError::new(ErrorKind::PlugboardError(), format!("Cannot connect plug {}, already in use!", plug.0)));
                }

                if p.1 == positions.0 || p.1 == positions.1 {
                    return Err(EnigmaError::new(ErrorKind::PlugboardError(), format!("Cannot connect plug {}, already in use!", plug.1)));
                }
            }

            plug_positions.push(p);
        }

        let plugboard = Plugboard {
            plugs: plug_positions,
        };

        Ok(plugboard)
    }

    pub fn scramble(&self, input: usize) -> usize {
        for plug in self.plugs.iter() {
            if plug.0 == input {
                return plug.1;
            }

            if plug.1 == input {
                return plug.0;
            }
        }

        input
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plugboard_scramble() {
        let plugs = vec![('A', 'G'), ('D', 'M'), ('Y', 'S')];
        let plugboard = Plugboard::new(&plugs).unwrap();

        // Connected.
        assert_eq!(0, plugboard.scramble(6)); // A -> G
        assert_eq!(6, plugboard.scramble(0)); // G -> A
        assert_eq!(3, plugboard.scramble(12)); // D -> M
        assert_eq!(12, plugboard.scramble(3)); // M -> D
        assert_eq!(24, plugboard.scramble(18)); // Y -> S
        assert_eq!(18, plugboard.scramble(24)); // S -> Y

        // Not connected.
        assert_eq!(4, plugboard.scramble(4)); // E -> E
        assert_eq!(5, plugboard.scramble(5)); // F -> F
        assert_eq!(25, plugboard.scramble(25)); // Z -> Z
        assert_eq!(17, plugboard.scramble(17)); // R -> R
        assert_eq!(25, plugboard.scramble(25)); // Z -> Z
    }

    #[test]
    #[should_panic(expected = "Cannot connect plug A to A!")]
    fn test_plugboard_duplicate_plugs() {
        let plugs = vec![('A', 'A')];
        let _ = Plugboard::new(&plugs).unwrap();
    }

    #[test]
    #[should_panic(expected = "Cannot connect plug B, already in use!")]
    fn test_plugboard_plug_in_use() {
        let plugs = vec![('A', 'B'), ('B', 'C')];
        let _ = Plugboard::new(&plugs).unwrap();
    }
}