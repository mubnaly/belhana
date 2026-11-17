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
    // essentials
    SeedProductDef { slug: "sugar", category: "essentials", name_ar: "سكر", name_en: Some("Sugar"), default_unit: Some("kg"), aliases: Some(&["سكر أبيض"]) , package_size: None, package_unit: None },
];

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
        steps: &["انقع الفول من الليل إذا كان الفول الجاف، أو استخدم الفول الجاهز المسلوق.",
                 "اسلق الفول حتى يصبح طرياً ثم اهرسه مع الثوم والكمون.",
                 "قدّمه بالليمون والزيت مع الخبز."],
    },
    SeedRecipeDef {
        name_ar: "طعمية", name_en: "Taameya (Falafel)", description: "أقراص الفول المصرية المشهورة، اقتصادية وتصلح للتجميد.",
        meal_types: &["breakfast", "lunch"], difficulty: "medium", prep: 30, cook: 20, servings: 6, method: "fried",
        tags: &["legume", "vegetarian", "economical", "batch_friendly"], allergens: &["سمسم"],
        ingredients: &[("فول", 0.5, "kg"), ("بقدونس", 0.05, "bundle"), ("كزبرة خضراء", 0.05, "bundle"), ("توم", 0.03, "kg"), ("بصل", 0.1, "kg"), ("دقيق", 0.05, "kg"), ("توابل متنوعة", 0.01, "package")],
        steps: &["انقع الفول الجاف 8 ساعات على الأقل.",
                 "اطحن الفول مع البقدونس والكزبرة والبصل والثوم حتى يصبح الخليط ناعماً.",
                 "أضف التوابل والدقيق وشكّل الأقراص ثم اقليها في زيت ساخن أو اخبزها في الفرن.",
                 "نصيحة: اخبزها في الفرن لتقليل كمية الزيت — الملمس يختلف قليلاً عن المقلي."],
    },
    SeedRecipeDef {
        name_ar: "شكشوكة", name_en: "Shakshouka", description: "بيض مطبوخ في صلصة الطماطم، فطور سريع ومغذٍ.",
        meal_types: &["breakfast", "lunch"], difficulty: "easy", prep: 10, cook: 20, servings: 4, method: "stewed",
        tags: &["vegetarian", "quick"], allergens: &["بيض"],
        ingredients: &[("طماطم", 0.5, "kg"), ("بيض", 4.0, "piece"), ("فلفل أخضر", 0.05, "kg"), ("بصل", 0.1, "kg"), ("زيت طعام", 0.05, "l")],
        steps: &["شوّح البصل والفلفل في الزيت ثم أضف الطماطم المقطعة.",
                 "اترك الصلصة تغلي على نار هادئة 10 دقائق.",
                 "اسكب البيض فوق الصلصة وغطِ حتى يتماسك البيض."],
    },
    SeedRecipeDef {
        name_ar: "شوربة عدس", name_en: "Lentil Soup", description: "شوربة اقتصادية دافئة من البقوليات.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 10, cook: 30, servings: 5, method: "boiled",
        tags: &["legume", "vegetarian", "economical"], allergens: &[],
        ingredients: &[("عدس", 0.5, "kg"), ("بصل", 0.1, "kg"), ("جزر", 0.1, "kg"), ("طماطم", 0.2, "kg"), ("توابل متنوعة", 0.01, "package")],
        steps: &["اغسل العدس وقطّع الخضار.",
                 "اسلق كل المكونات مع الماء والبهارات حتى تنضج.",
                 "اطمح الشوربة حتى تصبح ناعمة وقدّمها مع الليمون."],
    },
    SeedRecipeDef {
        name_ar: "كشري", name_en: "Koshari", description: "الطبق المصري الوطني: أرز وعدس ومكرونة وصلصة.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 20, cook: 40, servings: 6, method: "boiled",
        tags: &["legume", "vegetarian", "economical"], allergens: &["جلوتين"],
        ingredients: &[("أرز", 0.4, "kg"), ("عدس", 0.2, "kg"), ("مكرونة", 0.3, "kg"), ("بصل", 0.2, "kg"), ("طماطم", 0.5, "kg"), ("خل", 0.05, "l")],
        steps: &["اسلق العدس والأرز كل على حدة ثم اطبخ المكرونة.",
                 "اصنع صلصة الطماطم وحمّر البصل.",
                 "رتّب الطبقات وقدّمها بالصلصة والخل والبصل المحمّر."],
    },
    SeedRecipeDef {
        name_ar: "ملوخية", name_en: "Molokhia", description: "طبق مصري كلاسيكي مع شوربة الفراخ.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 10, cook: 25, servings: 4, method: "boiled",
        tags: &["traditional"], allergens: &[],
        ingredients: &[("ملوخية", 0.5, "kg"), ("توم", 0.05, "kg"), ("فراخ بيضاء", 0.3, "kg"), ("سمنة", 0.02, "kg")],
        steps: &["اسلق الفراخ لعمل الشوربة.",
                 "أضف الملوخية للشوربة الساخنة وقلّب.",
                 "حمّر الثوم بالسمنة وأضفه فوق الملوخية (التقلية)."],
    },
    SeedRecipeDef {
        name_ar: "مسقعة", name_en: "Musakaa", description: "باذنجان بالصلصة والخضار — وجبة نباتية مشبعة.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 20, cook: 45, servings: 4, method: "baked",
        tags: &["vegetarian"], allergens: &[],
        ingredients: &[("باذنجان", 0.7, "kg"), ("بطاطس", 0.3, "kg"), ("طماطم", 0.5, "kg"), ("بصل", 0.1, "kg"), ("فلفل أخضر", 0.05, "kg"), ("زيت طعام", 0.3, "l")],
        steps: &["قطّع الباذنجان والبطاطس واشرحها بالفرن أو القلي.",
                 "اصنع صلصة الطماطم مع البصل والفلفل.",
                 "رتّب الطبقات في صينية واخبزها حتى تتماسك."],
    },
    SeedRecipeDef {
        name_ar: "بطاطس بالصلصة", name_en: "Potatoes in tomato sauce", description: "بطاطس اقتصادية وسهلة بالصلصة.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 15, cook: 30, servings: 4, method: "stewed",
        tags: &["vegetarian", "economical"], allergens: &[],
        ingredients: &[("بطاطس", 1.0, "kg"), ("طماطم", 0.5, "kg"), ("توم", 0.02, "kg"), ("زيت طعام", 0.1, "l")],
        steps: &["قطّع البطاطس وشوّحها قليلاً بالزيت.",
                 "أضف صلصة الطماطم والثوم.",
                 "اتركها على نار هادئة حتى تنضج تماماً."],
    },
    SeedRecipeDef {
        name_ar: "صينية خضار", name_en: "Vegetable tray", description: "خضار مشكلة بالفرن — عملية وقليلة الدهون.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 20, cook: 40, servings: 4, method: "baked",
        tags: &["vegetarian", "healthy"], allergens: &[],
        ingredients: &[("كوسة", 0.3, "kg"), ("باذنجان", 0.3, "kg"), ("بطاطس", 0.3, "kg"), ("جزر", 0.2, "kg"), ("طماطم", 0.3, "kg"), ("فلفل ألوان", 0.1, "kg"), ("زيت طعام", 0.1, "l")],
        steps: &["قطّع الخضار إلى قطع متوسطة.",
                 "اخلطها بالزيت والتوابل في صينية.",
                 "اخبزها 40 دقيقة مع التقليب مرة واحدة."],
    },
    SeedRecipeDef {
        name_ar: "أرز بالخضار", name_en: "Rice with vegetables", description: "أرز مطبوخ مع خضار موسمية.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 15, cook: 30, servings: 4, method: "boiled",
        tags: &["vegetarian", "economical"], allergens: &[],
        ingredients: &[("أرز", 0.5, "kg"), ("كوسة", 0.2, "kg"), ("جزر", 0.2, "kg"), ("بصل", 0.1, "kg"), ("زيت طعام", 0.1, "l")],
        steps: &["شوّح البصل والخضار بالزيت.",
                 "أضف الأرز والماء والملح.",
                 "غطِ على نار هادئة حتى ينضج الأرز."],
    },
    SeedRecipeDef {
        name_ar: "بيض بالخضار", name_en: "Eggs with vegetables", description: "فطور سريع بالبيض والخضار.",
        meal_types: &["breakfast", "lunch"], difficulty: "easy", prep: 10, cook: 15, servings: 2, method: "stewed",
        tags: &["quick"], allergens: &["بيض"],
        ingredients: &[("بيض", 4.0, "piece"), ("طماطم", 0.3, "kg"), ("فلفل أخضر", 0.05, "kg"), ("بصل", 0.05, "kg"), ("زيت طعام", 0.03, "l")],
        steps: &["شوّح الخضار بالزيت حتى تلين.",
                 "أضف البيض المخفوق وقلّب حتى ينضج.",
                 "قدّمه مع الخبز البلدي."],
    },
    SeedRecipeDef {
        name_ar: "مجدرة", name_en: "Mujadara", description: "عدس وأرز وبصل محمّر — وجبة بقوليات اقتصادية.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 10, cook: 35, servings: 4, method: "boiled",
        tags: &["legume", "vegetarian", "economical"], allergens: &[],
        ingredients: &[("عدس", 0.4, "kg"), ("أرز", 0.2, "kg"), ("بصل", 0.3, "kg"), ("زيت طعام", 0.1, "l")],
        steps: &["اسلق العدس حتى نصف النضج.",
                 "أضف الأرز وكمية ماء مناسبة واتركه ينضج.",
                 "حمّر البصل بالزيت وأضفه فوق المجدرة."],
    },
    SeedRecipeDef {
        name_ar: "فتة حمص", name_en: "Fatta with chickpeas", description: "فتة اقتصادية نباتية بالحمص والخبز المحمّص.",
        meal_types: &["lunch"], difficulty: "easy", prep: 15, cook: 20, servings: 4, method: "stewed",
        tags: &["legume", "vegetarian", "economical"], allergens: &["جلوتين"],
        ingredients: &[("حمص", 0.5, "kg"), ("خبز بلدي", 4.0, "piece"), ("طماطم", 0.4, "kg"), ("خل", 0.05, "l"), ("توم", 0.02, "kg")],
        steps: &["اسلق الحمص حتى ينضج.",
                 "قطّع الخبز وحمّصه بالفرن.",
                 "اصنع صلصة الطماطم بالثوم والخل ورتّب الفتة."],
    },
    SeedRecipeDef {
        name_ar: "شاورما دجاج منزلية", name_en: "Homemade chicken shawarma", description: "شاورما دجاج اقتصادية في البيت.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 20, cook: 25, servings: 4, method: "grilled",
        tags: &["poultry"], allergens: &["جلوتين"],
        ingredients: &[("فراخ بيضاء", 0.8, "kg"), ("خبز بلدي", 4.0, "piece"), ("طماطم", 0.2, "kg"), ("بصل", 0.1, "kg"), ("توابل متنوعة", 0.02, "package"), ("زيت طعام", 0.05, "l")],
        steps: &["تبّل شرائح الدجاج بالتوابل والزيت واتركها نصف ساعة.",
                 "اشوِ الدجاج على صينية ساخنة مع البصل.",
                 "لفّه في الخبز مع الطماطم والصلصة."],
    },
    SeedRecipeDef {
        name_ar: "محمرة", name_en: "Mhammara rice", description: "أرز بالصلصة واللحم على الطريقة الليبية الشهية.",
        meal_types: &["lunch", "dinner"], difficulty: "medium", prep: 15, cook: 35, servings: 5, method: "stewed",
        tags: &["rice_dish"], allergens: &[],
        ingredients: &[("أرز", 0.5, "kg"), ("طماطم", 0.5, "kg"), ("توم", 0.02, "kg"), ("سمنة", 0.05, "kg"), ("لحم", 0.3, "kg")],
        steps: &["اصنع صلصة الطماطم بالثوم والسمنة.",
                 "شوّح اللحم ثم أضفه للصلصة.",
                 "أضف الأرز والماء واتركه ينضج على نار هادئة."],
    },
    SeedRecipeDef {
        name_ar: "شوربة خضار اقتصادية", name_en: "Economical vegetable soup", description: "شوربة خفيفة من خضار الموسم.",
        meal_types: &["lunch", "dinner"], difficulty: "easy", prep: 10, cook: 25, servings: 4, method: "boiled",
        tags: &["vegetarian", "economical"], allergens: &[],
        ingredients: &[("شوربة خضار جاهزة", 1.0, "package"), ("بصل", 0.1, "kg"), ("بطاطس", 0.2, "kg"), ("جزر", 0.2, "kg"), ("توابل متنوعة", 0.01, "package")],
        steps: &["قطّع الخضار.",
                 "اسلقها مع التوابل حتى تلين.",
                 "اطمحها أو قدمها قطع حسب الرغبة."],
    },
    SeedRecipeDef {
        name_ar: "أرز باللبن", name_en: "Rice pudding", description: "حلوى منزلية اقتصادية.",
        meal_types: &["dessert"], difficulty: "easy", prep: 5, cook: 25, servings: 6, method: "boiled",
        tags: &["dessert", "economical"], allergens: &["لبن"],
        ingredients: &[("أرز", 0.2, "kg"), ("سكر", 0.15, "kg"), ("لبن", 0.5, "l"), ("توابل متنوعة", 0.01, "package")],
        steps: &["اسلق الأرز في اللبن مع التحريك.",
                 "أضف السكر والفانيليا.",
                 "قدّمه بارداً مع القرفة."],
    },
    SeedRecipeDef {
        name_ar: "عصير ليمون بالنعناع", name_en: "Mint lemonade", description: "مشروب منعش اقتصادي.",
        meal_types: &["juice"], difficulty: "easy", prep: 10, cook: 0, servings: 5, method: "raw",
        tags: &["beverage", "economical"], allergens: &[],
        ingredients: &[("ليمون", 0.3, "kg"), ("سكر", 0.1, "kg"), ("نعناع", 1.0, "bundle")],
        steps: &["اعصر الليمون.",
                 "اخلط العصير مع الماء والسكر.",
                 "أضف النعناع الطازج وقدمه مثلجاً."],
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
