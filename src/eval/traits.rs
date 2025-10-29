use std::ops;

pub trait ParseNumber: Sized {
    fn parse_number(s: &str) -> Result<Self, String>;
}

impl ParseNumber for rug::Integer {
    fn parse_number(s: &str) -> Result<rug::Integer, String> {
        let result = match s.parse::<rug::Integer>() {
            Ok(num) => num,
            Err(_) => return Err(format!("Could not parse number: {}", s)),
        };

        Ok(result)
    }
}

impl ParseNumber for rug::Float {
    fn parse_number(s: &str) -> Result<rug::Float, String> {
        let parsed = rug::Float::parse(s);
        let f = rug::Float::with_val(53, parsed.unwrap());
        Ok(f)
    }
}

pub trait Number: ops::MulAssign + Clone + ParseNumber +
ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> { }

impl<T> Number for T where T: ops::MulAssign + Clone + ParseNumber +
ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> { }