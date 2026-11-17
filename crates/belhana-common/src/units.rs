//! Measurement units and *valid* conversions.
//!
//! Principle: dimensional conversion is only applied where it is factually
//! valid. A plate/package/carton has no universal net quantity, so those units
//! are never converted to mass/volume automatically. Product-specific package
//! conversions (e.g. a 30-egg carton) are stored in the database
//! (`unit_conversions`) and applied explicitly.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    Kg,
    G,
    Liter,
    Ml,
    Piece,
    Dozen,
    Carton,
    Plate,
    Package,
    Bottle,
    Bundle,
    /// A unit the source mentioned that we cannot normalize (e.g. "طبق" with
    /// unknown net quantity is still `Plate`; unknown free text goes here).
    Custom(String),
}

impl Unit {
    pub fn code(&self) -> String {
        match self {
            Unit::Kg => "kg".into(),
            Unit::G => "g".into(),
            Unit::Liter => "l".into(),
            Unit::Ml => "ml".into(),
            Unit::Piece => "piece".into(),
            Unit::Dozen => "dozen".into(),
            Unit::Carton => "carton".into(),
            Unit::Plate => "plate".into(),
            Unit::Package => "package".into(),
            Unit::Bottle => "bottle".into(),
            Unit::Bundle => "bundle".into(),
            Unit::Custom(s) => format!("custom:{s}"),
        }
    }

    pub fn from_code(code: &str) -> Unit {
        match code {
            "kg" => Unit::Kg,
            "g" => Unit::G,
            "l" => Unit::Liter,
            "ml" => Unit::Ml,
            "piece" => Unit::Piece,
            "dozen" => Unit::Dozen,
            "carton" => Unit::Carton,
            "plate" => Unit::Plate,
            "package" => Unit::Package,
            "bottle" => Unit::Bottle,
            "bundle" => Unit::Bundle,
            other => {
                if let Some(rest) = other.strip_prefix("custom:") {
                    Unit::Custom(rest.to_string())
                } else {
                    Unit::Custom(other.to_string())
                }
            }
        }
    }

    pub fn dimension(&self) -> Dimension {
        match self {
            Unit::Kg | Unit::G => Dimension::Mass,
            Unit::Liter | Unit::Ml => Dimension::Volume,
            Unit::Piece | Unit::Dozen => Dimension::Count,
            _ => Dimension::Container,
        }
    }

    /// Arabic display name (household-facing).
    pub fn display_ar(&self) -> String {
        match self {
            Unit::Kg => "كجم".into(),
            Unit::G => "جرام".into(),
            Unit::Liter => "لتر".into(),
            Unit::Ml => "مل".into(),
            Unit::Piece => "قطعة".into(),
            Unit::Dozen => "دزينة".into(),
            Unit::Carton => "كرتونة".into(),
            Unit::Plate => "طبق".into(),
            Unit::Package => "عبوة".into(),
            Unit::Bottle => "زجاجة".into(),
            Unit::Bundle => "ربطة".into(),
            Unit::Custom(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    Mass,
    Volume,
    Count,
    Container,
}

/// Universal conversions between units of the same dimension.
/// Returns `None` when the conversion is not factually valid.
pub fn universal_convert(amount: f64, from: &Unit, to: &Unit) -> Option<f64> {
    use Unit::*;
    if from == to {
        return Some(amount);
    }
    let factor = match (from, to) {
        (G, Kg) | (Ml, Liter) => 0.001,
        (Kg, G) | (Liter, Ml) => 1000.0,
        (Dozen, Piece) => 12.0,
        (Piece, Dozen) => 1.0 / 12.0,
        _ => return None,
    };
    Some(amount * factor)
}

/// Arabic keyword parsing for quantities commonly found in price posts.
pub fn parse_arabic_quantity_word(word: &str) -> Option<(f64, Unit)> {
    let w = word.trim();
    match w {
        "كيلو" | "كجم" | "كيلوجرام" => Some((1.0, Unit::Kg)),
        "نص" | "نصف كيلو" | "نص كيلو" => Some((0.5, Unit::Kg)),
        "ربع كيلو" | "ربع" => Some((0.25, Unit::Kg)),
        "جرام" => Some((1.0, Unit::G)),
        "لتر" => Some((1.0, Unit::Liter)),
        "نص لتر" | "نصف لتر" => Some((0.5, Unit::Liter)),
        "طبق" => Some((1.0, Unit::Plate)),
        "قطعة" => Some((1.0, Unit::Piece)),
        "دزينة" => Some((1.0, Unit::Dozen)),
        "كرتونة" => Some((1.0, Unit::Carton)),
        "عبوة" => Some((1.0, Unit::Package)),
        "زجاجة" => Some((1.0, Unit::Bottle)),
        "ربطة" => Some((1.0, Unit::Bundle)),
        "حسب الطلب" => Some((1.0, Unit::Custom("حسب الطلب".into()))),
        _ => None,
    }
}

/// Universal conversions between units of the same dimension (string codes).
pub fn universal_convert_str(amount: f64, from: &str, to: &str) -> Option<f64> {
    universal_convert(amount, &Unit::from_code(from), &Unit::from_code(to))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_dimension_conversions() {
        assert_eq!(universal_convert(2.0, &Unit::Kg, &Unit::G), Some(2000.0));
        assert_eq!(universal_convert(500.0, &Unit::G, &Unit::Kg), Some(0.5));
        assert_eq!(universal_convert(1.5, &Unit::Liter, &Unit::Ml), Some(1500.0));
        assert_eq!(universal_convert(3.0, &Unit::Dozen, &Unit::Piece), Some(36.0));
    }

    #[test]
    fn invalid_dimension_conversion_is_none() {
        assert_eq!(universal_convert(2.0, &Unit::Kg, &Unit::Liter), None);
        // Container units never convert universally.
        assert_eq!(universal_convert(1.0, &Unit::Carton, &Unit::Piece), None);
        assert_eq!(universal_convert(1.0, &Unit::Plate, &Unit::Kg), None);
    }

    #[test]
    fn unit_codes_roundtrip() {
        for u in [
            Unit::Kg, Unit::G, Unit::Liter, Unit::Ml, Unit::Piece, Unit::Dozen, Unit::Carton,
            Unit::Plate, Unit::Package, Unit::Bottle, Unit::Bundle, Unit::Custom("طبق كبير".into()),
        ] {
            assert_eq!(Unit::from_code(&u.code()), u);
        }
    }

    #[test]
    fn arabic_quantity_words() {
        assert_eq!(parse_arabic_quantity_word("نص كيلو"), Some((0.5, Unit::Kg)));
        assert_eq!(parse_arabic_quantity_word("طبق"), Some((1.0, Unit::Plate)));
        assert_eq!(parse_arabic_quantity_word("لتر"), Some((1.0, Unit::Liter)));
    }
}
