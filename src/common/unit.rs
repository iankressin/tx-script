use ethers::utils::{ConversionError, Units};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Unit {
    Gwei,
    Wei,
    Ether,
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Ether
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Gwei => String::from("gwei"),
            Unit::Wei => String::from("wei"),
            Unit::Ether => String::from("ether"),
        }
    }
}

impl TryInto<Units> for Unit {
    type Error = ConversionError;

    fn try_into(self) -> Result<Units, Self::Error> {
        match self {
            Unit::Gwei => Ok(Units::Gwei),
            Unit::Wei => Ok(Units::Wei),
            Unit::Ether => Ok(Units::Ether),
        }
    }
}
