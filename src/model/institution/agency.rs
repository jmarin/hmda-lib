use std::fmt;

pub enum Agency {
    OCC,
    FRS,
    FDIC,
    NCUA,
    HUD,
    CFPB,
    Undetermined,
}

impl Agency {
    pub fn value_of(&self) -> i32 {
        match *self {
            Agency::OCC => 1,
            Agency::FRS => 2,
            Agency::FDIC => 3,
            Agency::NCUA => 5,
            Agency::HUD => 7,
            Agency::CFPB => 9,
            Agency::Undetermined => -1,
        }
    }
}

impl fmt::Display for Agency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Agency::value_of(&self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_of() {
        assert_eq!(Agency::OCC.value_of(), 1);
        assert_eq!(Agency::FRS.value_of(), 2);
        assert_eq!(Agency::FDIC.value_of(), 3);
        assert_eq!(Agency::NCUA.value_of(), 5);
        assert_eq!(Agency::HUD.value_of(), 7);
        assert_eq!(Agency::CFPB.value_of(), 9);
        assert_eq!(Agency::Undetermined.value_of(), -1);
    }
}
