use std::ops;

pub trait ParseNumber: Sized {
    fn parse_number(s: &str) -> Result<Self, String>;

    fn from_number(num: i32) -> Result<Self, String>;
}

impl ParseNumber for rug::Integer {
    fn parse_number(s: &str) -> Result<rug::Integer, String> {
        let result = match s.parse::<rug::Integer>() {
            Ok(num) => num,
            Err(_) => return Err(format!("Could not parse number: {}", s)),
        };

        Ok(result)
    }

    fn from_number(num: i32) -> Result<Self, String> {
        Ok(rug::Integer::from(num))
    }
}

impl ParseNumber for rug::Float {
    fn parse_number(s: &str) -> Result<rug::Float, String> {
        let parsed = rug::Float::parse(s);
        let f = rug::Float::with_val(53, parsed.unwrap());
        Ok(f)
    }

    fn from_number(num: i32) -> Result<Self, String> {
        Ok(rug::Float::with_val(53, num))
    }
}

// Big container of traits necessary for rug types to work
pub trait Number: ops::MulAssign + Clone + ParseNumber +
ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> { }

impl<T> Number for T where T: ops::MulAssign + Clone + ParseNumber +
ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> { }