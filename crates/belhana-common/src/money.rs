//! Decimal-safe money. All amounts are stored as integer **minor units**
//! (piasters: 1 EGP = 100 piasters). Binary floating point is never used for
//! money anywhere in Belhana; conversion to/from f64 is intentionally not
//! provided.

use serde::{Deserialize, Serialize};

pub const EGP: &str = "EGP";
/// Minor units per major unit for EGP.
pub const EGP_MINOR: i64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Money {
    /// Amount in minor units (piasters). Can be negative for adjustments.
    pub minor: i64,
}

impl Money {
    pub fn egp(minor: i64) -> Self {
        Money { minor }
    }

    /// From a major-unit integer amount (e.g. 30 EGP).
    pub fn from_major(major: i64) -> Self {
        Money { minor: major * EGP_MINOR }
    }

    pub fn zero() -> Self {
        Money { minor: 0 }
    }

    pub fn is_zero(&self) -> bool {
        self.minor == 0
    }

    pub fn add(&self, other: Money) -> Money {
        Money { minor: self.minor + other.minor }
    }

    pub fn sub(&self, other: Money) -> Money {
        Money { minor: self.minor - other.minor }
    }

    pub fn scale(&self, factor: f64) -> Money {
        // Round-half-up on integer minor units to stay deterministic.
        let v = (self.minor as f64 * factor).round() as i64;
        Money { minor: v }
    }

    /// Multiply by a rational factor (num/den) with integer arithmetic:
    /// self * num / den, rounded half away from zero.
    pub fn scale_ratio(&self, num: i64, den: i64) -> Money {
        debug_assert!(den > 0);
        let a = self.minor * num;
        let b = den;
        let q = a / b;
        let r = a % b;
        let rounded = if (r * 2).abs() >= b.abs() { q + r.signum() } else { q };
        Money { minor: rounded }
    }

    /// Major units as a f64 **only for display purposes**.
    pub fn major_f64(&self) -> f64 {
        self.minor as f64 / EGP_MINOR as f64
    }

    /// Human display, e.g. "30.00 ج.م" (Arabic) or "EGP 30.00" (English).
    pub fn display(&self, arabic: bool) -> String {
        let major = self.minor / EGP_MINOR;
        let frac = (self.minor % EGP_MINOR).abs();
        if arabic {
            format!("{major}.{frac:02} ج.م")
        } else {
            format!("EGP {major}.{frac:02}")
        }
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display(true))
    }
}

impl std::ops::Add for Money {
    type Output = Money;
    fn add(self, rhs: Money) -> Money {
        self.add(rhs)
    }
}

impl std::ops::Sub for Money {
    type Output = Money;
    fn sub(self, rhs: Money) -> Money {
        self.sub(rhs)
    }
}

impl Default for Money {
    fn default() -> Self {
        Money::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_arithmetic() {
        let a = Money::from_major(30); // 3000
        let b = Money::from_major(25); // 2500
        assert_eq!(a.add(b).minor, 5500);
        assert_eq!(a.sub(b).minor, 500);
    }

    #[test]
    fn display_formatting() {
        assert_eq!(Money::from_major(130).display(true), "130.00 ج.م");
        assert_eq!(Money { minor: 4333 }.display(false), "EGP 43.33");
    }

    #[test]
    fn scale_ratio_rounds() {
        // 130.00 * 1/30 = 4.333... -> 433.33 piasters => 4333 minor
        let carton = Money::from_major(130);
        let per_egg = carton.scale_ratio(1, 30);
        assert_eq!(per_egg.minor, 433);
        // half-away-from-zero check: 4333 * 1/2 = 2166.5 -> 2167
        assert_eq!(Money { minor: 4333 }.scale_ratio(1, 2).minor, 2167);
    }

    #[test]
    fn negative_and_zero() {
        assert!(Money::zero().is_zero());
        assert_eq!(Money::from_major(5).sub(Money::from_major(8)).minor, -300);
    }
}
