use crate::error::{EnigmaError, ErrorKind};

pub fn get_position_from_char(a: char) -> Result<usize, EnigmaError> {
    if !a.is_ascii_uppercase() {
        return Err(EnigmaError::new(ErrorKind::InputError(), format!("Expected uppercase ASCII! Got {}", a)));
    }

    let mut position = 0;

    for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        if a == c {
            return Ok(position);
        }

        position += 1;
    }

    Err(EnigmaError::new(ErrorKind::InputError(), format!("Unable to convert char {} into a position!", a)))
}

pub fn get_char_from_position(position: usize) -> Result<char, EnigmaError> {
    if position > 25 {
        return Err(EnigmaError::new(ErrorKind::InputError(), format!("Expected position in the range of 0 to 25! Got {}", position)));
    }

    if let Some(c) = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(position) {
        return Ok(c);
    }

    Err(EnigmaError::new(ErrorKind::InputError(), format!("Unable to convert position {} into a char!", position)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_position_from_char() {
        assert_eq!(0, get_position_from_char('A').unwrap());
        assert_eq!(1, get_position_from_char('B').unwrap());
        assert_eq!(2, get_position_from_char('C').unwrap());
        assert_eq!(3, get_position_from_char('D').unwrap());
        assert_eq!(4, get_position_from_char('E').unwrap());
        assert_eq!(5, get_position_from_char('F').unwrap());
        assert_eq!(6, get_position_from_char('G').unwrap());
        assert_eq!(7, get_position_from_char('H').unwrap());
        assert_eq!(8, get_position_from_char('I').unwrap());
        assert_eq!(9, get_position_from_char('J').unwrap());
        assert_eq!(10, get_position_from_char('K').unwrap());
        assert_eq!(11, get_position_from_char('L').unwrap());
        assert_eq!(12, get_position_from_char('M').unwrap());
        assert_eq!(13, get_position_from_char('N').unwrap());
        assert_eq!(14, get_position_from_char('O').unwrap());
        assert_eq!(15, get_position_from_char('P').unwrap());
        assert_eq!(16, get_position_from_char('Q').unwrap());
        assert_eq!(17, get_position_from_char('R').unwrap());
        assert_eq!(18, get_position_from_char('S').unwrap());
        assert_eq!(19, get_position_from_char('T').unwrap());
        assert_eq!(20, get_position_from_char('U').unwrap());
        assert_eq!(21, get_position_from_char('V').unwrap());
        assert_eq!(22, get_position_from_char('W').unwrap());
        assert_eq!(23, get_position_from_char('X').unwrap());
        assert_eq!(24, get_position_from_char('Y').unwrap());
        assert_eq!(25, get_position_from_char('Z').unwrap());
    }

    #[test]
    #[should_panic]
    fn test_get_position_from_char_lowercase() {
        get_position_from_char('a').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_position_from_char_not_char() {
        get_position_from_char('5').unwrap();
    }

    #[test]
    fn test_get_char_from_position() {
        assert_eq!('A', get_char_from_position(0).unwrap());
        assert_eq!('B', get_char_from_position(1).unwrap());
        assert_eq!('C', get_char_from_position(2).unwrap());
        assert_eq!('D', get_char_from_position(3).unwrap());
        assert_eq!('E', get_char_from_position(4).unwrap());
        assert_eq!('F', get_char_from_position(5).unwrap());
        assert_eq!('G', get_char_from_position(6).unwrap());
        assert_eq!('H', get_char_from_position(7).unwrap());
        assert_eq!('I', get_char_from_position(8).unwrap());
        assert_eq!('J', get_char_from_position(9).unwrap());
        assert_eq!('K', get_char_from_position(10).unwrap());
        assert_eq!('L', get_char_from_position(11).unwrap());
        assert_eq!('M', get_char_from_position(12).unwrap());
        assert_eq!('N', get_char_from_position(13).unwrap());
        assert_eq!('O', get_char_from_position(14).unwrap());
        assert_eq!('P', get_char_from_position(15).unwrap());
        assert_eq!('Q', get_char_from_position(16).unwrap());
        assert_eq!('R', get_char_from_position(17).unwrap());
        assert_eq!('S', get_char_from_position(18).unwrap());
        assert_eq!('T', get_char_from_position(19).unwrap());
        assert_eq!('U', get_char_from_position(20).unwrap());
        assert_eq!('V', get_char_from_position(21).unwrap());
        assert_eq!('W', get_char_from_position(22).unwrap());
        assert_eq!('X', get_char_from_position(23).unwrap());
        assert_eq!('Y', get_char_from_position(24).unwrap());
        assert_eq!('Z', get_char_from_position(25).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_get_char_from_position_out_of_range() {
        get_char_from_position(26).unwrap();
    }
}