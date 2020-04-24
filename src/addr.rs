use std::fmt;
use std::ops::{Add, Sub};

#[derive(PartialEq)]
pub struct Addr(pub u64);

impl Add for Addr {
    type Output = Addr;

    fn add(self, other: Addr) -> Addr {
        Addr(self.0 + other.0)
    }
}

impl Sub for Addr {
    type Output = Addr;

    fn sub(self, other: Addr) -> Addr {
        Addr(self.0 - other.0)
    }
}

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Addr({:X})", &self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::Addr;

    #[test]
    fn adds() {
        assert_eq!(Addr(0x1000) + Addr(0x1000), Addr(0x2000));
    }

    #[test]
    fn subtracts() {
        assert_eq!(Addr(0x2000) - Addr(0x1000), Addr(0x1000));
    }
}
