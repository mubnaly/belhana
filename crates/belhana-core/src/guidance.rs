//! Editorial nutrition guidance: micronutrient food suggestions, diet plans,
//! activity MET values and meal timing.
//!
//! This module is deliberately educational. It does not diagnose deficiencies,
//! prescribe medical diets or claim verified nutrient measurements. Every value
//! is approximate and the UI labels it as an estimate. Sources are listed in
//! `docs/19-NUTRITION-AND-SPORT-EVIDENCE.md`.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};

use crate::nutrition;

#[derive(Debug, Clone, Serialize)]
pub struct Text {
    pub ar: String,
    pub en: String,
}

fn tx(ar: impl Into<String>, en: impl Into<String>) -> Text {
    Text {
        ar: ar.into(),
        en: en.into(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FoodTip {
    pub food: Text,
    pub reason: Text,
}

#[derive(Debug, Clone, Serialize)]
pub struct Nutrient {
    pub id: String,
    pub name: Text,
    pub summary: Text,
    pub foods: Vec<FoodTip>,
    pub advice: Text,
    pub caution: Text,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diet {
    pub id: String,
    pub name: Text,
    pub summary: Text,
    pub include: Vec<Text>,
    pub limit: Vec<Text>,
    pub meals: Vec<Text>,
    pub caution: Text,
}

#[derive(Debug, Clone, Serialize)]
pub struct Exercise {
    pub id: String,
    pub name: Text,
    pub met: f64,
    pub note: Text,
}

#[derive(Debug, Clone, Serialize)]
pub struct Timing {
    pub id: String,
    pub when: Text,
    pub body: Text,
    pub options: Vec<Text>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Source {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Guide {
    pub micronutrients: Vec<Nutrient>,
    pub diets: Vec<Diet>,
    pub exercises: Vec<Exercise>,
    pub meal_timing: Vec<Timing>,
    pub notes: Vec<Text>,
    pub sources: Vec<Source>,
}

fn exercises_data() -> Vec<Exercise> {
    vec![
        Exercise {
            id: "walking_slow".into(),
            name: tx("مشي بطيء (1-2 كم/س)", "Slow walking (1-2 mph)"),
            met: 2.0,
            note: tx("≈ 140 سعرة/ساعة لشخص 70 كجم", "≈ 140 kcal/h at 70 kg"),
        },
        Exercise {
            id: "walking_brisk".into(),
            name: tx("مشي سريع (حوالي 5 كم/س)", "Brisk walking (about 3 mph)"),
            met: 3.5,
            note: tx("≈ 245 سعرة/ساعة لشخص 70 كجم", "≈ 245 kcal/h at 70 kg"),
        },
        Exercise {
            id: "walking_fast".into(),
            name: tx("مشي سريع جدا (6.5 كم/س)", "Fast walking (4 mph)"),
            met: 5.0,
            note: tx("≈ 350 سعرة/ساعة لشخص 70 كجم", "≈ 350 kcal/h at 70 kg"),
        },
        Exercise {
            id: "weight_training".into(),
            name: tx("تمارين قوة (متوسطة)", "Weight training (moderate)"),
            met: 3.5,
            note: tx("≈ 245 سعرة/ساعة لشخص 70 كجم", "≈ 245 kcal/h at 70 kg"),
        },
        Exercise {
            id: "weight_training_vigorous".into(),
            name: tx("تمارين قوة (شديدة)", "Weight training (vigorous)"),
            met: 6.0,
            note: tx("≈ 420 سعرة/ساعة لشخص 70 كجم", "≈ 420 kcal/h at 70 kg"),
        },
        Exercise {
            id: "cycling".into(),
            name: tx("دراجة (16 كم/س)", "Cycling (10 mph)"),
            met: 4.0,
            note: tx("≈ 280 سعرة/ساعة لشخص 70 كجم", "≈ 280 kcal/h at 70 kg"),
        },
        Exercise {
            id: "jogging".into(),
            name: tx("هرولة (7 كم/س)", "Jogging (4.5 mph)"),
            met: 6.0,
            note: tx("≈ 420 سعرة/ساعة لشخص 70 كجم", "≈ 420 kcal/h at 70 kg"),
        },
        Exercise {
            id: "rowing".into(),
            name: tx("تجديف", "Rowing"),
            met: 7.0,
            note: tx("≈ 490 سعرة/ساعة لشخص 70 كجم", "≈ 490 kcal/h at 70 kg"),
        },
        Exercise {
            id: "swimming".into(),
            name: tx("سباحة (استمرار)", "Swimming laps"),
            met: 8.0,
            note: tx("≈ 560 سعرة/ساعة لشخص 70 كجم", "≈ 560 kcal/h at 70 kg"),
        },
        Exercise {
            id: "running".into(),
            name: tx("جري (10 كم/س)", "Running (6 mph)"),
            met: 10.0,
            note: tx("≈ 700 سعرة/ساعة لشخص 70 كجم", "≈ 700 kcal/h at 70 kg"),
        },
    ]
}

/// All static guidance, serialised for the web pages.
pub fn guide() -> Guide {
    let micronutrients = vec![
        Nutrient {
            id: "iron".into(),
            name: tx("الحديد", "Iron"),
            summary: tx(
                "الرجال البالغون ≈8 مجم/يوم، والنساء 19-50 سنة ≈18 مجم، والحمل ≈27 مجم. تقديرات عامة وليست تحليلاً مخبرياً.",
                "Adults: men ≈8 mg/day, women 19-50 ≈18 mg, pregnancy ≈27 mg. General estimates, not lab results.",
            ),
            foods: vec![
                FoodTip { food: tx("كبدة ولحوم حمراء خالية من الدهون", "Liver and lean red meat"), reason: tx("حديد هيم يمتصه الجسم بسهولة أكبر", "Heme iron is absorbed more readily") },
                FoodTip { food: tx("دجاج لحم داكن، سمك وتونة، محار", "Dark poultry meat, fish, tuna, shellfish"), reason: tx("مصدر حيواني بديل للحديد الهيم", "Alternative heme-iron sources") },
                FoodTip { food: tx("عدس، فول، حمص، حبوب مدعّمة، سبانخ، شوكولاتة داكنة", "Lentils, beans, chickpeas, fortified cereals, spinach, dark chocolate"), reason: tx("حديد غير هيم؛ تناوله مع فيتامين سي", "Non-heme iron; pair with vitamin C") },
            ],
            advice: tx("أضف عصير ليمون أو برتقال أو فلفل ألوان مع البقوليات والخضروات لتحسين الامتصاص.", "Add lemon/orange juice or bell peppers with legumes and vegetables to improve absorption."),
            caution: tx("الشاي والقهوة والنخالة ومضادات التغذية قد تقلل الامتصاص؛ لا تعالج نقص الحديد بالطعام وحده دون استشارة طبيب.", "Tea, coffee, bran and phytates may reduce absorption; do not treat deficiency with food alone without medical advice."),
        },
        Nutrient {
            id: "copper".into(),
            name: tx("النحاس", "Copper"),
            summary: tx("الاحتياج اليومي التقديري للبالغين ≈900 ميكروجرام.", "Approximate adult requirement ≈900 mcg/day."),
            foods: vec![
                FoodTip { food: tx("كبدة ومخروطات", "Organ meats"), reason: tx("من أعلى المصادر", "Among the richest sources") },
                FoodTip { food: tx("محار وسمك وقشريات", "Oysters, fish and shellfish"), reason: tx("مصدر حيواني غني", "Rich animal source") },
                FoodTip { food: tx("كاجو، سمسم، عباد الشمس، حمص، حبوب كاملة، شوكولاتة داكنة", "Cashews, sesame, sunflower seeds, chickpeas, whole grains, dark chocolate"), reason: tx("مصادر نباتية مفيدة", "Useful plant sources") },
            ],
            advice: tx("نوّع بين المكسرات والبقوليات والحبوب الكاملة ضمن الوجبات الأسبوعية.", "Rotate nuts, legumes and whole grains across weekly meals."),
            caution: tx("المكملات عالية الجرعة قد تكون ضارة؛ استشر مختصاً قبل استخدامها.", "High-dose supplements can be harmful; ask a professional first."),
        },
        Nutrient {
            id: "zinc".into(),
            name: tx("الزنك", "Zinc"),
            summary: tx("مهم للمناعة والتئام الجروح والنمو؛ يمتص من المصادر الحيوانية بسهولة أكبر.", "Supports immunity, wound healing and growth; animal sources absorb better."),
            foods: vec![
                FoodTip { food: tx("لحم بقر أو ضأن، لحم دجاج داكن", "Beef, lamb, dark chicken meat"), reason: tx("أعلى توافر حيوي", "Higher bioavailability") },
                FoodTip { food: tx("سمك ومأكولات بحرية", "Fish and seafood"), reason: tx("بديل حيواني أقل زنكاً من اللحوم الحمراء", "Animal alternative, lower than red meat") },
                FoodTip { food: tx("مكسرات، حبوب كاملة، بقوليات، خميرة", "Nuts, whole grains, legumes, yeast"), reason: tx("مصادر نباتية؛ نقع الحبوب والبقوليات يساعد", "Plant sources; soaking grains and legumes helps") },
            ],
            advice: tx("اجمع بين مصدر حيواني ومصدر نباتي في اليوم نفسه عندما أمكن.", "Combine an animal and a plant source in the same day when possible."),
            caution: tx("النباتات والفواكه ليست مصادر جيدة مقارنة باللحوم.", "Fruits and vegetables are comparatively poor sources."),
        },
        Nutrient {
            id: "calcium".into(),
            name: tx("الكالسيوم", "Calcium"),
            summary: tx("مهم لصحة العظام؛ الحد الأعلى العام ≈2000-2500 مجم/يوم من الطعام والمكملات.", "Important for bone health; general upper limit ≈2,000-2,500 mg/day from food and supplements."),
            foods: vec![
                FoodTip { food: tx("لبن، جبنة بيضاء، قشطة، زبادي", "Milk, white cheese, cream, yoghurt"), reason: tx("مصادر سهلة الاستخدام اليومي", "Convenient daily sources") },
                FoodTip { food: tx("بدائل نباتية مدعّمة بالكالسيوم", "Calcium-fortified plant drinks"), reason: tx("بديل لمن لا يتناول الألبان", "Alternative for non-dairy eaters") },
                FoodTip { food: tx("طحينة، سبانخ، سمك صغير بعظامه", "Tahini, spinach, small fish with bones"), reason: tx("خيارات غير ألبانية", "Non-dairy options") },
            ],
            advice: tx("وزّع المصادر على الوجبات بدلاً من جرعة واحدة كبيرة.", "Spread sources across meals rather than one large dose."),
            caution: tx("لا تجاوز الحد الأعلى دون إشراف طبي؛ حمية البحر المتوسط قد تكون أقل في الكالسيوم.", "Do not exceed the upper limit without medical supervision; Mediterranean-style eating may be lower in calcium."),
        },
        Nutrient {
            id: "vitamin_d".into(),
            name: tx("فيتامين د", "Vitamin D"),
            summary: tx("قليل من الأطعمة تحتويه طبيعياً؛ الشمس المصدر الأساسي وقد يحتاج كثيرون مكملاً.", "Few foods contain it naturally; sunlight is the main source and many people need a supplement."),
            foods: vec![
                FoodTip { food: tx("سمك دهني مثل السلمون والسردين", "Fatty fish such as salmon and sardines"), reason: tx("من أغنى المصادر الغذائية", "Among the richest food sources") },
                FoodTip { food: tx("صفار البيض", "Egg yolks"), reason: tx("مصدر غذائي محدود لكن مفيد", "A limited but useful source") },
                FoodTip { food: tx("ألبان أو بدائل مدعّمة", "Fortified milk or plant alternatives"), reason: tx("تحقق من الملصق", "Check the product label") },
            ],
            advice: tx("عرّض الجلد للشمس باعتدال وفق بشرتك ووقت اليوم، وراجع الطبيب قبل المكملات.", "Get moderate sun exposure appropriate to your skin and time of day; ask a clinician about supplements."),
            caution: tx("الاعتماد على الطعام وحده عادة لا يكفي.", "Food alone is usually not enough."),
        },
        Nutrient {
            id: "b12".into(),
            name: tx("فيتامين ب12", "Vitamin B12"),
            summary: tx("الاحتياج التقديري للبالغين ≈2.4 ميكروجرام/يوم.", "Approximate adult requirement ≈2.4 mcg/day."),
            foods: vec![
                FoodTip { food: tx("لحوم ودواجن وأسماك", "Meat, poultry and fish"), reason: tx("مصادر طبيعية حيوانية", "Natural animal sources") },
                FoodTip { food: tx("بيض وألبان", "Eggs and dairy"), reason: tx("خيارات بديلة", "Alternative options") },
                FoodTip { food: tx("أطعمة مدعّمة أو مكمل", "Fortified foods or a supplement"), reason: tx("ضروري للنباتيين", "Important for vegetarians and vegans") },
            ],
            advice: tx("من لا يأكل منتجات حيوانية يحتاج مصدراً مدعّماً أو مكملاً بانتظام.", "People avoiding animal products need a regular fortified source or supplement."),
            caution: tx("النباتيون والنباتيون الصرفون من الفئات الأكثر عرضة للنقص.", "Vegetarians and vegans are at higher risk."),
        },
        Nutrient {
            id: "folate".into(),
            name: tx("الفولات", "Folate"),
            summary: tx("مهم قبل الحمل وأثنائه؛ التوصية العامة ≈400 ميكروجرام/يوم لمن قد تحمل.", "Important before and during pregnancy; generally ≈400 mcg/day for those who may become pregnant."),
            foods: vec![
                FoodTip { food: tx("خضروات ورقية داكنة", "Dark leafy greens"), reason: tx("مصدر طبيعي", "Natural source") },
                FoodTip { food: tx("بقوليات مثل العدس والفول", "Legumes such as lentils and beans"), reason: tx("مصدر نباتي غني", "Rich plant source") },
                FoodTip { food: tx("حبوب مدعّمة بحمض الفوليك", "Folic-acid fortified grains"), reason: tx("مصدر موثوق التركيز", "Standardised source") },
            ],
            advice: tx("التزم بالتعليمات الطبية إذا كنت تخططين للحمل.", "Follow medical guidance if planning a pregnancy."),
            caution: tx("الجرعات العالية تحتاج إشرافاً طبياً.", "High doses need medical supervision."),
        },
    ];

    let diets = vec![
        Diet {
            id: "mediterranean".into(),
            name: tx("حمية البحر المتوسط", "Mediterranean diet"),
            summary: tx("نمط غذائي نباتي في أساسه: حبوب كاملة وخضروات وفواكه وبقوليات ومكسرات وزيت زيتون، مع سمك ومأكولات بحرية.", "A plant-forward pattern: whole grains, vegetables, fruit, legumes, nuts and olive oil, with fish and seafood."),
            include: vec![
                tx("خضروات وفواكه متنوعة يومياً", "A variety of vegetables and fruit daily"),
                tx("حبوب كاملة مثل الشوفان والبر والكينوا والأرز البني", "Whole grains such as oats, wheat, quinoa and brown rice"),
                tx("بقوليات ومكسرات غير مملحة", "Legumes and unsalted nuts"),
                tx("سمك مرتين أسبوعياً تقريباً وزيت زيتون كدهن أساسي", "Fish about twice a week and olive oil as the main fat"),
            ],
            limit: vec![
                tx("اللحوم الحمراء والحلويات والزبدة", "Red meat, sweets and butter"),
                tx("البيض واللحوم المصنعة باعتدال", "Eggs and processed meats in moderation"),
            ],
            meals: vec![
                tx("فطور: شوفان بالحليب وفاكهة ومكسرات", "Breakfast: oats with milk, fruit and nuts"),
                tx("غداء: عدس أو حمص مع أرز بني وخضروات وزيت زيتون", "Lunch: lentils or chickpeas with brown rice, vegetables and olive oil"),
                tx("عشاء: سمك مشوي مع خضروات وخبز حبوب كاملة", "Dinner: grilled fish with vegetables and whole-grain bread"),
            ],
            caution: tx("قد تنخفض فيها هذه العناصر: الحديد والكالسيوم وفيتامين د — راجع اقتراحات المغذيات الدقيقة.", "Iron, calcium and vitamin D may be lower — see the micronutrient suggestions."),
        },
        Diet {
            id: "elimination".into(),
            name: tx("نظام الاستبعاد", "Elimination diet"),
            summary: tx("إزالة أطعمة محددة مؤقتاً ثم إعادة إدخالها واحداً تلو الآخر لملاحظة الأثر. يجب أن يتم بإشراف مختص.", "Temporarily remove specific foods, then reintroduce them one at a time to observe effects. Should be professionally guided."),
            include: vec![
                tx("مفكرة يومية للطعام والأعراض", "A daily food and symptom journal"),
                tx("بدائل آمنة للأطعمة المستبعدة", "Safe alternatives to excluded foods"),
                tx("إعادة إدخال طعام واحد كل بضعة أيام", "Reintroduce one food every few days"),
            ],
            limit: vec![
                tx("الحليب والبيض والسمك والقشريات", "Milk, eggs, fish and shellfish"),
                tx("الفول السوداني والمكسرات الشجرية والصويا", "Peanuts, tree nuts and soy"),
                tx("القمح والجلوتين واللاكتوز والكحول والذرة", "Wheat, gluten, lactose, alcohol and corn"),
                tx("الطماطم والفلفل (السباذج) والهستامين والسلفيت", "Tomatoes and peppers (nightshades), histamines and sulfites"),
            ],
            meals: vec![
                tx("وجبات بسيطة مكوّنة من مكونات قليلة ومعروفة", "Simple meals made from a few known ingredients"),
                tx("احتفظ بنسخة من المكونات لكل وجبة في المفكرة", "Record every ingredient for each meal in the journal"),
            ],
            caution: tx("لا تبدأ نظام استبعاد دون استشارة طبيب أو أخصائي تغذية، خاصة للأطفال والحمل والحساسية. مدة الاستبعاد الشائعة ≈4-8 أسابيع.", "Do not start without a clinician or dietitian, especially for children, pregnancy or allergies. Common removal period ≈4-8 weeks."),
        },
        Diet {
            id: "balanced_family".into(),
            name: tx("نمط متوازن للأسرة", "Balanced family pattern"),
            summary: tx("نصف الطبق خضروات وفواكه، وربع حبوب كاملة، وربع بروتين صحي، مع زيوت نباتية وماء بدل المشروبات المحلّاة.", "Half the plate vegetables and fruit, a quarter whole grains, a quarter healthy protein, with plant oils and water instead of sweet drinks."),
            include: vec![
                tx("خضروات وفواكه بألوان متعددة", "Vegetables and fruit of many colours"),
                tx("سمك ودجاج وبقوليات ومكسرات كبروتين", "Fish, poultry, legumes and nuts as protein"),
                tx("حبوب كاملة مثل الأرز البني والشوفان والبر", "Whole grains such as brown rice, oats and wheat"),
                tx("ماء كشراب أساسي", "Water as the main drink"),
            ],
            limit: vec![
                tx("اللحوم المصنعة واللحوم الحمراء بكثرة", "Processed meats and frequent red meat"),
                tx("الجبن والزبدة والسكر المضاف", "Cheese, butter and added sugar"),
                tx("العصائر والمشروبات الغازية", "Juice and fizzy drinks"),
            ],
            meals: vec![
                tx("فطور: بيض أو لبن مع خبز حبوب كاملة وفاكهة", "Breakfast: eggs or yoghurt with whole-grain bread and fruit"),
                tx("غداء: طبق نصفه خضروات مع أرز بني وبروتين", "Lunch: a plate half full of vegetables with brown rice and protein"),
                tx("عشاء: شوربة بقوليات مع خبز كامل وخضروات", "Dinner: legume soup with whole-grain bread and vegetables"),
            ],
            caution: tx("هذا إرشاد عام للأسرة وليس خطة علاجية لأي حالة مرضية.", "General family guidance, not a treatment plan for any condition."),
        },
    ];

    let meal_timing = vec![
        Timing {
            id: "before_2_3h".into(),
            when: tx("قبل التمرين بـ2-3 ساعات", "2-3 hours before exercise"),
            body: tx("وجبة كاملة تحتوي كربوهيدرات وبروتيناً مع قليل من الدهون؛ جرّبها في التدريب لا في يوم المباراة.", "A complete meal with carbohydrates and protein plus a little fat; try it in training, not on race day."),
            options: vec![
                tx("شوفان بالحليب قليل الدسم وفاكهة", "Oats with low-fat milk and fruit"),
                tx("أرز أو خبز كامل مع دجاج وخضروات", "Rice or whole-grain bread with chicken and vegetables"),
                tx("ساندويتش زبدة الفول السوداني والموز", "Peanut butter and banana sandwich"),
            ],
        },
        Timing {
            id: "before_1h".into(),
            when: tx("قبل التمرين بـ1-1.5 ساعة", "1-1.5 hours before exercise"),
            body: tx("وجبة أصغر غنية بالكربوهيدرات مع بروتين سريع الهضم.", "A smaller meal rich in carbohydrates with easily digested protein."),
            options: vec![
                tx("زبادي يوناني مع توت", "Greek yoghurt with berries"),
                tx("تفاحة مع زبدة مكسرات", "Apple with nut butter"),
                tx("لفة ديك رومي بالخضروات", "Turkey and vegetable wrap"),
            ],
        },
        Timing {
            id: "before_45m".into(),
            when: tx("قبل التمرين بـ45-60 دقيقة", "45-60 minutes before exercise"),
            body: tx("كربوهيدرات سريعة الهضم إذا كانت المعدة فارغة أو قبل جلسة طويلة.", "Quick-digesting carbs if the stomach is empty or before a long session."),
            options: vec![
                tx("موزة", "A banana"),
                tx("حبة أو حبتين تمر", "One or two dates"),
                tx("سموثي صغير بالفاكهة والحليب", "A small fruit and milk smoothie"),
            ],
        },
        Timing {
            id: "after_1h".into(),
            when: tx("خلال ساعة بعد التمرين", "Within an hour after exercise"),
            body: tx("كربوهيدرات لتجديد الجليكوجين وبروتين ≈20-40 جرام لبناء العضل، مع سائل للترطيب.", "Carbohydrates to replenish glycogen and ≈20-40 g protein for muscle repair, plus fluid for hydration."),
            options: vec![
                tx("حليب قليل الدسم بالشوكولاتة أو سموثي", "Low-fat chocolate milk or a smoothie"),
                tx("زبادي بالتوت وخبز حبوب كامل", "Yoghurt with berries and whole-grain bread"),
                tx("بيض مع بطاطا حلوة، أو لفتة تونة", "Eggs with sweet potato, or a tuna wrap"),
            ],
        },
    ];

    let notes = vec![
        tx("كل الأرقام تقديرية مبنية على جداول غذائية عامة، وليست قياساً مخبرياً أو تشخيصاً طبياً.", "All numbers are estimates from general food tables, not laboratory measurements or medical diagnoses."),
        tx("حساسية الطعام والاستبعادات الطبية قيود صارمة: راجع القواعد العائلية قبل اقتراح أي بديل.", "Food allergies and medical exclusions are hard constraints: check household rules before suggesting alternatives."),
        tx("نظام الاستبعاد وعلاج النقص يحتاجان إشراف مختص.", "Elimination diets and treating deficiencies need professional supervision."),
    ];

    let sources = vec![
        Source { label: "MedlinePlus — Iron in diet".into(), url: "https://medlineplus.gov/ency/article/002422.htm".into() },
        Source { label: "Harvard Nutrition Source — Iron".into(), url: "https://nutritionsource.hsph.harvard.edu/iron/".into() },
        Source { label: "Harvard Nutrition Source — Copper".into(), url: "https://nutritionsource.hsph.harvard.edu/copper/".into() },
        Source { label: "MedlinePlus — Zinc in diet".into(), url: "https://medlineplus.gov/ency/article/002416.htm".into() },
        Source { label: "Harvard Nutrition Source — Vitamin D / B12 / Calcium".into(), url: "https://nutritionsource.hsph.harvard.edu/".into() },
        Source { label: "MedlinePlus — Mediterranean diet".into(), url: "https://medlineplus.gov/ency/patientinstructions/000110.htm".into() },
        Source { label: "Cleveland Clinic — Elimination diet".into(), url: "https://health.clevelandclinic.org/elimination-diet".into() },
        Source { label: "TopendSports — MET values".into(), url: "https://www.topendsports.com/weight-loss/energy-met.htm".into() },
        Source { label: "EatRight — Timing pre/post workout nutrition".into(), url: "https://www.eatright.org/fitness/physical-activity/exercise-nutrition/timing-your-pre-and-post-workout-nutrition".into() },
        Source { label: "ISSN — Protein and exercise".into(), url: "https://pubmed.ncbi.nlm.nih.gov/17908291/".into() },
    ];

    Guide {
        micronutrients,
        diets,
        exercises: exercises_data(),
        meal_timing,
        notes,
        sources,
    }
}

#[derive(Debug, Deserialize)]
pub struct AthleteInput {
    pub sex: String,
    pub age: f64,
    pub height_cm: f64,
    pub weight_kg: f64,
    pub activity: String,
    pub goal: String,
    pub exercise_id: String,
    pub minutes: f64,
}

#[derive(Debug, Serialize)]
pub struct AthletePlan {
    pub bmr: i64,
    pub tdee: i64,
    pub target_kcal: i64,
    pub protein_g: i64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub protein_per_kg: f64,
    pub met: f64,
    pub burn_kcal: i64,
    pub summary: Text,
    pub disclaimer: Text,
}

/// Computes a coaching estimate from an athlete profile. No personal data is
/// stored: the calculation happens per request and the result is not saved.
pub fn athlete_plan(input: &AthleteInput) -> Result<AthletePlan, CommonError> {
    if !(12.0..=100.0).contains(&input.age) {
        return Err(CommonError::Validation("age out of range".into()));
    }
    if !(100.0..=250.0).contains(&input.height_cm) {
        return Err(CommonError::Validation("height out of range".into()));
    }
    if !(25.0..=300.0).contains(&input.weight_kg) {
        return Err(CommonError::Validation("weight out of range".into()));
    }
    if !(1.0..=600.0).contains(&input.minutes) {
        return Err(CommonError::Validation("duration out of range".into()));
    }
    let bmr = nutrition::bmr_mifflin(&input.sex, input.weight_kg, input.height_cm, input.age)
        .ok_or_else(|| CommonError::Validation("sex must be male or female".into()))?;
    let factor = nutrition::activity_factor(&input.activity)
        .ok_or_else(|| CommonError::Validation("unknown activity level".into()))?;
    let (protein_per_kg, energy_factor) = match input.goal.as_str() {
        "maintain" => (1.4, 1.0),
        "muscle" => (1.8, 1.1),
        "endurance" => (1.6, 1.15),
        "cut" => (1.6, 0.85),
        _ => return Err(CommonError::Validation("unknown goal".into())),
    };
    let exercise = exercises_data()
        .into_iter()
        .find(|e| e.id == input.exercise_id)
        .ok_or_else(|| CommonError::Validation("unknown exercise".into()))?;
    let tdee = (bmr * factor).round() as i64;
    let target_kcal = (tdee as f64 * energy_factor).round() as i64;
    let protein_g = (input.weight_kg * protein_per_kg).round() as i64;
    let (carbs_g, fat_g) = nutrition::macro_targets(target_kcal, protein_g as f64);
    let burn_kcal = nutrition::met_burn_kcal(exercise.met, input.weight_kg, input.minutes)
        .ok_or_else(|| CommonError::Validation("invalid exercise calculation".into()))?;
    Ok(AthletePlan {
        bmr: bmr.round() as i64,
        tdee,
        target_kcal,
        protein_g,
        carbs_g,
        fat_g,
        protein_per_kg,
        met: exercise.met,
        burn_kcal,
        summary: tx(
            format!(
                "معدل الحرق الأساسي ≈{} سعرة/يوم، واليومي ≈{}، والهدف ≈{} مع {} جرام بروتين و{} جرام كربوهيدرات و{} جرام دهون. حرق التمرين المختار ≈{} سعرة.",
                bmr.round() as i64, tdee, target_kcal, protein_g, carbs_g, fat_g, burn_kcal
            ),
            format!(
                "Estimated basal burn ≈{} kcal/day, daily ≈{}, target ≈{}, with {} g protein, {} g carbs and {} g fat. Selected workout burns ≈{} kcal.",
                bmr.round() as i64, tdee, target_kcal, protein_g, carbs_g, fat_g, burn_kcal
            ),
        ),
        disclaimer: tx(
            "تقديرات تعليمية بمعادلة ميفلين-ست جيور ومعاملات MET؛ لا تشخيص طبي ولا وصفة علاجية.",
            "Educational estimates using Mifflin-St Jeor and MET factors; not a medical diagnosis or prescription.",
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> AthleteInput {
        AthleteInput {
            sex: "male".into(),
            age: 30.0,
            height_cm: 175.0,
            weight_kg: 70.0,
            activity: "moderate".into(),
            goal: "maintain".into(),
            exercise_id: "walking_brisk".into(),
            minutes: 60.0,
        }
    }

    #[test]
    fn athlete_plan_uses_met_and_bmr() {
        let p = athlete_plan(&input()).unwrap();
        assert_eq!(p.bmr, 1649);
        assert_eq!(p.tdee, 2556);
        assert_eq!(p.burn_kcal, 245);
        assert_eq!(p.protein_g, 98);
    }

    #[test]
    fn athlete_plan_rejects_bad_input() {
        let mut bad = input();
        bad.sex = "unknown".into();
        assert!(athlete_plan(&bad).is_err());
        let mut bad = input();
        bad.exercise_id = "nope".into();
        assert!(athlete_plan(&bad).is_err());
    }

    #[test]
    fn guide_contains_all_sections() {
        let g = guide();
        assert_eq!(g.exercises.len(), 10);
        assert!(g.diets.iter().any(|d| d.id == "mediterranean"));
        assert!(g.diets.iter().any(|d| d.id == "elimination"));
        assert!(g.micronutrients.iter().any(|n| n.id == "iron"));
        assert_eq!(g.meal_timing.len(), 4);
    }
}
