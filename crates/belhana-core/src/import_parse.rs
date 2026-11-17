//! Parser for pasted price-list posts (e.g. Facebook merchant price posts).
//!
//! Principles implemented here:
//! - The raw source text is preserved verbatim; parsing only *proposes*.
//! - Ambiguous units are flagged, never silently normalized.
//! - Weekday/date mismatches are detected, never silently repaired.
//! - Repeated or conflicting lines stay separately traceable.
//! - Promotional ("was X now Y") prices keep both values.

use chrono::{Datelike, NaiveDate, Weekday};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ParsedLine {
    /// The exact original line from the source text.
    pub raw: String,
    /// Proposed product label (text before the price).
    pub label: String,
    /// Proposed prices in minor units (multiple when the source lists tiers).
    pub prices_minor: Vec<i64>,
    /// Previously claimed price when the source mentions a promotion.
    pub previous_price_minor: Option<i64>,
    /// Unit keyword found in the line (raw Arabic), if any.
    pub unit_word: Option<String>,
    /// Normalized unit code proposed (e.g. "kg", "plate"); still needs confirmation.
    pub proposed_unit: Option<String>,
    /// Quantity mentioned explicitly, e.g. "نص كيلو" → 0.5 kg.
    pub proposed_qty: Option<f64>,
    /// Flags for the review screen.
    pub flags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ParsedPost {
    pub original_text: String,
    pub normalized_text: String,
    /// The original date sentence as it appears in the source (preserved).
    pub original_date_text: Option<String>,
    /// Normalized date only when unambiguous. `None` until a human confirms.
    pub proposed_date: Option<String>,
    pub weekday_word: Option<String>,
    pub weekday_mismatch: Option<bool>,
    pub lines: Vec<ParsedLine>,
}

/// Converts Arabic-Indic digits to ASCII.
pub fn normalize_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c as u32 {
            d @ 0x0660..=0x0669 => char::from_u32(d - 0x0660 + b'0' as u32).unwrap_or(c),
            _ => c,
        })
        .collect()
}

fn strip_emojis_and_entities(s: &str) -> String {
    let mut s = s.to_string();
    for (ent, rep) in [
        ("&nbsp;", " "),
        ("&amp;", "&"),
        ("&quot;", "\""),
        ("&#160;", " "),
        ("&lt;", "<"),
        ("&gt;", ">"),
    ] {
        s = s.replace(ent, rep);
    }
    s.chars()
        .filter(|c| {
            let cp = *c as u32;
            // Keep ASCII, Arabic, Arabic presentation forms, punctuation, spaces.
            cp < 0x0250
                || (0x0600..=0x06FF).contains(&cp)
                || (0x0750..=0x077F).contains(&cp)
                || (0x08A0..=0x08FF).contains(&cp)
                || (0xFB50..=0xFDFF).contains(&cp)
                || (0xFE70..=0xFEFF).contains(&cp)
        })
        .collect()
}

fn normalize_line(s: &str) -> String {
    let mut s = strip_emojis_and_entities(s);
    s = normalize_digits(&s);
    // Collapse whitespace.
    let mut out = String::new();
    let mut last_space = false;
    for c in s.chars() {
        if c.is_whitespace() {
            if !last_space {
                out.push(' ');
            }
            last_space = true;
        } else {
            out.push(c);
            last_space = false;
        }
    }
    out.trim().to_string()
}

const MARKETING_HINTS: &[&str] = &[
    "أسعارنا", "توصيل", "اطلب", "خدمة", "وصلت", "جديد", "بالتوفيق", "مستعدين", "زي", "العرض",
    "خصم", "باسم", "العنوان", "شكرا", "لا يوجد", "متاح",
];

const UNIT_WORDS: &[(&str, &str, Option<f64>)] = &[
    ("نص كيلو", "kg", Some(0.5)),
    ("نص كيلو", "kg", Some(0.5)),
    ("نصف كيلو", "kg", Some(0.5)),
    ("ربع كيلو", "kg", Some(0.25)),
    ("كيلو", "kg", Some(1.0)),
    ("كجم", "kg", Some(1.0)),
    ("جرام", "g", Some(1.0)),
    ("لتر", "l", Some(1.0)),
    ("نص لتر", "l", Some(0.5)),
    ("الطبق", "plate", None),
    ("طبق", "plate", None),
    ("دزينة", "dozen", None),
    ("كرتونة", "carton", None),
    ("قطعة", "piece", None),
    ("عبوة", "package", None),
    ("زجاجة", "bottle", None),
    ("ربطة", "bundle", None),
    ("حسب الطلب", "custom:حسب الطلب", None),
    ("جميع الأوزان", "custom:جميع الأوزان", None),
    ("من أول قطعة", "piece", None),
];

fn find_unit_word(text: &str) -> Option<(String, String, Option<f64>)> {
    for (word, unit, qty) in UNIT_WORDS {
        if text.contains(word) {
            return Some((word.to_string(), unit.to_string(), *qty));
        }
    }
    None
}

const WEEKDAY_WORDS: &[(&str, Weekday)] = &[
    ("السبت", Weekday::Sat),
    ("الاحد", Weekday::Sun),
    ("الأحد", Weekday::Sun),
    ("الاثنين", Weekday::Mon),
    ("الإثنين", Weekday::Mon),
    ("الإتنين", Weekday::Mon),
    ("الثلاثاء", Weekday::Tue),
    ("تلات", Weekday::Tue),
    ("الاربعاء", Weekday::Wed),
    ("الأربعاء", Weekday::Wed),
    ("الخميس", Weekday::Thu),
    ("الجمعة", Weekday::Fri),
];

fn find_weekday_word(text: &str) -> Option<&'static str> {
    for (w, _) in WEEKDAY_WORDS {
        if text.contains(w) {
            return Some(w);
        }
    }
    None
}

/// Finds a date like 14/11/2027 or 14-11-2027 in the text.
pub fn find_date(text: &str) -> Option<(String, NaiveDate)> {
    let t = normalize_digits(text);
    for sep in ['/', '-'] {
        let parts: Vec<&str> = t.split_whitespace().collect();
        for p in parts {
            let segs: Vec<&str> = p.split(sep).collect();
            if segs.len() == 3 {
                if let (Ok(d), Ok(m), Ok(y)) = (
                    segs[0].parse::<u32>(),
                    segs[1].parse::<u32>(),
                    segs[2].parse::<i32>(),
                ) {
                    if (1..=31).contains(&d) && (1..=12).contains(&m) && (1000..=3000).contains(&y) {
                        if let Some(date) = NaiveDate::from_ymd_opt(y, m, d) {
                            return Some((format!("{d:02}/{m:02}/{y}"), date));
                        }
                    }
                }
            }
        }
    }
    None
}

/// Parses a full multi-line post.
pub fn parse_post(text: &str) -> ParsedPost {
    let mut lines = Vec::new();
    let mut occurrence: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut original_date_text = None;
    let mut proposed_date = None;
    let mut weekday_word = None;
    let mut weekday_mismatch = None;

    for raw_line in text.lines() {
        let norm = normalize_line(raw_line);
        if norm.is_empty() {
            continue;
        }
        // Date sentence detection.
        if original_date_text.is_none() && (norm.contains("أسعار") || norm.contains("اليوم")) {
            if let Some((_, date)) = find_date(&norm) {
                original_date_text = Some(norm.clone());
                proposed_date = Some(date.format("%Y-%m-%d").to_string());
                weekday_word = find_weekday_word(&norm).map(|s| s.to_string());
                if let Some(w) = find_weekday_word(&norm) {
                    let expected = weekday_name_ar(date.weekday());
                    weekday_mismatch = Some(expected != w);
                }
                continue;
            }
        }

        let parsed = parse_price_line(&norm);
        let Some(mut line) = parsed else {
            // No digits: possible product mention without price or marketing text.
            let is_marketing = MARKETING_HINTS.iter().any(|h| norm.contains(h));
            if !is_marketing && norm.chars().count() <= 60 {
                lines.push(ParsedLine {
                    raw: raw_line.trim().to_string(),
                    label: norm.clone(),
                    prices_minor: vec![],
                    previous_price_minor: None,
                    unit_word: find_unit_word(&norm).map(|(_, u, _)| u),
                    proposed_unit: find_unit_word(&norm).map(|(_, u, _)| u),
                    proposed_qty: find_unit_word(&norm).and_then(|(_, _, q)| q),
                    flags: vec!["no_price".into()],
                });
            }
            continue;
        };

        // Duplicate occurrence tracking (same label + price).
        let key = format!("{}|{}", line.label, line.prices_minor.first().copied().unwrap_or(0));
        let idx = occurrence.entry(key).and_modify(|c| *c += 1).or_insert(0);
        if *idx > 0 {
            line.flags.push("repeated_occurrence".into());
        }
        lines.push(line);
    }

    ParsedPost {
        original_text: text.to_string(),
        normalized_text: text.lines().map(normalize_line).filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n"),
        original_date_text,
        proposed_date,
        weekday_word,
        weekday_mismatch,
        lines,
    }
}

pub fn weekday_name_ar(w: Weekday) -> &'static str {
    match w {
        Weekday::Sat => "السبت",
        Weekday::Sun => "الأحد",
        Weekday::Mon => "الاثنين",
        Weekday::Tue => "الثلاثاء",
        Weekday::Wed => "الأربعاء",
        Weekday::Thu => "الخميس",
        Weekday::Fri => "الجمعة",
    }
}

/// Parses one normalized line into a candidate price line.
pub fn parse_price_line(line: &str) -> Option<ParsedLine> {
    let mut flags = Vec::new();
    let mut working = line.to_string();

    // Leading quantity prefix like "2 كيلو سكر ..." → quantity, flag ambiguity.
    let mut proposed_qty: Option<f64> = None;
    let mut proposed_unit: Option<String> = None;
    let mut unit_word: Option<String> = None;
    if let Some((word, unit, qty)) = find_unit_word(&working) {
        // explicit quantity words anywhere in the line
        if let Some(q) = qty {
            proposed_qty = Some(q);
        }
        proposed_unit = Some(unit);
        unit_word = Some(word);
    }

    // Numbers in the line.
    let mut tokens: Vec<String> = Vec::new();
    let mut cur = String::new();
    for c in working.chars() {
        if c.is_ascii_digit() || c == '.' {
            cur.push(c);
        } else {
            if !cur.is_empty() {
                tokens.push(std::mem::take(&mut cur));
            }
        }
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }
    if tokens.is_empty() {
        return None;
    }

    // Find prices: sequences separated by "/" count as tiers; "كان/بدلا من" is previous.
    let mut prices: Vec<f64> = Vec::new();
    let mut previous: Option<f64> = None;
    let tier_split: Vec<&str> = working.split('/').collect();
    let tiered = tier_split.len() > 1;

    if tiered {
        for seg in tier_split {
            if let Some(t) = first_number_in(seg) {
                prices.push(t);
            }
        }
    } else {
        // previous price detection: "كان 200" / "بدلا من 200" / "بدل 200"
        let lowered = working.replace("بدلا", "كان").replace("بدل", "كان");
        if lowered.contains("كان") {
            if let Some(pos) = lowered.find("كان") {
                let after = &lowered[pos + "كان".len()..];
                if let Some(v) = first_number_in(after) {
                    previous = Some(v);
                }
            }
        }
        if let Some(v) = last_number_in(&working) {
            if previous.map(|p| p != v).unwrap_or(true) {
                prices.push(v);
            }
        }
    }

    if prices.is_empty() {
        // A line with only "was" price or quantity digits — still not a priced offer.
        if previous.is_none() {
            return None;
        }
        flags.push("promotion_only".into());
    }

    // Label: text before the last number token.
    if let Some(last) = tokens.last() {
        let pos = working.rfind(last).unwrap_or(0);
        working = working[..pos].trim().to_string();
    }
    // Strip leading list markers.
    let label = working
        .trim_start_matches(['•', '-', '*', '◆', '🔹'])
        .trim()
        .to_string();
    if label.is_empty() {
        return None;
    }

    // Quantity/packaging ambiguity flags.
    if let Some(q) = proposed_qty {
        if q != 1.0 {
            flags.push("explicit_quantity_confirm".into());
        }
    }
    if proposed_unit.is_none() && !prices.is_empty() {
        flags.push("unit_not_stated".into());
    }
    if let Some((word, _, _)) = find_unit_word(line) {
        if word == "الطبق" || word == "طبق" {
            flags.push("plate_quantity_unknown".into());
        }
        if word == "حسب الطلب" {
            flags.push("order_based_service".into());
        }
        if word == "جميع الأوزان" {
            flags.push("all_weights_unclear".into());
        }
    }
    if prices.len() > 1 || tiered {
        flags.push("tiered_prices".into());
    }

    let prices_minor: Vec<i64> = prices.iter().map(|p| (p * 100.0).round() as i64).collect();
    Some(ParsedLine {
        raw: line.to_string(),
        label,
        prices_minor,
        previous_price_minor: previous.map(|p| (p * 100.0).round() as i64),
        unit_word,
        proposed_unit,
        proposed_qty,
        flags,
    })
}

fn first_number_in(s: &str) -> Option<f64> {
    let mut num = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() || c == '.' {
            num.push(c);
        } else if !num.is_empty() {
            break;
        }
    }
    if num.is_empty() {
        None
    } else {
        num.parse::<f64>().ok()
    }
}

fn last_number_in(s: &str) -> Option<f64> {
    let mut num = String::new();
    for c in s.chars().rev() {
        if c.is_ascii_digit() || c == '.' {
            num.insert(0, c);
        } else if !num.is_empty() {
            break;
        }
    }
    if num.is_empty() {
        None
    } else {
        num.parse::<f64>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_line_without_unit() {
        let l = parse_price_line("طماطم جامدة جدا 30").unwrap();
        assert_eq!(l.label, "طماطم جامدة جدا");
        assert_eq!(l.prices_minor, vec![3000]);
        assert!(l.flags.contains(&"unit_not_stated".to_string()));
        assert!(l.proposed_unit.is_none());
    }

    #[test]
    fn parses_explicit_half_kg() {
        let l = parse_price_line("توم أحمر بلدي 30 نص كيلو").unwrap();
        assert_eq!(l.proposed_unit.as_deref(), Some("kg"));
        assert_eq!(l.proposed_qty, Some(0.5));
        assert_eq!(l.prices_minor, vec![3000]);
    }

    #[test]
    fn parses_plate_unit() {
        let l = parse_price_line("ماشروم طازج فريش الطبق 60").unwrap();
        assert_eq!(l.proposed_unit.as_deref(), Some("plate"));
        assert!(l.flags.contains(&"plate_quantity_unknown".to_string()));
    }

    #[test]
    fn parses_tiered_prices() {
        let l = parse_price_line("الحمام 200 / 250 / 300 / 350 حسب الحجم زوج").unwrap();
        assert_eq!(l.prices_minor.len(), 4);
        assert!(l.flags.contains(&"tiered_prices".to_string()));
    }

    #[test]
    fn parses_promotion_previous_price() {
        let l = parse_price_line("قشطة عبد الرزاق 150 بدلا من 200").unwrap();
        assert_eq!(l.prices_minor, vec![15000]);
        assert_eq!(l.previous_price_minor, Some(20000));
    }

    #[test]
    fn parses_leading_quantity_ambiguity() {
        let l = parse_price_line("2 كيلو سكر كيلو كيلو 55").unwrap();
        assert_eq!(l.proposed_qty, Some(1.0)); // first unit word found is كيلو
        assert_eq!(l.prices_minor, vec![5500]);
        // raw text preserved for review
        assert!(l.raw.contains("2 كيلو"));
    }

    #[test]
    fn detects_date_and_weekday_mismatch() {
        // 14/11/2027 is a Sunday; source claims Saturday.
        let p = parse_post("أسعارنا لصباح اليوم السبت 14/11/2027\nطماطم 30");
        assert!(p.original_date_text.is_some());
        assert_eq!(p.proposed_date.as_deref(), Some("2027-11-14"));
        assert_eq!(p.weekday_mismatch, Some(true));
        assert_eq!(p.lines.len(), 1);
    }

    #[test]
    fn keeps_no_price_mentions() {
        let p = parse_post("توابل متنوعة\nجلاش النحراوي\nاطلب الآن توصيل");
        assert_eq!(p.lines.len(), 2);
        assert!(p.lines[0].flags.contains(&"no_price".to_string()));
    }

    #[test]
    fn repeated_lines_traced_separately() {
        let text = "جنزبيل فريش 50 الطبق\nعنب أحمر 70\nجنزبيل فريش 50 الطبق";
        let p = parse_post(text);
        assert_eq!(p.lines.len(), 3);
        assert!(p.lines[2].flags.contains(&"repeated_occurrence".to_string()));
    }

    #[test]
    fn arabic_indic_digits_normalized() {
        let l = parse_price_line("طماطم ٣٠");
        assert!(l.is_some());
        assert_eq!(l.unwrap().prices_minor, vec![3000]);
    }
}
