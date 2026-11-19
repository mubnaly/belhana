//! Explainable, keyword-based food-preference filtering.
//!
//! A household's selected diet steers which recipes are suggested and planned.
//! This is a **preference filter**, not a medical protocol: matched recipes are
//! suggestions only, calories/macros stay `≈` estimates, and no health outcome
//! is claimed. Unknown ids and informational plans (elimination, balanced
//! family) intentionally apply no extra filter — hard allergy/exclusion rules
//! in the planner still protect the user.

use crate::recipes::Recipe;

/// (allow, deny) keyword lists in normalised Arabic + English.
/// Empty lists mean "unrestricted".
const MEDERRANEAN: (&[&str], &[&str]) = (
    &[
        "سمك", "عدس", "حمص", "خضار", "سلطة", "زيت زيتون", "مكسرات", "دجاج",
        "شوفان", "كينوا", "طماطم", "fish", "lentil", "chickpea", "salad", "olive oil",
    ],
    &["سجق", "كبد", "لحم مصنع", "حلوي", "sausage", "processed meat"],
);

const TAYYIBAT: (&[&str], &[&str]) = (
    &[
        "لحم", "سمك", "سردين", "ارز", "بطاطس", "تمر", "سمنه", "زيت زيتون",
        "جبن معتق", "meat", "fish", "sardine", "rice", "potato", "date", "ghee",
    ],
    &[
        "دجاج", "فراخ", "بيض", "لبن", "حليب", "جبن طازج", "فول", "عدس", "حمص",
        "لوبيا", "طماطم", "فلفل", "باذنجان", "خس", "سبانخ", "ملوخيه", "موز",
        "بطيخ", "شمام", "عصير", "صودا", "سكر", "دقيق", "مكرونة", "chicken", "egg",
        "milk", "lentil", "chickpea", "tomato", "pepper", "sugar", "juice", "flour", "pasta",
    ],
);

const KETO: (&[&str], &[&str]) = (
    &[
        "لحم", "كفتة", "دجاج", "فراخ", "سمك", "تونه", "بيض", "جبن", "زيتون",
        "لوز", "جوز", "مكسرات", "خيار", "كوسا", "سبانخ", "قرنبيط", "بروكلي",
        "افوكادو", "سمنه", "زيت زيتون", "meat", "kofta", "chicken", "fish", "tuna",
        "egg", "cheese", "olive oil", "avocado", "spinach", "cauliflower",
    ],
    &[
        "ارز", "مكرونة", "خبز", "دقيق", "بطاطس", "سكر", "عصير", "فول", "عدس",
        "حمص", "لوبيا", "موز", "تفاح", "تمر", "سميد", "برغل", "شوفان", "كينوا",
        "ذره", "حلوي", "كنافه", "بسبوسه", "مهلبية", "قطايف", "rice", "pasta", "bread",
        "flour", "potato", "sugar", "juice", "lentil", "chickpea", "bean", "banana", "apple",
    ],
);

/// Fold Arabic spelling variants and strip diacritics so keywords match
/// regardless of harakat, alef form, ta marbuta or alef maqsura.
pub fn normalize(text: &str) -> String {
    text.chars()
        .filter(|c| !matches!(c, '\u{064B}'..='\u{065F}' | '\u{0670}'))
        .map(|c| match c {
            'أ' | 'إ' | 'آ' => 'ا',
            'ة' => 'ه',
            'ى' => 'ي',
            _ => c,
        })
        .collect::<String>()
        .to_lowercase()
}

fn contains(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| text.contains(n))
}

/// Returns the keyword pair for a selectable diet id. `None` means the plan is
/// informational/unrestricted and must not remove any recipe.
pub fn keywords(diet_id: &str) -> Option<(&'static [&'static str], &'static [&'static str])> {
    match diet_id {
        "mediterranean" => Some(MEDERRANEAN),
        "tayyibat" => Some(TAYYIBAT),
        "keto" => Some(KETO),
        // elimination depends on each member's exclusions (handled as hard
        // constraints in the planner); balanced_family is unrestricted.
        _ => None,
    }
}

/// True when a recipe is compatible with the selected food preference.
pub fn recipe_matches(diet_id: Option<&str>, recipe: &Recipe, ingredients_text: &str) -> bool {
    let Some(id) = diet_id else { return true };
    if id.is_empty() || id == "all" {
        return true;
    }
    let Some((allow, deny)) = keywords(id) else {
        return true;
    };

    let mut text = normalize(&format!(
        "{} {} {}",
        recipe.name_ar,
        recipe.name_en.as_deref().unwrap_or(""),
        recipe.description.as_deref().unwrap_or("")
    ));
    text.push(' ');
    text.push_str(&normalize(ingredients_text));
    for tag in &recipe.tags {
        text.push(' ');
        text.push_str(&normalize(tag));
    }

    if !deny.is_empty() && contains(&text, deny) {
        return false;
    }
    allow.is_empty() || contains(&text, allow)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recipe(name: &str) -> Recipe {
        Recipe {
            id: "r1".into(),
            household_id: None,
            name_ar: name.into(),
            name_en: None,
            description: None,
            cuisine: "egyptian".into(),
            meal_types: vec!["lunch".into()],
            difficulty: "easy".into(),
            prep_minutes: 10,
            cook_minutes: 20,
            default_servings: 4,
            cooking_method: None,
            equipment: vec![],
            allergens: vec![],
            storage_notes: None,
            source_type: "system".into(),
            source_ref: None,
            child_portion_factor: 1.0,
            is_system: true,
            tags: vec![],
            kcal_per_serving: None,
            macros_per_serving: None,
        }
    }

    #[test]
    fn normalize_folds_arabic_variants() {
        assert_eq!(normalize("أَرُّز"), "ارز");
        assert_eq!(normalize("جبنة"), "جبنه");
        assert_eq!(normalize("الدجاج"), "الدجاج");
    }

    #[test]
    fn keto_excludes_rice_and_keeps_meat() {
        assert!(!recipe_matches(Some("keto"), &recipe("أرز بالخضار"), "أرز، خضار"));
        assert!(recipe_matches(Some("keto"), &recipe("كفتة"), "لحم مفروم، توابل"));
    }

    #[test]
    fn tayyibat_excludes_chicken_and_keeps_rice_and_meat() {
        assert!(!recipe_matches(Some("tayyibat"), &recipe("شاورما دجاج"), "دجاج"));
        assert!(recipe_matches(Some("tayyibat"), &recipe("محمرة"), "أرز، لحم"));
    }

    #[test]
    fn unrestricted_plans_keep_every_recipe() {
        assert!(recipe_matches(None, &recipe("كشري"), "عدس، مكرونة"));
        assert!(recipe_matches(Some("balanced_family"), &recipe("كشري"), "عدس"));
        assert!(recipe_matches(Some("unknown_plan"), &recipe("كشري"), "عدس"));
    }
}
