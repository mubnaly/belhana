//! Static seed data (embedded at compile time).
//!
//! Sources:
//! - Products/categories: Belhana editorial catalog for the Egyptian market.
//! - Recipes: original content written for Belhana (not copied from external
//!   sites), adapted to Egyptian households.
//! - Baseline prices: user-supplied editable estimates (never merchant quotes).
//! - Olad Zain post: transcription reconstructed from the tables supplied by
//!   the user; the app must preserve all ambiguities of the original source.

pub const CATEGORIES: &[(&str, &str, &str, i64)] = &[
    ("vegetables", "خضروات", "Vegetables", 1),
    ("fruits", "فواكة", "Fruits", 2),
    ("meat", "لحوم", "Meat", 3),
    ("poultry", "دواجن وطيور", "Poultry", 4),
    ("fish", "أسماك ومأكولات بحرية", "Fish & Seafood", 5),
    ("eggs", "بيض", "Eggs", 6),
    ("dairy", "ألبان وأجبان", "Dairy", 7),
    ("legumes", "بقوليات", "Legumes", 8),
    ("grains", "حبوب وأرز ومكرونة", "Grains & Rice", 9),
    ("flour_bakery", "دقيق ومخبوزات", "Flour & Bakery", 10),
    ("oils", "زيوت ودهون", "Oils & Fats", 11),
    ("spices", "توابل وبهارات", "Spices", 12),
    ("frozen", "مجمدات", "Frozen Products", 13),
    ("ready", "أطعمة جاهزة ومحضّرة", "Ready-made Foods", 14),
    ("snacks", "سناكس وحلويات", "Snacks & Sweets", 15),
    ("beverages", "مشروبات", "Beverages", 16),
    ("essentials", "أساسيات المطبخ", "Kitchen Essentials", 17),
];

pub struct SeedProductDef {
    #[allow(dead_code)]
    pub slug: &'static str,
    pub category: &'static str,
    pub name_ar: &'static str,
    pub name_en: Option<&'static str>,
    pub default_unit: Option<&'static str>,
    pub package_size: Option<f64>,
    pub package_unit: Option<&'static str>,
    pub aliases: Option<&'static [&'static str]>,
}

macro_rules! p {
    ($slug:expr, $cat:expr, $ar:expr, $en:expr, $unit:expr) => {
        SeedProductDef { slug: $slug, category: $cat, name_ar: $ar, name_en: Some($en), default_unit: Some($unit), package_size: None, package_unit: None, aliases: None }
    };
    ($slug:expr, $cat:expr, $ar:expr, $en:expr, $unit:expr, $pkg:expr, $pkgunit:expr) => {
        SeedProductDef { slug: $slug, category: $cat, name_ar: $ar, name_en: Some($en), default_unit: Some($unit), package_size: Some($pkg), package_unit: Some($pkgunit), aliases: None }
    };
}

pub const PRODUCTS: &[SeedProductDef] = &[
    // vegetables
    SeedProductDef { slug: "tomato", category: "vegetables", name_ar: "طماطم", name_en: Some("Tomato"), default_unit: Some("kg"), aliases: Some(&["طماطم جامدة", "طماطم حمراء", "طماطم"]) , package_size: None, package_unit: None },
    p!("green_beans", "vegetables", "فاصوليا خضراء", "Green beans", "kg"),
    p!("sweet_potato", "vegetables", "بطاطا سكري", "Sweet potato", "kg"),
    p!("mushrooms", "vegetables", "مشروم", "Mushrooms", "package"),
    p!("ginger", "vegetables", "جنزبيل", "Ginger", "kg"),
    p!("spinach", "vegetables", "سبانخ", "Spinach", "kg"),
    p!("green_peas", "vegetables", "بسلة", "Green peas", "kg"),
    p!("cucumber", "vegetables", "خيار بلدي", "Cucumber", "kg"),
    p!("potato", "vegetables", "بطاطس", "Potato", "kg"),
    p!("beetroot", "vegetables", "بنجر", "Beetroot", "kg"),
    SeedProductDef { slug: "onion", category: "vegetables", name_ar: "بصل", name_en: Some("Onion"), default_unit: Some("kg"), aliases: Some(&["بصل أحمر", "بصل أصفر"]) , package_size: None, package_unit: None },
    p!("garlic", "vegetables", "توم", "Garlic", "kg"),
    p!("lemon", "vegetables", "ليمون", "Lemon", "kg"),
    p!("carrot", "vegetables", "جزر", "Carrot", "kg"),
    p!("zucchini", "vegetables", "كوسة", "Zucchini", "kg"),
    p!("green_pepper", "vegetables", "فلفل أخضر", "Green pepper", "kg"),
    p!("chili_pepper", "vegetables", "شطة", "Chili pepper", "kg"),
    p!("molokhia", "vegetables", "ملوخية", "Molokhia", "kg"),
    p!("colored_peppers", "vegetables", "فلفل ألوان", "Bell peppers", "kg"),
    p!("eggplant", "vegetables", "باذنجان", "Eggplant", "kg"),
    SeedProductDef { slug: "cabbage", category: "vegetables", name_ar: "كرومب", name_en: Some("Cabbage"), default_unit: Some("kg"), aliases: Some(&["كرنب", "كرومب حشو"]) , package_size: None, package_unit: None },
    p!("lettuce", "vegetables", "خس", "Lettuce", "piece"),
    p!("mint", "vegetables", "نعناع", "Mint", "bundle"),
    p!("green_onion", "vegetables", "بصل أخضر", "Green onion", "bundle"),
    p!("arugula", "vegetables", "جرجير", "Arugula", "bundle"),
    p!("parsley", "vegetables", "بقدونس", "Parsley", "bundle"),
    p!("coriander", "vegetables", "كزبرة خضراء", "Coriander", "bundle"),
    // fruits
    p!("oranges", "fruits", "برتقال", "Oranges", "kg"),
    p!("guava", "fruits", "جوافة", "Guava", "kg"),
    p!("cantaloupe", "fruits", "كنتالوب", "Cantaloupe", "kg"),
    p!("persimmon", "fruits", "كاكا", "Persimmon", "kg"),
    p!("dates", "fruits", "بلح", "Dates", "kg"),
    p!("custard_apple", "fruits", "قشطة", "Custard apple", "kg"),
    p!("dragon_fruit", "fruits", "دراجون فروت", "Dragon fruit", "piece"),
    p!("mango", "fruits", "مانجو", "Mango", "kg"),
    p!("grapes", "fruits", "عنب", "Grapes", "kg"),
    p!("banana", "fruits", "موز", "Banana", "kg"),
    p!("kiwi", "fruits", "كيوي", "Kiwi", "kg"),
    p!("apples", "fruits", "تفاح", "Apples", "kg"),
    // meat
    SeedProductDef { slug: "beef", category: "meat", name_ar: "لحم", name_en: Some("Beef"), default_unit: Some("kg"), aliases: Some(&["لحم بقري", "لحوم"]) , package_size: None, package_unit: None },
    p!("minced_beef", "meat", "مفروم لحمة", "Minced beef", "kg"),
    // poultry
    SeedProductDef { slug: "chicken_white", category: "poultry", name_ar: "فراخ بيضاء", name_en: Some("White chicken"), default_unit: Some("kg"), aliases: Some(&["فراخ", "دجاج"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "chicken_baladi", category: "poultry", name_ar: "فراخ بلدي فلاحي", name_en: Some("Baladi chicken"), default_unit: Some("kg"), aliases: Some(&["البلدي الفلاحي", "فراخ بلدي"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "chicken_babari", category: "poultry", name_ar: "فراخ برابرة", name_en: Some("Babari chicken"), default_unit: Some("kg"), aliases: Some(&["البرابرة الفلاحي"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "chicken_fayoumi", category: "poultry", name_ar: "فراخ فيومي", name_en: Some("Fayoumi chicken"), default_unit: Some("kg"), aliases: Some(&["الفيومي"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "chicken_sasso", category: "poultry", name_ar: "فراخ ساسو", name_en: Some("Sasso chicken"), default_unit: Some("kg"), aliases: Some(&["الساسو"]) , package_size: None, package_unit: None },
    p!("duck_baladi", "poultry", "بط بلدي", "Baladi duck", "kg"),
    p!("duck_mugal", "poultry", "بط بغال", "Mugal duck", "kg"),
    p!("duck_french", "poultry", "بط فرنساوي", "French duck", "kg"),
    p!("turkey", "poultry", "رومي بلدي", "Turkey", "kg"),
    p!("rabbit", "poultry", "أرانب", "Rabbit", "kg"),
    SeedProductDef { slug: "pigeons", category: "poultry", name_ar: "حمام", name_en: Some("Pigeons"), default_unit: Some("pair"), aliases: Some(&["زوج"]) , package_size: None, package_unit: None },
    // fish & seafood
    SeedProductDef { slug: "fish", category: "fish", name_ar: "سمك", name_en: Some("Fish"), default_unit: Some("kg"), aliases: Some(&["أسماك"]) , package_size: None, package_unit: None },
    p!("shrimp", "fish", "جمبري", "Shrimp", "kg"),
    p!("fish_fillet", "fish", "سمك فيليه", "Fish fillet", "kg"),
    p!("seafood_mix", "fish", "سيفود", "Seafood mix", "kg"),
    // eggs
    p!("eggs", "eggs", "بيض", "Eggs", "carton", 30.0, "piece"),
    // dairy
    p!("mozzarella", "dairy", "موزاريلا", "Mozzarella", "kg"),
    p!("butter_crystal", "dairy", "زبدة كريستال", "Crystal butter", "package"),
    p!("butter", "dairy", "زبدة فرن", "Oven butter", "package"),
    p!("white_cheese", "dairy", "جبنة بيضاء", "White cheese", "kg"),
    p!("yogurt", "dairy", "لبن", "Yogurt", "l"),
    // legumes
    SeedProductDef { slug: "lentils", category: "legumes", name_ar: "عدس", name_en: Some("Lentils"), default_unit: Some("kg"), aliases: None , package_size: None, package_unit: None },
    SeedProductDef { slug: "cowpeas", category: "legumes", name_ar: "لوبيا", name_en: Some("Cowpeas"), default_unit: Some("kg"), aliases: None , package_size: None, package_unit: None },
    SeedProductDef { slug: "fava_beans", category: "legumes", name_ar: "فول", name_en: Some("Fava beans"), default_unit: Some("kg"), aliases: Some(&["فول بلدي"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "chickpeas", category: "legumes", name_ar: "حمص", name_en: Some("Chickpeas"), default_unit: Some("kg"), aliases: None , package_size: None, package_unit: None },
    // grains
    SeedProductDef { slug: "rice", category: "grains", name_ar: "أرز", name_en: Some("Rice"), default_unit: Some("kg"), aliases: Some(&["رز"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "pasta", category: "grains", name_ar: "مكرونة", name_en: Some("Pasta"), default_unit: Some("kg"), aliases: Some(&["معكرونة"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "bulgur", category: "grains", name_ar: "برغل", name_en: Some("Bulgur"), default_unit: Some("kg"), aliases: None , package_size: None, package_unit: None },
    // flour & bakery
    SeedProductDef { slug: "flour", category: "flour_bakery", name_ar: "دقيق", name_en: Some("Flour"), default_unit: Some("kg"), aliases: Some(&["دقيق فاخر"]) , package_size: None, package_unit: None },
    p!("yeast", "flour_bakery", "خميرة", "Yeast", "package"),
    SeedProductDef { slug: "baladi_bread", category: "flour_bakery", name_ar: "خبز بلدي", name_en: Some("Baladi bread"), default_unit: Some("piece"), aliases: Some(&["عيش بلدي", "خبز"]) , package_size: None, package_unit: None },
    p!("gellash_nahrawy", "flour_bakery", "جلاش النحراوي", "Nahrawy gellash", "package"),
    p!("rakak_nahrawy", "flour_bakery", "رقاق النحراوي", "Nahrawy rakak", "package"),
    p!("sombosa_nahrawy", "flour_bakery", "سمبوسة النحراوي", "Nahrawy sombosa", "package"),
    // oils & fats
    SeedProductDef { slug: "cooking_oil", category: "oils", name_ar: "زيت طعام", name_en: Some("Cooking oil"), default_unit: Some("l"), aliases: Some(&["زيت"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "ghee", category: "oils", name_ar: "سمنة", name_en: Some("Ghee"), default_unit: Some("kg"), aliases: Some(&["سمنة روابي", "سمنة كريستال"]) , package_size: None, package_unit: None },
    p!("oil_sweet", "oils", "زيت حلوة", "Sweet oil (brand)", "package"),
    // spices
    SeedProductDef { slug: "mixed_spices", category: "spices", name_ar: "توابل متنوعة", name_en: Some("Mixed spices"), default_unit: Some("package"), aliases: Some(&["توابل"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "fenugreek", category: "spices", name_ar: "حلبة", name_en: Some("Fenugreek"), default_unit: Some("kg"), aliases: Some(&["حلبة بلدي"]) , package_size: None, package_unit: None },
    SeedProductDef { slug: "cumin", category: "spices", name_ar: "كمون", name_en: Some("Cumin"), default_unit: Some("kg"), aliases: None , package_size: None, package_unit: None },
    SeedProductDef { slug: "black_pepper", category: "spices", name_ar: "فلفل أسود", name_en: Some("Black pepper"), default_unit: Some("kg"), aliases: None , package_size: None, package_unit: None },
    SeedProductDef { slug: "salt", category: "spices", name_ar: "ملح", name_en: Some("Salt"), default_unit: Some("kg"), aliases: None , package_size: None, package_unit: None },
    SeedProductDef { slug: "vinegar", category: "spices", name_ar: "خل", name_en: Some("Vinegar"), default_unit: Some("l"), aliases: None , package_size: None, package_unit: None },
    SeedProductDef { slug: "tahini", category: "spices", name_ar: "طحينة", name_en: Some("Tahini"), default_unit: Some("kg"), aliases: None , package_size: None, package_unit: None },
    // frozen
    p!("pane_frozen", "frozen", "بانيه فراخ", "Frozen pane", "package"),
    p!("pane_sharqia", "frozen", "بانيه التوابل الشرقية", "Sharqia spices pane", "package"),
    p!("pane_eldar", "frozen", "بانيه الدار", "Eldar pane", "package"),
    p!("strips_frozen", "frozen", "استربس", "Frozen strips", "package"),
    p!("strips_rustic", "frozen", "استربس روستي", "Rusty strips", "package"),
    p!("jordan_blue", "frozen", "جوردن بلو", "Jordan Blue", "package"),
    p!("shawarma_frozen", "frozen", "شاورما جاهزة", "Frozen shawarma", "package"),
    SeedProductDef { slug: "chicken_wings", category: "frozen", name_ar: "وراك فراخ", name_en: Some("Chicken wings"), default_unit: Some("kg"), aliases: Some(&["الوراك", "الأجنحة"]) , package_size: None, package_unit: None },
    p!("shish_tawook", "frozen", "شيش طاووق", "Shish tawook", "package"),
    p!("drumsticks", "frozen", "دبابيس", "Drumsticks", "kg"),
    p!("pigeon_kadab", "frozen", "الحمام الكداب", "Pigeon kadab", "package"),
    p!("liver_gizzards", "frozen", "كبد وقوانص", "Liver & gizzards", "kg"),
    p!("minced_brazilian", "frozen", "مفروم برازيلي", "Brazilian minced", "kg"),
    p!("cuts_brazilian", "frozen", "قطع برازيلي", "Brazilian cuts", "kg"),
    p!("cuts_indian", "frozen", "قطع هندي", "Indian cuts", "kg"),
    p!("minced_indian", "frozen", "مفروم هندي", "Indian minced", "kg"),
    p!("liver_alex", "frozen", "كبدة إسكندراني", "Alexandrian liver", "kg"),
    p!("bania", "frozen", "بانية", "Bania", "kg"),
    p!("sogo", "frozen", "سجق مخصوص", "Special sogo", "kg"),
    p!("burger_brazilian", "frozen", "برجر مخصوص", "Special burger", "package"),
    p!("burger_golden", "frozen", "برجر جولدن بيف", "Golden beef burger", "package"),
    p!("burger_mazaq", "frozen", "برجر المذاق", "Mazaq burger", "package"),
    p!("quail", "frozen", "سمان العابد", "Abiad quail", "package"),
    p!("mombar", "frozen", "ممبار ساده العابد", "Abiad mombar", "package"),
    p!("hamad_mix", "frozen", "ميكس الحمد", "El Hamd mix", "package"),
    p!("makhrouta", "frozen", "مخروطه", "Makhrouta", "package"),
    p!("masrouda", "frozen", "مصروده", "Masrouda", "package"),
    p!("kuskosi", "frozen", "كسكسي", "Kuskosi", "package"),
    p!("gellash", "frozen", "جلاش", "Gellash", "package"),
    p!("sombosk", "frozen", "سمبوسك", "Sombosk", "package"),
    p!("shrimp_jumbo", "frozen", "جمبري جامبو بقشر", "Jumbo shrimp", "kg"),
    // ready & prepared
    p!("molokhia_cut", "ready", "ملوخية مقطفة", "Chopped molokhia", "package"),
    p!("eggplant_hollowed", "ready", "باذنجان مقور جاهز على الحشو", "Hollowed eggplant", "kg"),
    p!("zucchini_hollowed", "ready", "كوسة مقورة", "Hollowed zucchini", "kg"),
    p!("vegetable_soup_mix", "ready", "شوربة خضار جاهزة", "Vegetable soup mix", "package"),
    // snacks
    SeedProductDef { slug: "feshar", category: "snacks", name_ar: "فيشار", name_en: Some("Feshar"), default_unit: Some("package"), aliases: None , package_size: None, package_unit: None },
    // beverages
    SeedProductDef { slug: "tea", category: "beverages", name_ar: "شاي", name_en: Some("Tea"), default_unit: Some("package"), aliases: None , package_size: None, package_unit: None },
    SeedProductDef { slug: "coffee", category: "beverages", name_ar: "قهوة سريعة الذوبان", name_en: Some("Instant coffee"), default_unit: Some("package"), aliases: None , package_size: None, package_unit: None },
    // specialty baking & dessert ingredients
    SeedProductDef { slug: "semolina", category: "flour_bakery", name_ar: "سميد", name_en: Some("Semolina"), default_unit: Some("kg"), aliases: Some(&["سميد خشن", "سميد ناعم"]) , package_size: None, package_unit: None },
    p!("desiccated_coconut", "flour_bakery", "جوز هند", "Desiccated coconut", "package"),
    p!("mixed_nuts", "snacks", "مكسرات مشكلة", "Mixed nuts", "kg"),
    p!("kunafa", "flour_bakery", "كنافة", "Kunafa", "kg"),
    p!("qatayef", "flour_bakery", "قطايف", "Qatayef", "package"),
    p!("starch", "flour_bakery", "نشا", "Starch", "package"),
    p!("vanilla", "essentials", "فانيليا", "Vanilla", "package"),
    // essentials
    SeedProductDef { slug: "sugar", category: "essentials", name_ar: "سكر", name_en: Some("Sugar"), default_unit: Some("kg"), aliases: Some(&["سكر أبيض"]) , package_size: None, package_unit: None },
];

/// Editorial calorie estimates for the catalog. `kcal` is per 100 g (or 100 ml
/// for liquids) and `grams` is the typical edible weight of one `default_unit`.
/// These are rounded household estimates, not laboratory data; the UI labels
/// them as estimates.
pub fn product_nutrition(category: &str, name_ar: &str, default_unit: Option<&str>) -> (f64, f64) {
    // Named overrides first: the everyday foods where a category average would
    // be clearly wrong (bread, eggs, oils, sugar, dry legumes, ...).
    let kcal = match name_ar {
        "خبز بلدي" => 270.0,
        "بيض" => 155.0,
        "لبن" => 60.0,
        "جبنة بيضاء" => 250.0,
        "موزاريلا" => 280.0,
        "زبدة كريستال" | "زبدة فرن" => 720.0,
        "سمنة" => 880.0,
        "زيت طعام" | "زيت حلوة" => 880.0,
        "أرز" => 360.0,
        "مكرونة" => 370.0,
        "دقيق" => 364.0,
        "سميد" => 360.0,
        "سكر" => 400.0,
        "فول" | "لوبيا" => 340.0,
        "عدس" => 350.0,
        "حمص" => 360.0,
        "طحينة" => 595.0,
        "مكسرات مشكلة" => 600.0,
        "جوز هند" => 660.0,
        "كنافة" => 330.0,
        "قطايف" => 300.0,
        "جلاش النحراوي" | "جلاش" => 380.0,
        "رقاق النحراوي" | "رقاق" => 360.0,
        "سمبوسة النحراوي" | "سمبوسك" => 350.0,
        "نشا" => 380.0,
        "فانيليا" => 280.0,
        "خميرة" => 300.0,
        "فيشار" => 380.0,
        "شاي" => 0.0,
        "قهوة سريعة الذوبان" => 350.0,
        "ملح" => 0.0,
        "كمون" => 375.0,
        "فلفل أسود" => 250.0,
        "حلبة" => 320.0,
        "خل" => 20.0,
        "توابل متنوعة" => 250.0,
        "خس" => 15.0,
        "طماطم" => 18.0,
        "خيار بلدي" => 16.0,
        "بطاطس" => 77.0,
        "بطاطا سكري" => 86.0,
        "بصل" | "بصل أخضر" => 40.0,
        "جزر" => 41.0,
        "كوسة" => 17.0,
        "باذنجان" => 25.0,
        "فلفل أخضر" | "فلفل ألوان" => 20.0,
        "سبانخ" => 23.0,
        "ملوخية" | "ملوخية مقطفة" => 40.0,
        "كرومب" => 25.0,
        "بسلة" => 81.0,
        "فاصوليا خضراء" => 31.0,
        "مشروم" => 22.0,
        "بنجر" => 43.0,
        "توم" => 149.0,
        "ليمون" => 29.0,
        "نعناع" => 70.0,
        "جرجير" => 25.0,
        "بقدونس" => 36.0,
        "كزبرة خضراء" => 23.0,
        "موز" => 89.0,
        "تفاح" => 52.0,
        "برتقال" => 47.0,
        "جوافة" => 68.0,
        "مانجو" => 60.0,
        "عنب" => 69.0,
        "كيوي" => 61.0,
        "كنتالوب" => 34.0,
        "بلح" => 280.0,
        "قشطة" => 100.0,
        "كاكا" => 70.0,
        "دراجون فروت" => 60.0,
        "لحم" | "مفروم لحمة" => 250.0,
        "فراخ بيضاء" | "فراخ بلدي فلاحي" | "فراخ برابرة" | "فراخ فيومي" | "فراخ ساسو" => 190.0,
        "بط بلدي" | "بط بغال" | "بط فرنساوي" => 340.0,
        "رومي بلدي" => 150.0,
        "أرانب" => 170.0,
        "حمام" => 290.0,
        "سمك" | "سمك فيليه" => 130.0,
        "جمبري" | "جمبري جامبو بقشر" | "سيفود" => 100.0,
        "كبدة إسكندراني" | "كبد وقوانص" => 200.0,
        "سجق مخصوص" => 300.0,
        "بانيه فراخ" | "بانيه التوابل الشرقية" | "بانيه الدار" | "استربس" | "استربس روستي"
        | "جوردن بلو" => 250.0,
        "برجر مخصوص" | "برجر جولدن بيف" | "برجر المذاق" => 280.0,
        "شاورما جاهزة" | "شيش طاووق" => 200.0,
        "وراك فراخ" | "دبابيس" => 220.0,
        "ممبار ساده العابد" | "مخروطه" | "مصروده" | "كسكسي" => 280.0,
        "باذنجان مقور جاهز على الحشو" => 25.0,
        "كوسة مقورة" => 17.0,
        "شوربة خضار جاهزة" => 60.0,
        _ => match category {
            "vegetables" => 30.0,
            "fruits" => 55.0,
            "meat" => 250.0,
            "poultry" => 190.0,
            "fish" => 130.0,
            "eggs" => 155.0,
            "dairy" => 250.0,
            "legumes" => 340.0,
            "grains" => 350.0,
            "flour_bakery" => 350.0,
            "oils" => 880.0,
            "spices" => 250.0,
            "frozen" => 220.0,
            "ready" => 80.0,
            "snacks" => 450.0,
            "beverages" => 200.0,
            "essentials" => 300.0,
            _ => 100.0,
        },
    };

    let grams = match name_ar {
        "بيض" => 1500.0, // carton of 30 eggs × 50 g
        "خبز بلدي" => 90.0,
        "خس" => 300.0,
        "دراجون فروت" => 300.0,
        "زبدة كريستال" | "زبدة فرن" => 250.0,
        "جلاش النحراوي" | "جلاش" => 500.0,
        "رقاق النحراوي" | "رقاق" => 400.0,
        "سمبوسة النحراوي" | "سمبوسك" => 400.0,
        "كنافة" | "قطايف" => 500.0,
        "نشا" => 250.0,
        "فانيليا" => 20.0,
        "خميرة" => 100.0,
        "جوز هند" => 200.0,
        "فيشار" => 100.0,
        "شاي" => 250.0,
        "قهوة سريعة الذوبان" => 200.0,
        "زيت حلوة" => 250.0,
        "توابل متنوعة" => 100.0,
        "نعناع" | "بصل أخضر" | "جرجير" | "بقدونس" | "كزبرة خضراء" => 100.0,
        _ => match (category, default_unit) {
            ("frozen", Some("package")) => 400.0,
            ("flour_bakery", Some("package")) => 400.0,
            ("ready", Some("package")) => 400.0,
            ("snacks", Some("package")) => 150.0,
            ("beverages", Some("package")) => 200.0,
            ("dairy", Some("package")) => 250.0,
            (_, Some("kg")) | (_, Some("l")) => 1000.0,
            (_, Some("g")) | (_, Some("ml")) => 1.0,
            (_, Some("piece")) => 100.0,
            (_, Some("bundle")) => 100.0,
            (_, Some("carton")) => 600.0,
            (_, Some("dozen")) => 600.0,
            (_, Some("pair")) => 500.0,
            _ => 100.0,
        },
    };

    (kcal, grams)
}

/// Editorial macronutrient estimates per 100 g (or 100 ml), in the order
/// (protein, carbohydrates, fat, fibre). Calibrated from USDA-derived food
/// tables (Wikipedia's table of food nutrients) and rounded for household use.
/// Like the calorie figures these are estimates, not laboratory data.
pub fn product_macros(name_ar: &str, category: &str) -> (f64, f64, f64, f64) {
    let macros = match name_ar {
        "خبز بلدي" => (10.0, 55.0, 2.5, 3.0),
        "بيض" => (13.0, 1.1, 11.0, 0.0),
        "لبن" => (3.5, 4.5, 2.5, 0.0),
        "جبنة بيضاء" => (14.0, 3.0, 19.0, 0.0),
        "موزاريلا" => (22.0, 2.0, 22.0, 0.0),
        "قشطة" => (3.0, 3.0, 9.0, 0.0),
        "زبدة كريستال" | "زبدة فرن" | "سمنة" | "زيت طعام" | "زيت حلوة" => (0.0, 0.0, 100.0, 0.0),
        "أرز" | "أرز بسمتي" => (7.0, 80.0, 0.6, 1.3),
        "مكرونة" => (13.0, 72.0, 1.5, 3.0),
        "دقيق" => (10.0, 76.0, 1.0, 2.7),
        "سميد" => (12.0, 72.0, 1.0, 2.0),
        "سكر" => (0.0, 100.0, 0.0, 0.0),
        "فول" | "لوبيا" => (23.0, 60.0, 1.5, 8.0),
        "عدس" => (24.0, 60.0, 1.0, 10.0),
        "حمص" => (20.0, 61.0, 4.0, 12.0),
        "طحينة" => (17.0, 21.0, 53.0, 9.0),
        "مكسرات مشكلة" => (20.0, 20.0, 50.0, 6.0),
        "جوز هند" => (7.0, 15.0, 68.0, 7.0),
        "كنافة" | "قطايف" | "جلاش" | "رقاق" | "سمبوسة النحراوي" | "سمبوسك" => (7.0, 55.0, 12.0, 2.0),
        "لحم" | "مفروم لحمة" => (22.0, 0.0, 16.0, 0.0),
        "فراخ بيضاء" | "فراخ بلدي فلاحي" | "فراخ برابرة" | "فراخ فيومي" | "فراخ ساسو" => (23.0, 0.0, 9.0, 0.0),
        "بط بلدي" | "بط بغال" | "بط فرنساوي" => (18.0, 0.0, 28.0, 0.0),
        "سمك" | "سمك فيليه" => (20.0, 0.0, 5.0, 0.0),
        "جمبري" | "جمبري جامبو بقشر" | "سيفود" => (23.0, 0.0, 1.0, 0.0),
        "كبدة إسكندراني" | "كبد وقوانص" => (20.0, 4.0, 12.0, 0.0),
        "وراك فراخ" | "دبابيس" => (21.0, 0.0, 13.0, 0.0),
        "طماطم" => (0.9, 4.0, 0.2, 1.2),
        "خيار بلدي" => (0.7, 3.6, 0.1, 0.5),
        "بطاطس" => (2.0, 20.0, 0.1, 2.0),
        "بطاطا سكري" => (1.6, 20.0, 0.1, 3.0),
        "بصل" | "بصل أخضر" => (1.1, 9.0, 0.1, 1.7),
        "جزر" => (0.9, 10.0, 0.2, 2.8),
        "كوسة" => (1.2, 3.0, 0.3, 1.0),
        "سبانخ" => (3.0, 3.0, 0.4, 2.0),
        "بسلة" => (5.0, 14.0, 0.4, 5.0),
        "فاصوليا خضراء" => (1.8, 7.0, 0.2, 3.0),
        "خس" => (1.4, 2.9, 0.2, 1.3),
        "موز" => (1.1, 23.0, 0.3, 2.6),
        "تفاح" => (0.3, 14.0, 0.2, 2.4),
        "برتقال" => (0.9, 12.0, 0.1, 2.4),
        "شاي" | "ملح" => (0.0, 0.0, 0.0, 0.0),
        "خل" => (0.0, 1.0, 0.0, 0.0),
        _ => match category {
            "vegetables" => (2.0, 5.0, 0.3, 2.0),
            "fruits" => (1.0, 15.0, 0.3, 2.0),
            "meat" => (22.0, 0.0, 16.0, 0.0),
            "poultry" => (23.0, 0.0, 9.0, 0.0),
            "fish" => (20.0, 0.0, 5.0, 0.0),
            "eggs" => (13.0, 1.1, 11.0, 0.0),
            "dairy" => (12.0, 5.0, 15.0, 0.0),
            "legumes" => (22.0, 60.0, 2.0, 9.0),
            "grains" => (9.0, 75.0, 1.5, 2.0),
            "flour_bakery" => (9.0, 55.0, 8.0, 2.0),
            "oils" => (0.0, 0.0, 100.0, 0.0),
            "spices" => (10.0, 40.0, 10.0, 15.0),
            "frozen" => (10.0, 25.0, 9.0, 2.0),
            "ready" => (8.0, 22.0, 8.0, 2.0),
            "snacks" => (7.0, 55.0, 20.0, 3.0),
            "beverages" => (1.0, 20.0, 0.5, 0.0),
            "essentials" => (5.0, 40.0, 5.0, 2.0),
            _ => (5.0, 20.0, 5.0, 2.0),
        },
    };
    macros
}

pub struct SeedRecipeDef {
    pub name_ar: &'static str,
    pub name_en: &'static str,
    pub description: &'static str,
    pub meal_types: &'static [&'static str],
    pub difficulty: &'static str,
    pub prep: i64,
    pub cook: i64,
    pub servings: i64,
    pub method: &'static str,
    pub tags: &'static [&'static str],
    pub allergens: &'static [&'static str],
    pub ingredients: &'static [(&'static str, f64, &'static str)],
    pub steps: &'static [&'static str],
}

pub const RECIPES: &[SeedRecipeDef] = &[
    SeedRecipeDef {
        name_ar: "فول مدمس", name_en: "Ful Medames", description: "فطور مصري اقتصادي غني بالبروتين.",
        meal_types: &["breakfast", "lunch"], difficulty: "easy", prep: 5, cook: 45, servings: 4, method: "boiled",
        tags: &["legume", "vegetarian", "economical"], allergens: &[],
        ingredients: &[("فول", 0.5, "kg"), ("توم", 0.02, "kg"), ("ليمون", 0.05, "kg"), ("زيت طعام", 0.05, "l"), ("كمون", 0.005, "kg")],
        steps: &["لو الفول جاف: انقعه ليلة كاملة (8–12 ساعة) في ماء بارد، وغيّر الماء مرة على الأقل. لو عندك فول مسلوق أو معلّب، اغسله وصفّه وابدأ من الخطوة الثالثة.",
                 "ضع الفول في حلة مع ماء يغطيه بثلاثة أصابع، وضع طبقًا صغيرًا فوقه حتى لا يطفو. اغلِ ثم خفّف النار واتركه 45–60 دقيقة للفول الجاف أو 15 دقيقة للمعلّب حتى ينضج ويتفتت بالضغط بالإصبع.",
                 "صفّي الفول مع الاحتفاظ بكوب من ماء السلق. اهرس نصف الكمية بالشوكة أو بالهاند بلندر مع الثوم المدقوق والكمون والملح، واترك النصف الآخر حبًا كاملًا لقوام تقليدي.",
                 "أعد الفول للحلة مع قليل من ماء السلق على نار هادئة، وقلّب حتى يصبح كثيفًا مثل الكريمة السميكة. لو جفّ زوّد ماءً ساخنًا بالتدريج.",
                 "قدّمه في طبق ساخن مع زيت الزيتون أو الزيت العادي، عصير ليمون طازج، شطة أو كمون إضافي، وخبز بلدي ساخن. يكفي 4 أشخاص مع الطعمية والسلطة."],
    },
    SeedRecipeDef {
        name_ar: "طعمية", name_en: "Taameya (Falafel)", description: "أقراص الفول المصرية المشهورة، اقتصادية وتصلح للتجميد.",
        meal_types: &["breakfast", "lunch"], difficulty: "medium", prep: 30, cook: 20, servings: 6, method: "fried",
        tags: &["legume", "vegetarian", "economical", "batch_friendly"], allergens: &["سمسم"],
        ingredients: &[("فول", 0.5, "kg"), ("بقدونس", 0.05, "bundle"), ("كزبرة خضراء", 0.05, "bundle"), ("توم", 0.03, "kg"), ("بصل", 0.1, "kg"), ("دقيق", 0.05, "kg"), ("توابل متنوعة", 0.01, "package")],
        steps: &["انقع الفول الجاف (المقشّر أفضل) في ماء بارد من 8 إلى 12 ساعة. لا تستخدم الفول المسلوق أو المعلّب لأن الأقراص ستتفكك في الزيت.",
                 "صفّي الفول جيدًا وجففه بالمناديل. اطحنه مع البصل والثوم والبقدونس والكزبرة الخضراء والفلفل الأخضر إن رغبت، حتى يصبح ناعمًا كالعجين.",
                 "انقل الخليط لوعاء، أضف التوابل والملح والكمون والدقيق. اعجن 2–3 دقائق، ثم غطّه واتركه في الثلاجة 30 دقيقة ليتكاثف ويسهل تشكيله.",
                 "شكّل أقراصًا بقطر 5 سم وسماكة 1.5 سم بيد مبللة أو بقالب الطعمية. لو الخليط ليّن زوّد دقيقًا ملعقة في كل مرة.",
                 "سخّن زيتًا غزيرًا على حرارة متوسطة (170–175°م). اقلي الأقراص 3–4 دقائق لكل جانب حتى تصبح بنية ذهبية، ثم صفّها على ورق مطبخ.",
                 "لنسخة أقل زيتًا: رصّ الأقراص على صينية مدهونة، ادهن وجهها بقليل زيت واخبزها في فرن 200°م لمدة 12–15 دقيقة مع التقليب. قدّمها ساخنة مع الطحينة والسلطة والخبز."],
    },
    SeedRecipeDef {
        name_ar: "شكشوكة", name_en: "Shakshouka", description: "بيض مطبوخ في صلصة الطماطم، فطور سريع ومغذٍ.",
        meal_types: &["breakfast", "lunch"], difficulty: "easy", prep: 10, cook: 20, servings: 4, method: "stewed",
        tags: &["vegetarian", "quick"], allergens: &["بيض"],
        ingredients: &[("طماطم", 0.5, "kg"), ("بيض", 4.0, "piece"), ("فلفل أخضر", 0.05, "kg"), ("بصل", 0.1, "kg"), ("زيت طعام", 0.05, "l")],
        steps: &["سخّن الزيت في طاسة واسعة على نار متوسطة، وشوّح البصل المقطع مكعبات صغيرة 3–4 دقائق حتى يشف.",
                 "أضف الفلفل الأخضر المقطع والثوم المفروم والملح والكمون والفلفل الأسود. قلّب دقيقة حتى تخرج رائحة الثوم.",
                 "أضف الطماطم المبشورة أو المقطعة وملعقة صغيرة سكر لتقليل الحموضة. اتركها 10–12 دقيقة على نار هادئة حتى تتكاثف الصلصة ويظهر الزيت على السطح.",
                 "اعمل حفرًا بالملعقة في الصلصة، واكسر بيضة في كل حفرة. رشّ ملحًا وفلفلًا على البيض.",
                 "غطِ الطاسة واطبخ 5–7 دقائق حتى يتماسك البياض ويبقى الصفار سائلًا، أو 8–10 دقائق لو تحب البيض كاملًا.",
                 "قدّمها فورًا في الطاسة مع خبز بلدي ساخن وسلطة خضراء. تكفي 4 أشخاص."],
    },
    SeedRecipeDef {
        name_ar: "شوربة عدس", name_en: "Lentil Soup", description: "شوربة اقتصادية دافئة من البقوليات.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 10, cook: 30, servings: 5, method: "boiled",
        tags: &["legume", "vegetarian", "economical"], allergens: &[],
        ingredients: &[("عدس", 0.5, "kg"), ("بصل", 0.1, "kg"), ("جزر", 0.1, "kg"), ("طماطم", 0.2, "kg"), ("توابل متنوعة", 0.01, "package")],
        steps: &["اغسل العدس الأحمر جيدًا حتى يصفو الماء. قطّع البصل والجزر والطماطم مكعبات متوسطة.",
                 "سخّن ملعقة زيت في حلة، شوّح البصل 3 دقائق، ثم أضف الجزر والطماطم والتوابل (كمون، فلفل أسود، ملح) وقلّب دقيقتين.",
                 "أضف العدس و6 أكواب ماء ساخن. اغلِ ثم خفّف النار واترك 25–30 دقيقة حتى يتفتت العدس تمامًا.",
                 "اطحن الشوربة بالهاند بلندر حتى تصبح ناعمة. لو سميكة زوّد ماءً ساخنًا بالتدريج واضبط الملح.",
                 "قدّمها ساخنة مع عصير ليمون طازج ورشة كمون ومكعبات خبز محمّص. تحفظ في الثلاجة 3 أيام وتتجمد بشكل ممتاز."],
    },
    SeedRecipeDef {
        name_ar: "كشري", name_en: "Koshari", description: "الطبق المصري الوطني: أرز وعدس ومكرونة وصلصة.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 20, cook: 40, servings: 6, method: "boiled",
        tags: &["legume", "vegetarian", "economical"], allergens: &["جلوتين"],
        ingredients: &[("أرز", 0.4, "kg"), ("عدس", 0.2, "kg"), ("مكرونة", 0.3, "kg"), ("بصل", 0.2, "kg"), ("طماطم", 0.5, "kg"), ("خل", 0.05, "l")],
        steps: &["اغسل الأرز وانقعه 15 دقيقة. اسلق العدس البني في ماء مملح 20–25 دقيقة حتى ينضج مع بقائه متماسكًا، ثم صفّه.",
                 "اسلق المكرونة (صغيرة أو إسباجتي مقطعة) في ماء مملح حسب تعليمات العلبة، صفّها ورشّ عليها قليل زيت حتى لا تلتصق.",
                 "اطبخ الأرز: شوّحه في ملعقة زيت دقيقة، أضف ماءً ساخنًا بنسبة 1:1.5 وملحًا، غطّه على نار هادئة 15–18 دقيقة.",
                 "للصلصة: شوّح البصل والثوم في زيت، أضف الطماطم المبشورة والخل والكمون والشطة، واتركها 10 دقائق حتى تتكاثف.",
                 "حمّر البصل المقطع حلقات رفيعة في زيت غزير على نار متوسطة مع رشة دقيق حتى يصبح مقرمشًا ذهبيًا، ثم صفّه على ورق مطبخ.",
                 "رتّب في الطبق: أرز ثم عدس ثم مكرونة، وفوقها صلصة وبصل محمّر ورشة خل وشطة. قدّمه مع الحمص المسلوق إن توفّر. يكفي 6 أشخاص."],
    },
    SeedRecipeDef {
        name_ar: "ملوخية", name_en: "Molokhia", description: "طبق مصري كلاسيكي مع شوربة الفراخ.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 10, cook: 25, servings: 4, method: "boiled",
        tags: &["traditional"], allergens: &[],
        ingredients: &[("ملوخية", 0.5, "kg"), ("توم", 0.05, "kg"), ("فراخ بيضاء", 0.3, "kg"), ("سمنة", 0.02, "kg")],
        steps: &["لو الفراخ كاملة: اغسلها، ضعها في حلة مع بصل وورق لورا وحبهان وملح، وغطّها بماء. اغلِ ثم أزل الرغوة واتركها 35–40 دقيقة حتى تنضج. صفّ الشوربة واحتفظ بالفراخ.",
                 "لو تستخدم ملوخية مقطفة مجمدة اتركها تفك قليلًا. لو طازجة: اغسلها وجففها جيدًا ثم قطّعها ناعمًا بسكين حاد.",
                 "سخّن الشوربة حتى تغلي بلطف. أضف الملوخية تدريجيًا مع التحريك المستمر في اتجاه واحد حتى لا تتكتل، واتركها 5–7 دقائق فقط على نار هادئة. لا تغلِ بقوة حتى لا تفقد لونها.",
                 "للتقلية: دقّ الثوم مع الكزبرة الجافة إن توفّرت. سخّن السمنة في مقلاة صغيرة، حمّر الثوم حتى يصبح ذهبيًا، ثم اسكبه فورًا فوق الملوخية وقلّب.",
                 "تبّل بالملح، وقدّمها ساخنة مع الأرز الأبيض والفراخ وعصير ليمون لمن يحب. لو أصبحت سميكة زوّد شوربة ساخنة."],
    },
    SeedRecipeDef {
        name_ar: "مسقعة", name_en: "Musakaa", description: "باذنجان بالصلصة والخضار — وجبة نباتية مشبعة.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 20, cook: 45, servings: 4, method: "baked",
        tags: &["vegetarian"], allergens: &[],
        ingredients: &[("باذنجان", 0.7, "kg"), ("بطاطس", 0.3, "kg"), ("طماطم", 0.5, "kg"), ("بصل", 0.1, "kg"), ("فلفل أخضر", 0.05, "kg"), ("زيت طعام", 0.3, "l")],
        steps: &["قطّع الباذنجان حلقات بسماكة 1 سم، رشّه بالملح واتركه 15 دقيقة لخروج الماء المر، ثم جففه. قطّع البطاطس حلقات رفيعة.",
                 "اشوِ الباذنجان والبطاطس في فرن 200°م مع رشة زيت لمدة 15–20 دقيقة، أو اقلهما سريعًا. الشوي يقلل الزيت كثيرًا.",
                 "للصلصة: شوّح البصل والفلفل في ملعقتين زيت، أضف الثوم ثم الطماطم المبشورة والملح والفلفل والكمون. اتركها 10 دقائق.",
                 "في صينية، رتّب طبقة بطاطس ثم باذنجان ثم صلصة، وكرّر الطبقات. غطِ الوجه بالصلصة.",
                 "اخبزها في فرن 190°م لمدة 25–30 دقيقة حتى تتماسك وتتحمّر الحواف. اتركها 10 دقائق قبل التقديم.",
                 "قدّمها دافئة مع الخبز أو الأرز. تحفظ في الثلاجة 3 أيام."],
    },
    SeedRecipeDef {
        name_ar: "بطاطس بالصلصة", name_en: "Potatoes in tomato sauce", description: "بطاطس اقتصادية وسهلة بالصلصة.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 15, cook: 30, servings: 4, method: "stewed",
        tags: &["vegetarian", "economical"], allergens: &[],
        ingredients: &[("بطاطس", 1.0, "kg"), ("طماطم", 0.5, "kg"), ("توم", 0.02, "kg"), ("زيت طعام", 0.1, "l")],
        steps: &["قطّع البطاطس مكعبات أو شرائح بسماكة 1 سم، اغسلها من النشا وصفّها.",
                 "سخّن الزيت في حلة، شوّح البطاطس 5 دقائق حتى تكتسب لونًا خفيفًا دون قلي غزير.",
                 "أضف الثوم المدقوق وقلّب 30 ثانية، ثم أضف الطماطم المبشورة والملح والفلفل الأسود ورشة كمون وملعقة صغيرة سكر.",
                 "أضف كوب ماء ساخن، غطِ الحلة واطبخ على نار هادئة 20–25 دقيقة حتى تنضج البطاطس وتثخن الصلصة.",
                 "لو الصلصة سائلة ارفع الغطاء 5 دقائق. قدّمها ساخنة مع الأرز الأبيض والخبز. تكفي 4."],
    },
    SeedRecipeDef {
        name_ar: "صينية خضار", name_en: "Vegetable tray", description: "خضار مشكلة بالفرن — عملية وقليلة الدهون.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 20, cook: 40, servings: 4, method: "baked",
        tags: &["vegetarian", "healthy"], allergens: &[],
        ingredients: &[("كوسة", 0.3, "kg"), ("باذنجان", 0.3, "kg"), ("بطاطس", 0.3, "kg"), ("جزر", 0.2, "kg"), ("طماطم", 0.3, "kg"), ("فلفل ألوان", 0.1, "kg"), ("زيت طعام", 0.1, "l")],
        steps: &["سخّن الفرن على 200°م. اغسل وقطّع كل الخضار قطعًا متساوية بحجم 2–3 سم حتى تنضج معًا.",
                 "ضع الخضار في صينية كبيرة، أضف الزيت والملح والفلفل الأسود والكمون والأعشاب الجافة إن توفّرت. قلّب بيديك حتى تتغطى.",
                 "وزّع الخضار في طبقة واحدة، وأضف نصف كوب ماء أو شوربة في قاع الصينية لمنع الالتصاق.",
                 "اخبزها 25 دقيقة، ثم قلّب واخبز 15–20 دقيقة إضافية حتى تتحمّر الحواف وتنضج البطاطس.",
                 "قدّمها كطبق جانبي أو وجبة نباتية مع الأرز أو الخبز. رشّ عليها ليمونًا وبقدونس قبل التقديم."],
    },
    SeedRecipeDef {
        name_ar: "أرز بالخضار", name_en: "Rice with vegetables", description: "أرز مطبوخ مع خضار موسمية.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 15, cook: 30, servings: 4, method: "boiled",
        tags: &["vegetarian", "economical"], allergens: &[],
        ingredients: &[("أرز", 0.5, "kg"), ("كوسة", 0.2, "kg"), ("جزر", 0.2, "kg"), ("بصل", 0.1, "kg"), ("زيت طعام", 0.1, "l")],
        steps: &["اغسل الأرز وانقعه 15 دقيقة ثم صفّه. قطّع الكوسة والجزر مكعبات صغيرة والبصل مفرومًا.",
                 "سخّن الزيت في حلة، شوّح البصل حتى يشف، أضف الجزر ثم الكوسة وقلّب 3–4 دقائق.",
                 "أضف الأرز وقلّب دقيقتين حتى يتغطى بالزيت، ثم أضف ماءً ساخنًا بنسبة 1:1.5 والملح والتوابل.",
                 "اغلِ ثم خفّف النار وغطِ الحلة 15–18 دقيقة حتى ينضج الأرز ويمتص الماء.",
                 "اتركه 5 دقائق مغطى ثم حرّكه بالشوكة. قدّمه مع الزبادي أو السلطة."],
    },
    SeedRecipeDef {
        name_ar: "بيض بالخضار", name_en: "Eggs with vegetables", description: "فطور سريع بالبيض والخضار.",
        meal_types: &["breakfast", "lunch"], difficulty: "easy", prep: 10, cook: 15, servings: 2, method: "stewed",
        tags: &["quick"], allergens: &["بيض"],
        ingredients: &[("بيض", 4.0, "piece"), ("طماطم", 0.3, "kg"), ("فلفل أخضر", 0.05, "kg"), ("بصل", 0.05, "kg"), ("زيت طعام", 0.03, "l")],
        steps: &["قطّع الطماطم والفلفل الأخضر والبصل قطعًا صغيرة. اخفق البيض مع رشة ملح وفلفل.",
                 "سخّن الزيت في طاسة على نار متوسطة، شوّح البصل والفلفل 2–3 دقائق.",
                 "أضف الطماطم واتركها 3–4 دقائق حتى تلين وتقلّ عصارتها.",
                 "اسكب البيض المخفوق وقلّب بلطف بملعقة سيليكون حتى ينضج حسب رغبتك (5–7 دقائق).",
                 "قدّمه فورًا مع خبز بلدي وجبنة بيضاء. تكفي شخصين."],
    },
    SeedRecipeDef {
        name_ar: "مجدرة", name_en: "Mujadara", description: "عدس وأرز وبصل محمّر — وجبة بقوليات اقتصادية.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 10, cook: 35, servings: 4, method: "boiled",
        tags: &["legume", "vegetarian", "economical"], allergens: &[],
        ingredients: &[("عدس", 0.4, "kg"), ("أرز", 0.2, "kg"), ("بصل", 0.3, "kg"), ("زيت طعام", 0.1, "l")],
        steps: &["اغسل العدس. اسلقه في ماء مملح 15–20 دقيقة حتى ينضج نصف نضج مع بقائه متماسكًا، ثم صفّه واحتفظ بماء السلق.",
                 "اغسل الأرز وانقعه 15 دقيقة. سخّن ملعقة زيت في حلة وشوّح الأرز دقيقة.",
                 "أضف العدس والأرز معًا، ثم ماءً ساخنًا بنسبة 1:1.5 وملحًا وكمونًا وفلفلًا أسود. اغلِ ثم خفّف وغطِ 18–20 دقيقة.",
                 "أثناء ذلك، قطّع البصل حلقات رفيعة وحمّره في زيت غزير مع رشة ملح حتى يصبح مقرمشًا. صفّه على ورق مطبخ.",
                 "اترك المجدرة 10 دقائق مغطاة ثم حرّكها بالشوكة. قدّمها مع البصل المحمّر والزبادي والسلطة."],
    },
    SeedRecipeDef {
        name_ar: "فتة حمص", name_en: "Fatta with chickpeas", description: "فتة اقتصادية نباتية بالحمص والخبز المحمّص.",
        meal_types: &["lunch"], difficulty: "easy", prep: 15, cook: 20, servings: 4, method: "stewed",
        tags: &["legume", "vegetarian", "economical"], allergens: &["جلوتين"],
        ingredients: &[("حمص", 0.5, "kg"), ("خبز بلدي", 4.0, "piece"), ("طماطم", 0.4, "kg"), ("خل", 0.05, "l"), ("توم", 0.02, "kg")],
        steps: &["اسلق الحمص المنقوع من الليل 45–60 دقيقة حتى ينضج، أو استخدم حمصًا معلّبًا مغسولًا. احتفظ بكوب من ماء السلق.",
                 "قطّع الخبز البلدي مكعبات، رشّه بقليل زيت واخبزه على 150°م لمدة 10–12 دقيقة حتى يصبح مقرمشًا.",
                 "للصلصة: شوّح الثوم في زيت، أضف الطماطم المبشورة والخل والكمون والملح ورشة شطة، واتركها 10 دقائق.",
                 "رتّب الخبز في طبق عميق، اسكب عليه قليلًا من ماء السلق الساخن حتى يلين قليلًا، ثم أضف الحمص.",
                 "اسكب الصلصة فوق الفتة، ثم الزبادي إن رغبت والسمنة الساخنة. قدّمها فورًا حتى لا يذوب الخبز."],
    },
    SeedRecipeDef {
        name_ar: "شاورما دجاج منزلية", name_en: "Homemade chicken shawarma", description: "شاورما دجاج اقتصادية في البيت.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 20, cook: 25, servings: 4, method: "grilled",
        tags: &["poultry"], allergens: &["جلوتين"],
        ingredients: &[("فراخ بيضاء", 0.8, "kg"), ("خبز بلدي", 4.0, "piece"), ("طماطم", 0.2, "kg"), ("بصل", 0.1, "kg"), ("توابل متنوعة", 0.02, "package"), ("زيت طعام", 0.05, "l")],
        steps: &["قطّع صدور الدجاج شرائح رفيعة. اخلط الزبادي أو ملعقتين زيت مع الثوم وعصير الليمون والكمون والكزبرة والبابريكا والملح والفلفل. تبّل الدجاج واتركه في الثلاجة 30 دقيقة على الأقل، وساعتين أفضل.",
                 "سخّن طاسة أو جريل على نار عالية مع قليل زيت. اشوِ الدجاج في دفعات 4–5 دقائق لكل جانب حتى يتحمّر وينضج تمامًا.",
                 "أثناء الشوي، قطّع البصل شرائح وقلّبه في نفس الطاسة حتى يلين ويأخذ لون الشوي.",
                 "لصلصة الثوم: اخلط زباديًا وثومًا مدقوقًا وعصير ليمون وملحًا ورشة نعناع جاف.",
                 "سخّن الخبز البلدي، ضع الدجاج والبصل والطماطم، اسكب الصلصة، ولفّه بإحكام. قدّمه مع مخلل أو بطاطس."],
    },
    SeedRecipeDef {
        name_ar: "محمرة", name_en: "Mhammara rice", description: "أرز بالصلصة واللحم على الطريقة الليبية الشهية.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 15, cook: 35, servings: 5, method: "stewed",
        tags: &["rice_dish"], allergens: &[],
        ingredients: &[("أرز", 0.5, "kg"), ("طماطم", 0.5, "kg"), ("توم", 0.02, "kg"), ("سمنة", 0.05, "kg"), ("لحم", 0.3, "kg")],
        steps: &["اصنع صلصة الطماطم: شوّح البصل والثوم في السمنة، أضف الطماطم المبشورة والملح والفلفل الأسود والكمون، واتركها 10 دقائق.",
                 "قطّع اللحم مكعبات صغيرة، تبّله بالملح والفلفل، وشوّحه في حلة ساخنة حتى يتحمّر من كل الجهات.",
                 "أضف الصلصة واللحم معًا، أضف كوب ماء ساخن واتركه على نار هادئة 20 دقيقة حتى يلين اللحم.",
                 "اغسل الأرز وانقعه. أضف الأرز إلى الصلصة مع ماء ساخن إضافي بنسبة 1:1.5، وقلّب برفق.",
                 "اغلِ ثم خفّف النار، غطِ الحلة واطبخ 18–20 دقيقة. اتركها 10 دقائق ثم قدّمها ساخنة."],
    },
    SeedRecipeDef {
        name_ar: "شوربة خضار اقتصادية", name_en: "Economical vegetable soup", description: "شوربة خفيفة من خضار الموسم.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 10, cook: 25, servings: 4, method: "boiled",
        tags: &["vegetarian", "economical"], allergens: &[],
        ingredients: &[("شوربة خضار جاهزة", 1.0, "package"), ("بصل", 0.1, "kg"), ("بطاطس", 0.2, "kg"), ("جزر", 0.2, "kg"), ("توابل متنوعة", 0.01, "package")],
        steps: &["قطّع البصل والبطاطس والجزر وأي خضار موسمي مكعبات متوسطة.",
                 "سخّن ملعقة زيت في حلة وشوّح البصل دقيقتين، ثم أضف باقي الخضار وقلّب 3 دقائق.",
                 "أضف 5 أكواب ماء ساخن أو شوربة فراخ والتوابل، وغطِ الحلة 20–25 دقيقة حتى تلين الخضار.",
                 "اطحن نصف الشوربة بالهاند بلندر لتصبح كريمية مع بقاء قطع خضار، أو اطحنها كلها.",
                 "اضبط الملح والفلفل، وقدّمها ساخنة مع خبز محمّص وليمون."],
    },
    SeedRecipeDef {
        name_ar: "أرز باللبن", name_en: "Rice pudding", description: "حلوى منزلية اقتصادية.",
        meal_types: &["dessert"], difficulty: "easy", prep: 5, cook: 25, servings: 6, method: "boiled",
        tags: &["dessert", "economical"], allergens: &["لبن"],
        ingredients: &[("أرز", 0.2, "kg"), ("سكر", 0.15, "kg"), ("لبن", 0.5, "l"), ("توابل متنوعة", 0.01, "package")],
        steps: &["اغسل الأرز المصري وانقعه 15 دقيقة. سخّن اللبن في حلة على نار متوسطة مع التحريك حتى لا يلتصق.",
                 "أضف الأرز والسكر ورشة ملح، واطبخ على نار هادئة 25–30 دقيقة مع التحريك كل 5 دقائق حتى ينضج الأرز ويصبح القوام كريميًا.",
                 "أضف الفانيليا في آخر 5 دقائق. لو تحب القوام أسمك اتركه أكثر، ولو أسمك من اللازم زوّد لبنًا ساخنًا.",
                 "اسكب في أكواب، غطِ سطحه ببلاستيك مباشرة لمنع تكون طبقة، واتركه يبرد ثم ضعه في الثلاجة ساعتين.",
                 "قدّمه باردًا مع قرفة وجوز هند أو مكسرات محمّصة. يكفي 6 أكواب صغيرة."],
    },
    SeedRecipeDef {
        name_ar: "عصير ليمون بالنعناع", name_en: "Mint lemonade", description: "مشروب منعش اقتصادي.",
        meal_types: &["juice"], difficulty: "easy", prep: 10, cook: 0, servings: 5, method: "raw",
        tags: &["beverage", "economical"], allergens: &[],
        ingredients: &[("ليمون", 0.3, "kg"), ("سكر", 0.1, "kg"), ("نعناع", 1.0, "bundle")],
        steps: &["اغسل الليمون جيدًا. اعصره (5–6 ليمونات) واحتفظ بقشر واحدة منه لو تحب نكهة أقوى.",
                 "اخلط عصير الليمون مع لتر ماء بارد و100 جم سكر في دورق. قلّب حتى يذوب السكر.",
                 "أضف أوراق النعناع الطازج واعصرها قليلًا بملعقة خشبية لإخراج الزيوت، أو اخلط نصف الكمية مع العصير في الخلاط ثم صفّه.",
                 "أضف الثلج واتركه 10 دقائق في الثلاجة. تذوّقه وزوّد سكرًا أو ليمونًا حسب الحموضة.",
                 "قدّمه مثلجًا مع شرائح ليمون ونعناع. يكفي 5 أكواب."],
    },
    SeedRecipeDef {
        name_ar: "رقاق باللحمة", name_en: "Rakakat with beef", description: "رقاق مصري محشي باللحمة المفرومة أو المكعبات، طبق عزائم كلاسيكي.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 25, cook: 45, servings: 6, method: "baked",
        tags: &["traditional", "meat"], allergens: &["جلوتين"],
        ingredients: &[("رقاق النحراوي", 1.0, "package"), ("لحم", 0.5, "kg"), ("بصل", 0.2, "kg"), ("سمنة", 0.05, "kg"), ("توابل متنوعة", 0.02, "package")],
        steps: &["سخّن الفرن على 180°م. قطّع البصل شرائح رفيعة وشوّحه في نصف السمنة حتى يلين.",
                 "أضف اللحم المقطع مكعبات صغيرة والملح والفلفل الأسود والتوابل، وقلّب حتى يتغير لونه. أضف نصف كوب ماء ساخن واتركه 20–25 دقيقة على نار هادئة حتى ينضج ويتبقى قليل من الصلصة.",
                 "ادهن صينية بالسمنة المتبقية. رتّب طبقة من الرقاق، وادهن كل ورقة بقليل من السمنة المذابة. لو الرقاق جاف رشّه بقليل من الماء أو الشوربة الساخنة.",
                 "وزّع خليط اللحم فوق الرقاق، ثم غطّه بطبقة أخرى من الرقاق المدهون. كرّر حتى تنتهي الكمية وانتهِ بطبقة رقاق مدهونة.",
                 "اسكب نصف كوب شوربة أو ماء ساخن حول الحواف، غطِ الصينية بورق فويل، واخبزها 25 دقيقة.",
                 "ارفع الفويل واخبز 10–15 دقيقة حتى يصبح الوجه ذهبيًا ومقرمشًا. اتركها 10 دقائق ثم قطّعها وقدّمها مع سلطة خضراء وزبادي."],
    },
    SeedRecipeDef {
        name_ar: "حواوشي", name_en: "Hawawshi", description: "خبز بلدي محشو باللحمة المفرومة والمبهّرة، من أشهر أكلات الشارع المصرية.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 20, cook: 20, servings: 4, method: "baked",
        tags: &["street_food", "meat"], allergens: &["جلوتين"],
        ingredients: &[("مفروم لحمة", 0.5, "kg"), ("بصل", 0.2, "kg"), ("فلفل أخضر", 0.05, "kg"), ("خبز بلدي", 4.0, "piece"), ("توابل متنوعة", 0.02, "package"), ("زيت طعام", 0.03, "l")],
        steps: &["اخلط المفروم مع البصل المفروم ناعمًا والفلفل الأخضر المفروم والملح والفلفل الأسود والتوابل. اعجن 2–3 دقائق حتى يتماسك.",
                 "قسّم الخليط 4 كرات، وافرد كل كرة داخل قطعة خبز بلدي مفتوحة، مع ترك حافة 1 سم حتى لا يخرج الحشو.",
                 "اضغط على الخبز برفق، وادهن الوجه بقليل زيت. لو الخبز صغير استخدم قطعتين لكل حشوة.",
                 "سخّن صينية أو طاسة ثقيلة على نار متوسطة، وضع الحواوشي بحيث يلامس السطح. اضغط بطبق فوقه واطبخ 5–6 دقائق لكل جانب حتى يتحمّر الخبز وينضج اللحم.",
                 "لو تحب الفرن: اخبزه على 200°م لمدة 12–15 دقيقة مع التقليب مرة. تأكد أن اللحم نضج تمامًا قبل التقديم.",
                 "قدّمه ساخنًا مع طحينة وسلطة ومخلل. يفضّل تناوله فورًا."],
    },
    SeedRecipeDef {
        name_ar: "مكرونة بشاميل", name_en: "Pasta béchamel", description: "مكرونة باللحمة المفرومة وصلصة البشاميل والجبنة، طبق فرن محبوب.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 25, cook: 40, servings: 6, method: "baked",
        tags: &["pasta", "meat"], allergens: &["جلوتين", "لبن"],
        ingredients: &[("مكرونة", 0.5, "kg"), ("مفروم لحمة", 0.4, "kg"), ("طماطم", 0.3, "kg"), ("بصل", 0.1, "kg"), ("لبن", 0.5, "l"), ("دقيق", 0.05, "kg"), ("سمنة", 0.03, "kg"), ("موزاريلا", 0.15, "kg")],
        steps: &["اسلق المكرونة في ماء مملح حسب تعليمات العلبة حتى تنضج مع بقائها متماسكة (أقل بدقيقة). صفّها ورشّ قليل زيت.",
                 "للحشو: شوّح البصل في ملعقة سمنة، أضف المفروم وقلّب حتى يتفتت وينضج، ثم أضف الطماطم المبشورة والملح والفلفل والبهارات. اتركه 10 دقائق حتى يتكاثف.",
                 "للبيشاميل: سخّن 3 ملاعق سمنة، أضف الدقيق وقلّب دقيقة على نار هادئة، ثم أضف اللبن تدريجيًا مع التحريك المستمر بمضرب حتى لا تتكتل. اطبخ 5–7 دقائق حتى يثخن، ثم أضف ملحًا وفلفلًا ورشة جوزة الطيب.",
                 "اخلط نصف كمية البيشاميل مع المكرونة، ثم أضف الحشو وقلّب برفق.",
                 "في صينية مدهونة، ضع نصف المكرونة، ثم طبقة موزاريلا، ثم باقي المكرونة. اسكب باقي البيشاميل على الوجه ووزّعه.",
                 "اخبز في فرن 190°م لمدة 25–30 دقيقة حتى يتحمّر الوجه. اتركها 10–15 دقيقة قبل التقطيع حتى تتماسك."],
    },
    SeedRecipeDef {
        name_ar: "كفتة مشوية", name_en: "Grilled kofta", description: "أصابع كفتة اللحمة المبهّرة على الجريل، خفيفة وطعمها مدخّن.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 20, cook: 20, servings: 4, method: "grilled",
        tags: &["meat", "grilled"], allergens: &[],
        ingredients: &[("مفروم لحمة", 0.5, "kg"), ("بصل", 0.1, "kg"), ("بقدونس", 0.03, "bundle"), ("توابل متنوعة", 0.01, "package"), ("فلفل أسود", 0.003, "kg")],
        steps: &["افرم البصل والبقدونس ناعمًا، وصفّي البصل من الماء الزائد بالضغط عليه في منديل.",
                 "اخلط المفروم مع البصل والبقدونس والملح والفلفل الأسود والبهارات. اعجن 5 دقائق حتى يصبح الخليط متماسكًا.",
                 "غطّ الخليط واتركه في الثلاجة 30 دقيقة ليتكاثف ويسهل تشكيله.",
                 "شكّل أصابع كفتة بسماكة 2 سم حول أسياخ معدنية أو خشبية منقوعة، واضغط جيدًا.",
                 "سخّن الجريل أو الشواية على نار عالية، ادهن الكفتة بقليل زيت، واشوِها 4–5 دقائق لكل جانب حتى تتحمّر وتنضج تمامًا.",
                 "قدّمها مع أرز أو خبز وسلطة وطحينة. لو تحب يمكن شويها في فرن 220°م لمدة 15–18 دقيقة."],
    },
    SeedRecipeDef {
        name_ar: "لوبيا باللحمة", name_en: "Cowpeas with beef", description: "طبيخ لوبيا بالصلصة واللحمة، وجبة مصرية بيتية مشبعة.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 15, cook: 40, servings: 4, method: "stewed",
        tags: &["legume", "meat", "economical"], allergens: &[],
        ingredients: &[("لوبيا", 0.4, "kg"), ("لحم", 0.3, "kg"), ("طماطم", 0.4, "kg"), ("بصل", 0.1, "kg"), ("توم", 0.02, "kg"), ("زيت طعام", 0.05, "l")],
        steps: &["انقع اللوبيا الجافة 4–6 ساعات، أو استخدم مجمدة أو معلّبة. اسلقها 15–20 دقيقة حتى تنضج نصف نضج ثم صفّها.",
                 "قطّع اللحم مكعبات وشوّحه في الزيت حتى يتحمّر. أضف البصل والثوم وقلّب 2–3 دقائق.",
                 "أضف الطماطم المبشورة والملح والفلفل الأسود والكمون ونصف كوب ماء. اتركه 20 دقيقة على نار هادئة حتى يلين اللحم.",
                 "أضف اللوبيا المسلوقة وقلّب، واطبخ 15 دقيقة أخرى حتى تتشرّب الصلصة وتنضج تمامًا. لو جفت زوّد ماءً ساخنًا.",
                 "قدّمها مع أرز أبيض وخبز. تحفظ في الثلاجة 3 أيام."],
    },
    SeedRecipeDef {
        name_ar: "صينية بطاطس باللحمة", name_en: "Beef and potato tray", description: "بطاطس وشرائح لحم في صينية واحدة بالفرن، سهلة واقتصادية.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 20, cook: 50, servings: 5, method: "baked",
        tags: &["meat", "tray"], allergens: &[],
        ingredients: &[("بطاطس", 1.0, "kg"), ("لحم", 0.4, "kg"), ("طماطم", 0.4, "kg"), ("بصل", 0.15, "kg"), ("زيت طعام", 0.05, "l"), ("توابل متنوعة", 0.01, "package")],
        steps: &["سخّن الفرن على 190°م. قطّع البطاطس حلقات بسماكة 1 سم، واللحم مكعبات صغيرة، والبصل شرائح.",
                 "شوّح اللحم في ملعقة زيت حتى يتحمّر، أضف البصل والطماطم والتوابل والملح، واتركه 15 دقيقة مع نصف كوب ماء.",
                 "رتّب في صينية طبقة بطاطس ثم خليط اللحم ثم طبقة بطاطس. اسكب باقي الصلصة وكوب ماء ساخن.",
                 "غطِ الصينية بورق فويل واخبزها 30 دقيقة حتى تبدأ البطاطس في النضج.",
                 "ارفع الفويل واخبز 15–20 دقيقة حتى يتحمّر الوجه وتنضج البطاطس تمامًا. اتركها 10 دقائق قبل التقديم.",
                 "قدّمها مع سلطة خضراء أو خبز. تكفي 5 أشخاص."],
    },
    SeedRecipeDef {
        name_ar: "كبدة إسكندراني", name_en: "Alexandrian liver", description: "كبدة حارة بالفلفل والثوم والليمون على الطريقة الإسكندرانية.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 15, cook: 15, servings: 4, method: "fried",
        tags: &["meat", "quick", "spicy"], allergens: &[],
        ingredients: &[("كبدة إسكندراني", 0.5, "kg"), ("فلفل أخضر", 0.1, "kg"), ("شطة", 0.01, "kg"), ("توم", 0.03, "kg"), ("ليمون", 0.05, "kg"), ("زيت طعام", 0.05, "l")],
        steps: &["اغسل الكبدة وصفّها جيدًا، وقطّعها قطعًا متوسطة. أزل الأغشية الزائدة إن وجدت.",
                 "سخّن الزيت في طاسة واسعة على نار عالية جدًا. أضف الكبدة في طبقة واحدة واتركها دقيقة دون تقليب حتى تأخذ لونًا.",
                 "قلّب الكبدة 3–4 دقائق حتى تنضج مع بقائها طرية. لا تطبخها طويلًا حتى لا تقسو.",
                 "أضف الثوم المفروم والفلفل الأخضر المقطع والشطة، وقلّب دقيقة واحدة فقط.",
                 "أضف عصير ليمون طازج ورشة ملح وفلفل أسود وقلّب. قدّمها فورًا مع خبز بلدي وسلطة أو أرز."],
    },
    SeedRecipeDef {
        name_ar: "سمك سنجاري", name_en: "Singari fish", description: "سمك مخبوز بالطماطم والفلفل والليمون، طبق بحري مصري شهي.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 25, cook: 30, servings: 4, method: "baked",
        tags: &["seafood", "healthy"], allergens: &[],
        ingredients: &[("سمك", 0.8, "kg"), ("طماطم", 0.4, "kg"), ("بصل", 0.2, "kg"), ("فلفل أخضر", 0.1, "kg"), ("ليمون", 0.1, "kg"), ("توم", 0.03, "kg"), ("زيت طعام", 0.05, "l")],
        steps: &["نظّف السمك جيدًا أو استخدم فيليه، اغسله بالليمون والملح، واتركه 15 دقيقة ثم جففه.",
                 "تبّل السمك بالثوم المدقوق والكمون والفلفل الأسود والملح، وادهنه بقليل زيت. اتركه 20 دقيقة.",
                 "قطّع البصل والطماطم والفلفل حلقات. رتّب نصف الخضار في صينية، ضع السمك فوقه، ثم باقي الخضار.",
                 "اسكب عصير ليمون وملعقتين زيت وكوب ماء ساخن حول السمك. غطِ الصينية بورق فويل.",
                 "اخبز في فرن 190°م لمدة 20 دقيقة، ثم ارفع الفويل واخبز 10 دقائق حتى يتحمّر الوجه وينضج السمك (يتفتت بالشوكة).",
                 "قدّمه مع أرز بالشعرية أو خبز وسلطة. رشّ بقدونس وليمون قبل التقديم."],
    },
    SeedRecipeDef {
        name_ar: "محشي كرومب", name_en: "Stuffed cabbage", description: "ورق كرومب محشي بالأرز واللحمة المفرومة، طبق مصري تقليدي.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 40, cook: 45, servings: 6, method: "stewed",
        tags: &["traditional", "meat"], allergens: &[],
        ingredients: &[("كرومب", 1.0, "kg"), ("أرز", 0.3, "kg"), ("مفروم لحمة", 0.2, "kg"), ("طماطم", 0.3, "kg"), ("بصل", 0.1, "kg"), ("توابل متنوعة", 0.01, "package"), ("زيت طعام", 0.05, "l")],
        steps: &["اسلق أوراق الكرومب كاملة في ماء مغلي 3–5 دقائق حتى تلين وتصبح مرنة، ثم صفّها وجففها. احتفظ بأكبر الأوراق للحشو.",
                 "اغسل الأرز وانقعه 15 دقيقة وصفّه. اخلطه مع المفروم والبصل المفروم والطماطم المبشورة والملح والفلفل الأسود والبهارات وملعقة زيت.",
                 "احشِ كل ورقة بملعقة كبيرة من الخليط، ولفّها بإحكام مع طي الجوانب. لا تحشِ كثيرًا حتى لا يتمزق الورق أثناء الطهي.",
                 "رتّب اللفائف في حلة واسعة بإحكام، وضع فوقها طبقًا ثقيلًا لمنعها من الحركة.",
                 "اسكب شوربة أو ماء مملحًا مع عصير ليمون حتى يغطي المحشي تقريبًا. اغلِ ثم خفّف النار واطبخ 35–45 دقيقة حتى ينضج الأرز.",
                 "اتركه 10 دقائق ثم قدّمه دافئًا مع الزبادي. تحفظ بقية اللفائف في الثلاجة 3 أيام."],
    },
    SeedRecipeDef {
        name_ar: "فتة لحمة", name_en: "Beef fatta", description: "فتة الخبز المحمّص مع اللحم والأرز والصلصة، طبق عزائم مصري.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 25, cook: 45, servings: 5, method: "stewed",
        tags: &["traditional", "meat"], allergens: &["جلوتين", "لبن"],
        ingredients: &[("لحم", 0.5, "kg"), ("خبز بلدي", 4.0, "piece"), ("أرز", 0.3, "kg"), ("طماطم", 0.3, "kg"), ("خل", 0.05, "l"), ("توم", 0.02, "kg"), ("سمنة", 0.03, "kg")],
        steps: &["اسلق اللحم في ماء مع بصل وورق لورا وملح 40–50 دقيقة حتى ينضج، واحتفظ بالشوربة.",
                 "اطبخ الأرز بالطريقة المعتادة واتركه دافئًا. قطّع الخبز مكعبات وحمّصه في الفرن حتى يصبح مقرمشًا.",
                 "للصلصة: شوّح الثوم في السمنة، أضف الطماطم المبشورة والخل والكمون والملح، واتركها 10 دقائق. أضف قليلًا من شوربة اللحم.",
                 "في طبق عميق، رتّب الخبز المحمّص، اسكب عليه قليلًا من الشوربة الساخنة، ثم الأرز ثم اللحم.",
                 "اسكب الصلصة فوق الفتة، ووزّع السمنة الساخنة على الوجه. قدّمها فورًا مع الزبادي والسلطة."],
    },
    SeedRecipeDef {
        name_ar: "بسبوسة", name_en: "Basbousa", description: "حلوى السميد المصرية بالشربات وجوز الهند، سهلة واقتصادية.",
        meal_types: &["dessert"], difficulty: "easy", prep: 15, cook: 35, servings: 8, method: "baked",
        tags: &["dessert", "sweet"], allergens: &["جلوتين", "لبن"],
        ingredients: &[("سميد", 0.4, "kg"), ("سكر", 0.2, "kg"), ("لبن", 0.25, "l"), ("سمنة", 0.15, "kg"), ("جوز هند", 0.05, "package"), ("فانيليا", 0.005, "package")],
        steps: &["سخّن الفرن على 180°م وادهن صينية مستطيلة بالسمنة.",
                 "في وعاء، اخلط السميد والسكر وجوز الهند والفانيليا. أضف اللبن والسمنة المذابة وقلّب حتى يتجانس الخليط دون عجن كثير.",
                 "اترك الخليط 10 دقائق ليتشرّب السميد السوائل، ثم اسكبه في الصينية وافرد السطح.",
                 "اخبزها 30–35 دقيقة حتى يتحمّر الوجه وتتماسك. لا تتركها تجف أكثر من اللازم.",
                 "للشربات: اغلِ كوب سكر مع كوب ماء 5 دقائق، أضف عصير ليمون. اسكب الشربات الدافئ على البسبوسة وهي ساخنة.",
                 "اتركها تتشرّب 30 دقيقة، ثم قطّعها ورشّ مكسرات. تحفظ 3–4 أيام في الثلاجة."],
    },
    SeedRecipeDef {
        name_ar: "أم علي", name_en: "Om Ali", description: "حلوى مصرية بالجلاش واللبن والمكسرات، تُخبز في صينية واحدة.",
        meal_types: &["dessert"], difficulty: "medium", prep: 20, cook: 30, servings: 6, method: "baked",
        tags: &["dessert", "traditional"], allergens: &["جلوتين", "لبن", "مكسرات"],
        ingredients: &[("جلاش النحراوي", 0.2, "package"), ("لبن", 0.75, "l"), ("سكر", 0.15, "kg"), ("مكسرات مشكلة", 0.1, "kg"), ("جوز هند", 0.05, "package"), ("زبدة كريستال", 0.05, "package")],
        steps: &["سخّن الفرن على 190°م. اكسر الجلاش قطعًا صغيرة، ووزّعه في صينية مدهونة.",
                 "حمّص الجلاش في الفرن 8–10 دقائق حتى يصبح ذهبيًا ومقرمشًا، مع التقليب مرة.",
                 "سخّن اللبن مع السكر والفانيليا على نار هادئة حتى يغلي بلطف. لا تدعه يفور.",
                 "وزّع المكسرات وجوز الهند على الجلاش، ثم اسكب اللبن الساخن فوقه. اتركه 10 دقائق ليتشرّب.",
                 "وزّع قطع الزبدة على الوجه، واخبزه 15–20 دقيقة حتى يتحمّر الوجه ويتكاثف اللبن.",
                 "قدّمه دافئًا أو باردًا. يمكن تحضيره قبلها بيوم وحفظه في الثلاجة."],
    },
    SeedRecipeDef {
        name_ar: "كنافة", name_en: "Kunafa", description: "كنافة بالشربات والمكسرات، حلوى رمضان المصرية الأشهر.",
        meal_types: &["dessert"], difficulty: "medium", prep: 20, cook: 30, servings: 8, method: "baked",
        tags: &["dessert", "traditional"], allergens: &["جلوتين", "لبن", "مكسرات"],
        ingredients: &[("كنافة", 0.5, "kg"), ("سكر", 0.25, "kg"), ("سمنة", 0.15, "kg"), ("مكسرات مشكلة", 0.15, "kg"), ("لبن", 0.2, "l")],
        steps: &["لو الكنافة مجمدة اتركها تفك في الثلاجة. افركها بأصابعك مع السمنة المذابة حتى تتغطى كل الخيوط.",
                 "ادهن صينية جيدًا، وافرد نصف الكنافة مع الضغط الخفيف. وزّع المكسرات فوقها، ثم غطّها بباقي الكنافة واضغط.",
                 "اخبزها في فرن 180°م لمدة 25–30 دقيقة حتى تصبح ذهبية من الأسفل. ثم اقلبها على طبق باستخدام صينية أخرى.",
                 "للشربات: اغلِ كوبين سكر مع كوب ماء وملعقة ليمون 5–7 دقائق حتى يثخن قليلًا.",
                 "اسكب الشربات الدافئ على الكنافة الساخنة تدريجيًا، واتركها تتشرّب 15–20 دقيقة.",
                 "قدّمها دافئة مع مكسرات إضافية. تكفي 8 أشخاص."],
    },
    SeedRecipeDef {
        name_ar: "قطايف", name_en: "Qatayef", description: "أقراص قطايف محشوة بالمكسرات، تُقلى أو تُخبز وتُغمس في الشربات.",
        meal_types: &["dessert"], difficulty: "medium", prep: 25, cook: 25, servings: 6, method: "fried",
        tags: &["dessert", "traditional"], allergens: &["جلوتين", "مكسرات"],
        ingredients: &[("قطايف", 0.5, "package"), ("سكر", 0.2, "kg"), ("مكسرات مشكلة", 0.15, "kg"), ("زبدة كريستال", 0.05, "package"), ("لبن", 0.2, "l")],
        steps: &["افتح أقراص القطايف وامسح سطحها الداخلي بقليل من الزبدة المذابة. لو جاهزة ومقفولة اتركها كما هي.",
                 "اخلط المكسرات مع ملعقتين سكر ورشة قرفة. ضع ملعقة صغيرة في كل قرص، ثم اقفله على شكل نصف دائرة واضغط الحواف جيدًا.",
                 "سخّن زيتًا غزيرًا على نار متوسطة (170°م)، واقلي القطايف 2–3 دقائق لكل جانب حتى تصبح ذهبية. صفّها على ورق مطبخ.",
                 "أو اخبزها في فرن 200°م على صينية مدهونة 12–15 دقيقة مع دهن الوجه بالزبدة.",
                 "اغلِ كوب سكر مع نصف كوب ماء وعصير ليمون 5 دقائق، واغمس القطايف في الشربات الساخن ثوانٍ ثم صفّها.",
                 "قدّمها دافئة. تحفظ غير المقلية في الفريزر حتى شهر."],
    },
    SeedRecipeDef {
        name_ar: "مهلبية", name_en: "Mahalabia", description: "حلوى لبن باردة بالنشا، خفيفة وسريعة.",
        meal_types: &["dessert"], difficulty: "easy", prep: 10, cook: 15, servings: 6, method: "boiled",
        tags: &["dessert", "quick"], allergens: &["لبن", "مكسرات"],
        ingredients: &[("لبن", 0.75, "l"), ("سكر", 0.15, "kg"), ("نشا", 0.05, "package"), ("فانيليا", 0.005, "package"), ("مكسرات مشكلة", 0.05, "kg")],
        steps: &["اخلط النشا مع نصف كوب من اللبن البارد في كوب حتى يذوب تمامًا دون تكتلات.",
                 "سخّن باقي اللبن مع السكر على نار متوسطة، وقلّب حتى يذوب السكر قبل أن يغلي.",
                 "أضف خليط النشا تدريجيًا مع التحريك المستمر بمضرب. استمر في التحريك 5–7 دقائق حتى يثخن القوام ويصبح مثل الكريم.",
                 "أضف الفانيليا وارفع عن النار. اسكب فورًا في أكواب أو قالب مبلل بالماء.",
                 "اتركها تبرد في الغرفة ثم غطّها وضعها في الثلاجة ساعتين على الأقل حتى تتماسك.",
                 "قدّمها باردة مع مكسرات محمّصة أو قرفة. تحفظ في الثلاجة يومين."],
    },
    SeedRecipeDef {
        name_ar: "فطير مشلتت", name_en: "Feteer meshaltet", description: "فطير مصري طبقات بالسمنة، يُقدّم مع العسل أو الجبنة.",
        meal_types: &["breakfast", "lunch"], difficulty: "medium", prep: 30, cook: 25, servings: 4, method: "baked",
        tags: &["traditional", "pastry"], allergens: &["جلوتين", "لبن"],
        ingredients: &[("دقيق", 0.5, "kg"), ("سمنة", 0.15, "kg"), ("لبن", 0.2, "l"), ("سكر", 0.02, "kg")],
        steps: &["اخلط الدقيق والملح والسكر، أضف الماء أو اللبن تدريجيًا واعجن 8–10 دقائق حتى تتكون عجينة ناعمة مرنة. غطّها واتركها ترتاح 30 دقيقة.",
                 "قسّم العجينة كرات بحجم البيضة، وادهنها بالسمنة، واتركها 10 دقائق.",
                 "افرد كل كرة على سطح مدهون حتى تصبح رقيقة جدًا. ادهن سطحها بالسمنة واطوِها مربعًا أو لفّها، ثم افردها قليلًا.",
                 "سخّن طاسة ثقيلة أو صاج على نار متوسطة. اطبخ الفطير 3–4 دقائق لكل جانب حتى ينتفخ وتظهر بقع ذهبية.",
                 "لو تحب الفطير المشلتت بالفرن: رصّ الفطائر في صينية، ادهن الوجه بالسمنة، واخبز 200°م لمدة 15–20 دقيقة حتى يتحمّر.",
                 "قدّمه ساخنًا مع العسل والزبدة أو الجبنة البيضاء والشاي. يفضّل تناوله في نفس اليوم."],
    },
    // ----- Quick homemade basics (side components, not full planned meals) -----
    // meal_types is deliberately empty so the planner never picks these as a
    // breakfast/lunch/dinner on their own; they complete a meal on the home table.
    SeedRecipeDef {
        name_ar: "خبز بلدي سريع", name_en: "Quick baladi bread", description: "خبز بلدي طري وسريع من مطبخك، بمرة تخمير واحدة قصيرة.",
        meal_types: &[], difficulty: "easy", prep: 15, cook: 10, servings: 6, method: "baked",
        tags: &["component", "bread", "basic", "quick"], allergens: &["جلوتين"],
        ingredients: &[("دقيق", 0.5, "kg"), ("خميرة", 1.0, "package"), ("ملح", 0.01, "kg"), ("سكر", 0.01, "kg"), ("زيت طعام", 0.02, "l")],
        steps: &["في وعاء كبير اخلط الدقيق والملح. في كوب صغير اذبّ الخميرة والسكر في نصف كوب ماء دافئ واتركه 5 دقائق حتى تظهر رغوة — لو الخميرة قديمة أضف رشة سكر إضافية.",
                 "أضف خليط الخميرة والزيت إلى الدقيق، ثم أضف الماء الدافئ تدريجيًا (حوالي كوب ونصف) واعجن 5–7 دقائق حتى تتكون عجينة لينة لا تلتصق باليد. غطّها واتركها 20–30 دقيقة في مكان دافئ، وهذه هي المرة الوحيدة للترتاح.",
                 "سخّن الفرن على أعلى درجة (250°م) وضع صينية معدنية فارغة بالداخل 10 دقائق. قسّم العجينة 6–8 كرات، وافرد كل كرة على سطح مرشوش بالدقيق حتى تصبح رفيعة (نصف سم).",
                 "أخرج الصينية الساخنة بحرص، رشّها قليلًا من الدقيق، وضع قرصًا أو قرصين واخبزهما 3–4 دقائق حتى ينتفخا، ثم اقلبهما 2–3 دقائق حتى يتحمّرا. كرّر مع باقي الأقراص.",
                 "غطّ الخبز بمنشفة نظيفة فور خروجه ليبقى طريًا. قدّمه ساخنًا مع أي أدام. يبقى طريًا يومين في كيس محكم، ويُجمّد حتى شهر."],
    },
    SeedRecipeDef {
        name_ar: "أرز أبيض", name_en: "Plain white rice", description: "أرز أبيض مفلفل بطريقة مضمونة لكل يوم.",
        meal_types: &[], difficulty: "easy", prep: 5, cook: 20, servings: 4, method: "boiled",
        tags: &["component", "rice", "basic", "quick"], allergens: &[],
        ingredients: &[("أرز", 0.4, "kg"), ("ملح", 0.005, "kg"), ("زيت طعام", 0.02, "l")],
        steps: &["اغسل الأرز 3–4 مرات حتى يصفو الماء، ثم انقعه 15 دقيقة وصفّه. هذه الخطوة تمنع الأرز من التكتل.",
                 "سخّن الزيت في حلة، أضف الأرز وقلّب دقيقتين حتى يتغطى بالزيت وتظهر حباته لامعة.",
                 "أضف ماءً ساخنًا بنسبة 1 كوب أرز إلى 1.5 كوب ماء، ثم الملح. اغلِ بقوة دقيقة واحدة.",
                 "خفّف النار لأقل درجة، غطِ الحلة بإحكام واتركها 15–18 دقيقة دون تحريك. لا تفتح الغطاء قبل الوقت.",
                 "ارفع الحلة عن النار واتركها 5 دقائق مغطاة، ثم حرّك الأرز بالشوكة برفق. قدّمه مع أي طبق رئيسي وسلطة."],
    },
    SeedRecipeDef {
        name_ar: "سلطة خضراء", name_en: "Green salad", description: "سلطة خضراء منعشة وسريعة بصلصة الليمون والزيت.",
        meal_types: &[], difficulty: "easy", prep: 10, cook: 0, servings: 4, method: "raw",
        tags: &["component", "salad", "basic", "quick"], allergens: &[],
        ingredients: &[("خيار بلدي", 0.3, "kg"), ("طماطم", 0.3, "kg"), ("بصل", 0.05, "kg"), ("ليمون", 0.05, "kg"), ("زيت طعام", 0.02, "l"), ("ملح", 0.003, "kg")],
        steps: &["اغسل الخيار والطماطم جيدًا. قطّع الخيار مكعبات أو شرائح، والطماطم مكعبات متوسطة، والبصل شرائح رفيعة جدًا.",
                 "في كوب صغير اخلط عصير الليمون والزيت والملح ورشة فلفل أسود إن رغبت. تذوّق الصلصة واضبطها.",
                 "ضع الخضار في طبق التقديم واسكب الصلصة فوقها قبل التقديم مباشرة حتى تبقى مقرمشة.",
                 "أضف خسًا أو جرجيرًا أو نعناعًا لو متوفر، ورشّة كمون اختيارية. قدّمها فورًا مع أي وجبة."],
    },
    SeedRecipeDef {
        name_ar: "جبنة بيضاء بالزيت", name_en: "White cheese with oil", description: "أدام سريع من الجبنة البيضاء بالزيت والطماطم.",
        meal_types: &[], difficulty: "easy", prep: 5, cook: 0, servings: 3, method: "raw",
        tags: &["component", "adama", "basic", "quick"], allergens: &["لبن"],
        ingredients: &[("جبنة بيضاء", 0.2, "kg"), ("طماطم", 0.1, "kg"), ("زيت طعام", 0.02, "l"), ("كمون", 0.002, "kg")],
        steps: &["لو الجبنة مالحة جدًا، انقعها في ماء بارد 10 دقائق ثم صفّها وجففها.",
                 "قطّع الجبنة مكعبات صغيرة أو اهرسها بالشوكة، وقطّع الطماطم مكعبات صغيرة.",
                 "اخلط الجبنة والطماطم في طبق، واسكب زيت الزيتون أو الزيت العادي ورشّة كمون.",
                 "قدّمها فورًا مع الخبز البلدي الساخن وبعض الخضار الطازجة مثل الخيار."],
    },
    SeedRecipeDef {
        name_ar: "بيض مسلوق", name_en: "Boiled eggs", description: "بيض مسلوق سهل وسريع لأي فطور أو أدام.",
        meal_types: &[], difficulty: "easy", prep: 2, cook: 10, servings: 2, method: "boiled",
        tags: &["component", "adama", "basic", "quick"], allergens: &["بيض"],
        ingredients: &[("بيض", 2.0, "piece"), ("ملح", 0.002, "kg")],
        steps: &["ضع البيض في حلة صغيرة وغطّه بماء بارد بفارق 2 سم. أضف رشة ملح حتى لا يتشقق أثناء السلق.",
                 "ارفع الحلة على نار متوسطة حتى يغلي الماء بقوة، ثم خفّف النار واتركه 8–10 دقائق حسب رغبتك.",
                 "انقل البيض فورًا إلى ماء بارد 2–3 دقائق، ثم قشّره بسهولة. القشر تحت ماء جارٍ يسهّل التقشير.",
                 "قطّع البيض أنصافًا أو شرائح، ورشّ عليه الملح والفلفل. قدّمه مع الخبز والجبنة أو السلطة."],
    },
    SeedRecipeDef {
        name_ar: "زبادي بالخيار", name_en: "Cucumber yogurt", description: "أدام بارد خفيف من الزبادي والخيار والنعناع.",
        meal_types: &[], difficulty: "easy", prep: 8, cook: 0, servings: 3, method: "raw",
        tags: &["component", "adama", "basic", "quick"], allergens: &["لبن"],
        ingredients: &[("لبن", 0.4, "l"), ("خيار بلدي", 0.1, "kg"), ("نعناع", 0.5, "bundle"), ("ملح", 0.002, "kg")],
        steps: &["ابشر الخيار أو قطّعه مكعبات صغيرة جدًا، واعصره بيدك للتخلص من الماء الزائد حتى لا يسيّل الزبادي.",
                 "اخلط الزبادي مع الخيار والملح حتى يتجانس. لو تحب قوامًا أخف أضف ملعقتين ماء بارد.",
                 "أضف أوراق النعناع المفرومة أو المجففة وقلّب. تذوّق الملح.",
                 "اتركه في الثلاجة 10 دقائق ليبرد، وقدّمه مع الخبز أو الأرز أو أي طبق رئيسي."],
    },
    SeedRecipeDef {
        name_ar: "سناك فواكه", name_en: "Fruit snack", description: "طبق فواكه سريع بين الفطور والغداء أو بين الغداء والعشاء.",
        meal_types: &[], difficulty: "easy", prep: 5, cook: 0, servings: 3, method: "raw",
        tags: &["component", "fruit_snack", "snack", "basic", "quick"], allergens: &[],
        ingredients: &[("موز", 0.3, "kg"), ("تفاح", 0.3, "kg"), ("برتقال", 0.3, "kg")],
        steps: &["اغسل الفواكه جيدًا. استخدم أي فاكهة موسمية متوفرة عندك بدلًا من المذكورة.",
                 "قشّر الموز والبرتقال، وقطّع التفاح والموز شرائح. رشّ قليلًا من عصير الليمون على التفاح حتى لا يسود.",
                 "رتّب الفواكه في طبق صغير أو وزّعها في أكواب للأطفال.",
                 "قدّمها فورًا، أو غطّها وضعها في الثلاجة حتى ساعتين. لا تضيف سكرًا حتى تبقى خفيفة."],
    },
    SeedRecipeDef {
        name_ar: "سناك خضار", name_en: "Vegetable snack", description: "أعواد خضار طازجة ومقرمشة بين الوجبات.",
        meal_types: &[], difficulty: "easy", prep: 7, cook: 0, servings: 3, method: "raw",
        tags: &["component", "veg_snack", "snack", "basic", "quick"], allergens: &[],
        ingredients: &[("جزر", 0.2, "kg"), ("خيار بلدي", 0.2, "kg"), ("خس", 0.5, "piece")],
        steps: &["اغسل الجزر والخيار والخس جيدًا. قشّر الجزر إن رغبت.",
                 "قطّع الجزر والخيار أعوادًا بطول الإصبع، وافصل أوراق الخس.",
                 "رتّب الخضار في طبق أو علبة. قدّمها وحدها أو مع غموس زبادي بالليمون والنعناع.",
                 "احفظها في الثلاجة في علبة محكمة حتى يومين، واغسلها قبل التقديم لو تُركت مدة."],
    },
];

/// User-supplied editable baseline estimates (EGP minor units).
pub const BASELINES: &[(&str, i64, &str, &str)] = &[
    ("دقيق", 3000, "kg", "Flour baseline"),
    ("أرز", 3000, "kg", "Rice baseline"),
    ("لحم", 45000, "kg", "Meat baseline"),
    ("سمك", 15000, "kg", "Fish baseline"),
    ("بيض", 13000, "carton", "Eggs: carton of 30"),
    ("سكر", 3000, "kg", "Sugar baseline"),
    ("زيت طعام", 10000, "l", "Cooking oil baseline"),
];

pub struct BatchGuideDef {
    pub title_ar: &'static str,
    pub title_en: &'static str,
    pub category: &'static str,
    pub batch_size_text: &'static str,
    pub ingredients: &'static str,   // JSON
    pub steps: &'static str,         // JSON
    pub portions: i64,
    pub storage: &'static str,       // JSON
}

pub const BATCH_GUIDES: &[BatchGuideDef] = &[
    BatchGuideDef {
        title_ar: "تحضير عجينة طعمية لكمية كبيرة", title_en: "Batch taameya mixture",
        category: "taameya", batch_size_text: "2 كجم فول منقوع",
        ingredients: r#"[{"name_ar":"فول","quantity":2,"unit":"kg"},{"name_ar":"بقدونس","quantity":0.2,"unit":"kg"},{"name_ar":"كزبرة خضراء","quantity":0.2,"unit":"kg"},{"name_ar":"توم","quantity":0.1,"unit":"kg"},{"name_ar":"بصل","quantity":0.3,"unit":"kg"}]"#,
        steps: r#"[{"text_ar":"اطحن كل المكونات حتى يصبح الخليط ناعماً."},{"text_ar":"قسّم الخليط في أكياس مسطحة بوزن وجبة لكل كيس."},{"text_ar":"جمّد ما سيؤكل بعد 3 أيام."}]"#,
        portions: 20,
        storage: r#"{"container":"أكياس محكمة مسطحة","refrigerate":"حتى 2-3 أيام في الثلاجة","freeze":"حتى شهر إلى شهرين في الفريزر","defrost":"انقله للثلاجة من الليل","reheat":"اقلي أو اخبز بعد إذابة كاملة"}"#,
    },
    BatchGuideDef {
        title_ar: "تحضير الفول لعدة فطور", title_en: "Batch beans for breakfasts",
        category: "beans", batch_size_text: "1 كجم فول جاف",
        ingredients: r#"[{"name_ar":"فول","quantity":1,"unit":"kg"}]"#,
        steps: r#"[{"text_ar":"اسلق كمية كبيرة واتركها ناعمة."},{"text_ar":"قسّمها حصصاً بمقدار فطور واحد."},{"text_ar":"برّد الحصص بسرعة قبل التخزين."}]"#,
        portions: 8,
        storage: r#"{"container":"علب محكمة","refrigerate":"3-4 أيام في الثلاجة","freeze":"حتى شهرين في الفريزر","defrost":"في الثلاجة طوال الليل","reheat":"سخّن جيداً حتى الغليان قبل الأكل"}"#,
    },
    BatchGuideDef {
        title_ar: "صلصة طماطم لعدة أطباق", title_en: "Batch tomato sauce",
        category: "sauce", batch_size_text: "2 كجم طماطم",
        ingredients: r#"[{"name_ar":"طماطم","quantity":2,"unit":"kg"},{"name_ar":"توم","quantity":0.05,"unit":"kg"},{"name_ar":"زيت طعام","quantity":0.15,"unit":"l"}]"#,
        steps: r#"[{"text_ar":"اعصر الطماطم وصفّها."},{"text_ar":"اطبخها مع الثوم والزيت حتى تتكثف."},{"text_ar":"قسّمها في عبوات بحجم طبق."}]"#,
        portions: 8,
        storage: r#"{"container":"عبوات زجاجية محكمة","refrigerate":"3-4 أيام","freeze":"حتى 3 أشهر","defrost":"في الثلاجة أو على نار هادئة مباشرة","reheat":"سخّن حتى الغليان"}"#,
    },
    BatchGuideDef {
        title_ar: "تقطيع وتحضير خضار الأسبوع", title_en: "Weekly vegetable prep",
        category: "vegetables", batch_size_text: "خضار الأسبوع",
        ingredients: r#"[{"name_ar":"خضار موسمية","quantity":0,"unit":"kg"}]"#,
        steps: r#"[{"text_ar":"اغسل الخضار وجففها جيداً قبل التخزين."},{"text_ar":"قطّع ما سيؤكل خلال يومين فقط."},{"text_ar":"الخضار المقطعة تفقد جودتها أسرع — قطّع على دفعات."}]"#,
        portions: 7,
        storage: r#"{"container":"علب محكمة مع مناديل ورقية","refrigerate":"الخضار المقطعة 1-2 يوم، الكاملة أطول","freeze":"البازلاء والجزر والسبانخ تصلح للتجميد بعد السلق","defrost":"مباشرة في الطبخ","reheat":null}"#,
    },
    BatchGuideDef {
        title_ar: "مرقة الدجاج والاستفادة منها", title_en: "Chicken stock reuse",
        category: "stock", batch_size_text: "2 كجم عظام/فراخ",
        ingredients: r#"[{"name_ar":"فراخ بيضاء","quantity":1,"unit":"kg"},{"name_ar":"بصل","quantity":0.2,"unit":"kg"},{"name_ar":"جزر","quantity":0.2,"unit":"kg"}]"#,
        steps: r#"[{"text_ar":"اسلق العظام والخضار على نار هادئة ساعتين."},{"text_ar":"صفّ المرقة وقسّمها في علب."},{"text_ar":"استخدمها للملوخية والشوربات والأرز."}]"#,
        portions: 10,
        storage: r#"{"container":"علب أو أكياس محكمة","refrigerate":"3-4 أيام","freeze":"حتى 3 أشهر","defrost":"في الثلاجة","reheat":"حتى الغليان"}"#,
    },
    BatchGuideDef {
        title_ar: "مكونات صالحة للتجميد في الفريزر", title_en: "Freezer-friendly components",
        category: "freezer", batch_size_text: "حسب الكمية",
        ingredients: r#"[{"name_ar":"متفرقات","quantity":0,"unit":"kg"}]"#,
        steps: r#"[{"text_ar":"البان والشاورما والدبابيس تُجمّد كما هي من العبوة أو بعد التوزيع."},{"text_ar":"اكتب تاريخ التجميد على كل عبوة."},{"text_ar":"رتب الفريزر: الأقدم في المقدمة (FEFO)."}]"#,
        portions: 0,
        storage: r#"{"container":"أكياس تفريغ هواء أو علب محكمة","refrigerate":null,"freeze":"الدجاج الخام 9-12 شهراً، المفروم 3-4 أشهر، المطبخ 2-3 أشهر","defrost":"في الثلاجة وليس في حرارة الغرفة","reheat":"مرة واحدة فقط وبشكل كامل"}"#,
    },
    BatchGuideDef {
        title_ar: "الاستخدام الآمن للفطائر المتبقية", title_en: "Safe leftover reuse",
        category: "leftovers", batch_size_text: "حسب المتبقي",
        ingredients: r#"[{"name_ar":"متفرقات","quantity":0,"unit":"kg"}]"#,
        steps: r#"[{"text_ar":"برّد الأكل المتبقي في ساعتين بعد الطبخ."},{"text_ar":"استخدم المتبقي في وجبات مختلفة: الأرز يصبح محشي أو مقلي، الفراخ تُستخدم في السندوتشات."},{"text_ar":"لا تعيد تسخين الوجبة أكثر من مرة واحدة."}]"#,
        portions: 0,
        storage: r#"{"container":"علب محكمة صغيرة","refrigerate":"3-4 أيام","freeze":"حسب النوع 1-2 شهر","defrost":"في الثلاجة","reheat":"سخّن حتى الغليان أو 74 درجة مئوية داخل الطعام"}"#,
    },
    BatchGuideDef {
        title_ar: "مقارنة القلي والخبز والسلق والشوي", title_en: "Frying vs baking vs boiling vs grilling",
        category: "cooking_methods", batch_size_text: "دليل مقارنة",
        ingredients: r#"[]"#,
        steps: r#"[{"text_ar":"القلي: أسرع نتيجة مقرمشة لكنه يستهلك زيتاً أكثر وترفع التكلفة."},{"text_ar":"الخبز: يقلل الزيت بشكل كبير والملمس يختلف عن المقلي."},{"text_ar":"السلق: لا يحتاج زيتاً ويحفظ الطعم الطبيعي."},{"text_ar":"الشوي: طعم مشوي بدون دهون إضافية."},{"text_ar":"اختيار الطريقة يؤثر على الميزانية والصحة — لا يوجد بديل مطابق تماماً للقلي."}]"#,
        portions: 0,
        storage: r#"{"container":null,"refrigerate":null,"freeze":null,"defrost":null,"reheat":null}"#,
    },
    BatchGuideDef {
        title_ar: "حلويات اقتصادية منزلية", title_en: "Economical homemade desserts",
        category: "dessert", batch_size_text: "6-8 حصص",
        ingredients: r#"[{"name_ar":"أرز","quantity":0.2,"unit":"kg"},{"name_ar":"سكر","quantity":0.15,"unit":"kg"},{"name_ar":"لبن","quantity":0.5,"unit":"l"}]"#,
        steps: r#"[{"text_ar":"أرز باللبن ومهلبية وقطايف المنزل أرخص بكثير من الجاهزة."},{"text_ar":"اصنع كمية ووزعها في أكواب فردية."},{"text_ar":"قلل السكر تدريجياً لتقليل التكلفة والسعرات."}]"#,
        portions: 8,
        storage: r#"{"container":"أكواب مغطاة","refrigerate":"2-3 أيام","freeze":null,"defrost":null,"reheat":null}"#,
    },
];

/// The user-supplied Olad Zain price post, reconstructed from the transcription
/// tables in the master prompt. Original URL unknown; city/area unknown.
pub const OLAD_ZAIN_POST: &str = r#"محلات أولاد زين للخضروات والفواكة وجميع انواع الطيور الحية والمذبوحة وجميع انواع اللحوم المجمدة والمعجنات والتوابل والبقالة والماركت
أسعارنا لصباح اليوم السبت 14/11/2027
الخضروات والفواكة طازجة يوميا وبأفضل جودة وبأسعار أقل من أي مكان
الخضروات والفواكة
طماطم جامدة جدا 30
طماطم حمراء بجودة عالية 25
فاصوليا خضراء فريش 40
بطاطا سكري حمراء 20
ماشروم طازج فريش الطبق 60
جنزبيل فريش الطبق 50
سبانخ فلاحي فريش 30
بسلة خضراء فريش بشاير 60
خيار بلدي فلاحي 30
بطاطس تحمير فاخر 30
بطاطس وسط للسليق 20
بنجر أحمر سكري الطبق 20
بصل أحمر إيطالي فاخر 30
بصل أصفر ذهبي إيطالي 30
توم أحمر بلدي 30 نص كيلو
ليمون أصفر فاخر 25 نص كيلو
جزر سكري 30
كوسة فلاحي 25
فلفل أخضر بلدي 10 نص كيلو
شطة فلاحي باردة 10 نص كيلو
شطة حمراء فاخر 20 النص
شطة بلدي حارة 25
ملوخية فلاحي 25
فلفل ألوان 30 نص كيلو
جنزبيل فريش 50 الطبق
باذنجان رومي فلاحي 20
باذنجان حشو بلدي 20
كرومب حشو فلاحي
خس كابوتشي فريش
كرومب كابوتشي أحمر
كرومب كابوتشي أبيض
خضار سلطة ومحشي ونعناع وبصل أخضر وجرجير حسب الطلب
ملوخية مقطفة
باذنجان مقور جاهز على الحشو
كوسة مقورة
شوربة خضار جاهزة
الفواكة
برتقال بصرة فاخر 40
جوافة بناتي رشيدي 40
كنتالوب إسماعلاوي فاخر 40
كاكا بشاير فاخر 40
بلح أسمر رطب 40
بلح برحي 100
قشطة عبد الرزاق 150 بدلا من 200
دراجون فروت 250 من أول قطعة
مانجو 70
عنب أحمر كريمسون 70 بدلا من 75
موز بلدي 50
كيوي إيطالي سوبر الطبق 120 بدلا من 150
تفاح أصفر لبناني 100 بدلا من 120
تفاح أحمر سكري إيطالي 100 بدلا من 120
تفاح أخضر جرين للدايت إيطالي 250 بدلا من 300
الطيور الحية والمذبوحة
الفراخ البيضاء 88
البلدي الفلاحي 150
البرابرة الفلاحي 150
الفيومي 145
الساسو 140
البط البلدي 150 جميع الأوزان
بط بغال سوبر 120
بط فرنساوي بيور 100
الرومي البلدي 170
الأرانب 150
الحمام 200 / 250 / 300 / 350 حسب الحجم زوج
المجمدات والمعجنات
البانيه 270
استربس 270
جوردن بلو 270
شاورما 270
الوراك 110
الشيش طاووق 270
الدبابيس 220
الحمام الكداب 220
الأجنحة 85
الكبد والقوانص 120
البقالة
سمنة روابي أو كريستال أو جنة أو هانم 205
زيت 80 اللتر
سكر 2 كيلو كيلو كيلو 55
زيت حلوة وهنادي 100
عدس 60 الكيلو
لوبيا 80 الكيلو
رز 25 الكيلو
رز 30 الكيلو
فيشار 50
حلبة بلدي 60
فول بلدي 60
توابل متنوعة
زبدة فرن
زبدة كريستال 155
جلاش النحراوي
رقاق النحراوي
سمبوسة النحراوي 40
"#;

// -------------------------------------------------------------------------
// Meal tips & tricks — original editorial guidance, no prices or nutrition
// claims. Displayed as practical help next to recipes and the weekly plan.
// -------------------------------------------------------------------------

pub struct MealTipDef {
    pub title_ar: &'static str,
    pub title_en: &'static str,
    pub category: &'static str,
    pub body_ar: &'static str,
    pub body_en: &'static str,
}

macro_rules! tip {
    ($ar:expr, $en:expr, $cat:expr, $body_ar:expr, $body_en:expr) => {
        MealTipDef { title_ar: $ar, title_en: $en, category: $cat, body_ar: $body_ar, body_en: $body_en }
    };
}

pub const MEAL_TIPS: &[MealTipDef] = &[
    tip!("ابدأ بالبقايا قبل الطبخ", "Start with leftovers before cooking", "leftovers",
        "قبل ما تخطط وجبة جديدة، افتح المخزن واستخدم البقايا أولًا. رتب العبوات حسب تاريخ الانتهاء، واجعل القديم في مقدمة الثلاجة.",
        "Before planning a new meal, check the pantry and use leftovers first. Arrange containers by expiry date and keep the oldest at the front of the fridge."),
    tip!("البقايا تنتقل للأسبوع التالي", "Leftovers roll into next week", "leftovers",
        "أي كمية غير مستخدمة تظل في المخزن ولا تختفي نهاية الأسبوع. الخطة والقائمة يحسبانها كمخزون متاح، فلا تشتريها مرتين.",
        "Anything you do not use stays in stock at week end. The planner and shopping list count it as already owned, so you do not buy it twice."),
    tip!("قسّم الطبخة الكبيرة فورًا", "Portion big batches immediately", "batch",
        "بعد الطبخ قسّم الكمية إلى علب حسب عدد الحصص، واتركها تبرد قبل التخزين. وجبة جاهزة في الفريزر توفر وقت وقرارات الأسبوع التالي.",
        "After cooking, split the batch into meal-size containers and let them cool before storing. A frozen ready meal saves time and decisions next week."),
    tip!("اكتب تاريخ على كل عبوة", "Label every container", "batch",
        "ورقة صغيرة عليها اسم الطبخة والتاريخ كافية. بدون تاريخ ستضطر للتخمين، وال تخمين يسبب هدرًا.",
        "A small note with the dish name and date is enough. Without a date you will have to guess, and guessing causes waste."),
    tip!("الأرز: انقعه وصفّه جيدًا", "Rice: soak and drain well", "rice",
        "انقع الأرز 20-30 دقيقة ثم صفّه قبل الطبخ. الكمية المعتادة كوب أرز لكل شخص تقريبًا، ويمكن زيادتها لمن يحتاج حصة أكبر.",
        "Soak rice for 20-30 minutes and drain before cooking. Roughly one cup of dry rice per person is a common starting point; adjust for your household."),
    tip!("الأرز بالخضار يغني عن طبقين", "Rice with vegetables replaces two dishes", "rice",
        "أضف جزر وبصل و بازلاء مبشورة إلى الأرز أثناء الطبخ لتحصل على وجبة متكاملة في قدر واحد وغسيل أقل.",
        "Add grated carrot, onion and peas while the rice cooks for a one-pot meal and less washing up."),
    tip!("لا تغسل الأرز بعد الطبخ", "Do not rinse cooked rice", "rice",
        "غسل الأرز بعد نضجه يذهب بالنكهة والقوام. إن زادت الرطوبة اترك القدر مفتوحًا على نار هادئة دقائق قليلة.",
        "Rinsing cooked rice washes away flavour and texture. If it is too moist, leave the pot uncovered on low heat for a few minutes."),
    tip!("الخبز: أعد تسخينه بدل شراء جديد", "Bread: refresh it instead of buying more", "bread",
        "الخبز البلدي القديم يعود طريًا بتسخينه في فرن ساخن دقائق، أو في مقلاة مغطاة. استخدمه في الفطور والعشاء قبل شراء رغيف جديد.",
        "Stale baladi bread softens in a hot oven for a few minutes or in a covered pan. Use it for breakfast and dinner before buying a fresh loaf."),
    tip!("خبز الفريزر يُحمّص مباشرة", "Frozen bread toasts straight from the freezer", "bread",
        "لا تنتظر ذوبان الخبز المجمد؛ ضعه في المحمصة أو الفرن مباشرة. قسّم الرغيف قبل التخزين لتأخذ ما تحتاجه فقط.",
        "Do not thaw frozen bread first; toast or bake it straight from the freezer. Split the loaf before storing so you only take what you need."),
    tip!("الخبز الزائد → فتات مفيد", "Extra bread becomes useful crumbs", "bread",
        "جفّف الخبز الزائد واطحنه واحتفظ به للقليات أو تحمير الكفتة أو إضافة قوام للشوربة بدل شراء بقسماط.",
        "Dry and grind extra bread for coating fried food, meatballs, or thickening soup instead of buying breadcrumbs."),
    tip!("التوابل تُشترى بمبلغ محدد", "Buy spices by a set amount", "spices",
        "الملح والكمون والفلفل لا تُشترى بالكيلو للبيت الصغير. حدّد مبلغًا مثل 20 جنيهًا للملح واشترِ حسب السعر المعلن، وسجّل السعر والتاريخ.",
        "Salt, cumin and pepper are not bought by the kilo for a small home. Set an amount such as 20 EGP for salt, buy at the observed price, and record the price and date."),
    tip!("قارن سعر العبوة لا سعر القطعة", "Compare pack value, not pack price", "spices",
        "عبوة 30 بيضة بـ130 جنيهًا ليست بالضرورة أغلى من 15 بيضة بـ65. احسب السعر للوحدة قبل اختيار الأرخص.",
        "A pack of 30 eggs for 130 EGP is not automatically dearer than 15 for 65. Work out the price per unit before choosing the cheaper option."),
    tip!("البهارات المحمّصة تعطي نكهة أكبر", "Toasting spices deepens flavour", "spices",
        "حمّص الكمون أو الكزبرة في مقلاة جافة ثوانٍ قبل إضافتها. كمية أقل من البهارات المحمّصة تعطي طعمًا أوضح من كمية كبيرة نيئة.",
        "Toast cumin or coriander in a dry pan for a few seconds before adding. Less toasted spice gives a clearer flavour than a large raw amount."),
    tip!("احفظ التوابل بعيدًا عن الرطوبة", "Keep spices away from moisture", "spices",
        "احفظ الملح والبهارات في برطمانات مغلقة بعيدًا عن البخار فوق البوتاجاز. الرطوبة تكتل الملح وتفسد النكهة.",
        "Store salt and spices in sealed jars away from steam above the stove. Moisture clumps salt and dulls flavour."),
    tip!("نقع البقوليات يختصر الطبخ", "Soak legumes to shorten cooking", "legumes",
        "انقع العدس أو الفول قبل الطبخ لتقليل الوقت وتحسين الهضم. راجع قواعد الأسرة قبل إضافة بقوليات إذا كان هناك تحسس أو استثناء.",
        "Soak lentils or beans before cooking to shorten the time and improve digestibility. Check household rules before adding legumes if there are allergies or exclusions."),
    tip!("طبق بقوليات واحد في الأسبوع على الأقل", "At least one legume meal a week", "legumes",
        "اجعل للبقوليات وجبة ثابتة أسبوعيًا مثل العدس أو الفول أو المجدرة. القاعدة في التطبيق تقترح ذلك ولا تجبرك عليه.",
        "Keep one regular legume meal a week, such as lentils, beans or mujaddara. The app rule suggests this rather than forcing it."),
    tip!("قطّع الخضار مرة واحدة", "Chop vegetables once", "prep",
        "في يوم التحضير قطّع البصل والطماطم والخضار وحفظها في علب. الطبخ في أيام الأسبوع يصبح أسرع ويقل التفكير في كل وجبة.",
        "On a prep day, chop onion, tomato and vegetables and store them in containers. Weeknight cooking becomes faster with fewer decisions."),
    tip!("صلصة الطماطم المبشورة تختصر الوقت", "Grated tomato sauce saves time", "prep",
        "بشر الطماطم واطبخها مع قليل من الزيت والملح، ثم خزّنها. تصبح أساسًا سريعًا للأرز والمعكرونة والشوربة.",
        "Grate tomato and cook it with a little oil and salt, then store it. It becomes a quick base for rice, pasta and soup."),
    tip!("القائمة تُرتّب حسب القسم", "Order the list by shop section", "budget",
        "امشِ في القائمة حسب قسم المتجر: خضار، ثم لحوم، ثم بقالة. تقل النسيان والعودة لنفس الممر.",
        "Follow the list in shop order: produce, meat, then groceries. You forget less and avoid walking the same aisle twice."),
    tip!("قارن قبل الشراء، لا بعده", "Compare before you buy", "budget",
        "سجّل السعر في التطبيق يوم الشراء مع اسم التاجر والتاريخ. ملاحظة واحدة في وقتها تغنيك عن البحث عن السعر الحقيقي لاحقًا.",
        "Record the price in the app on the day you buy it with the merchant and date. One note now saves hunting for the real price later."),
    tip!("لا تثق بسعر قديم", "Do not trust an old price", "budget",
        "السعر ملاحظة مؤرخة وليست حقيقة دائمة. إذا مرّ أكثر من أسبوعين فاعتبره قديمًا وأعد التحقق قبل الاعتماد عليه.",
        "A price is a dated observation, not a permanent fact. If more than two weeks have passed, treat it as stale and check again."),
    tip!("اترك هامشًا صغيرًا في الميزانية", "Leave a small buffer", "budget",
        "اجعل جزءًا صغيرًا من الميزانية احتياطيًا للزيادة غير المتوقعة في سعر منتج واحد بدل إلغاء وجبة كاملة.",
        "Keep a small part of the budget as a buffer for one unexpected price rise rather than cancelling a whole meal."),
    tip!("الفطور والغداء والعشاء بنظام", "A simple rhythm for breakfast, lunch and dinner", "planning",
        "الخبز أساس الفطور والعشاء عند كثير من العائلات، والأرز أساس الغداء. ثبّت الأساس وغيّر الطبق الذي بجانبه لتقل الفوضى والتكلفة.",
        "Bread is a staple at breakfast and dinner for many families, rice at lunch. Fix the staple and vary what sits beside it to cut cost and confusion."),
    tip!("اسأل المساعد ولا تصدّق أرقامه", "Ask the assistant, verify its numbers", "planning",
        "المساعد يقترح ويشرح لكنه لا يخترع الأسعار ولا يعدّل بياناتك. أي تغيير في القواعد أو الميزانية يظهر كاقتراح تحتاج موافقتك.",
        "The assistant suggests and explains but never invents prices or edits your data. Any rule or budget change appears as a proposal you must approve."),
    tip!("اذكر تفضيلاتك للمساعد", "Tell the assistant what matters", "planning",
        "اكتب جملة مثل «لا نحب السمك مرتين في الأسبوع» كقاعدة إرشادية. التطبيق يمررها للمساعد ليأخذها في اعتباره مع بقية القواعد.",
        "Write a sentence such as 'we do not like fish twice a week' as a guidance rule. The app passes it to the assistant alongside your other rules."),
];

