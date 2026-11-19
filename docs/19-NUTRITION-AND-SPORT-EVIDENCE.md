# Nutrition, diet and sport evidence notes

These notes record the web research done before implementing the nutrition, diet and
athlete features. Every number shown in the app is an **estimate**, not a clinical or
verified claim. Sources were checked online in November 2026; the built-in web search was
unavailable, so public pages were fetched directly and via DuckDuckGo HTML results.

## Macro values

- Wikipedia's *Table of food nutrients* (compiled from USDA data):
  https://en.wikipedia.org/wiki/Table_of_food_nutrients
  - Included fields per food: serving weight (g), kcal, protein (g), carbohydrate (g),
    fibre (g), fat (g) and saturated fat (g).
  - Examples used to calibrate per-100 g editorial defaults:
    - beef chuck roasted: 85 g → 245 kcal, 23 g protein, 0 g carbs, 16 g fat
    - chicken broiled: 85 g → 185 kcal, 23 g protein, 0 g carbs, 9 g fat
    - salmon canned: 85 g → 120 kcal, 17 g protein, 0 g carbs, 5 g fat
    - tuna canned: 85 g → 170 kcal, 25 g protein, 0 g carbs, 7 g fat
    - lentils: 200 g → 212 kcal, 15 g protein, 38 g carbs, 2.4 g fibre
    - banana: 150 g → 85 kcal, 1 g protein, 23 g carbs, 0.9 g fibre
    - whole milk: 976 g → 660 kcal, 32 g protein, 48 g carbs, 40 g fat
    - almonds: 70 g → 425 kcal, 13 g protein, 13 g carbs, 1.8 g fibre
- USDA FoodData Central is the upstream source: https://fdc.nal.usda.gov/
- The user's example ("lamb has 20 g protein, 12 g carbs, 3 g fat") is illustrative, not a
  verified food record; the app therefore labels all macro values with `≈`.

## Protein targets

- ISSN position stand: 1.4–2.0 g protein/kg body weight/day for exercising adults:
  https://pubmed.ncbi.nlm.nih.gov/17908291/ and
  https://link.springer.com/article/10.1186/s12970-017-0177-8
- Harvard Nutrition Source: general adult minimum 0.8 g/kg/day:
  https://nutritionsource.hsph.harvard.edu/nutritionsource/what-should-you-eat/protein/
- Healthline quoting ISSN: 20–40 g protein every 3–4 hours post training:
  https://www.healthline.com/nutrition/eat-after-workout

## Basal and daily energy

- Mifflin–St Jeor BMR: men `10×kg + 6.25×cm − 5×age + 5`, women `− 161`:
  https://www.mifflinstjeorcalculator.com/mifflin-st-jeor-equation
- Activity multipliers are conventional estimates (sedentary ≈1.2, light ≈1.375,
  moderate ≈1.55, active ≈1.725, very active ≈1.9) and are labelled as such.

## Exercise energy expenditure (MET)

- Formula: `kcal = MET × body mass (kg) × duration (hours)`:
  https://www.topendsports.com/weight-loss/energy-met.htm and
  https://calculatorhub.com/tools/metabolic-equivalent-calculator/
- Compendium-based MET values from TopendSports:
  - standing 1.3, desk work 1.5, slow walking (1–2 mph) 2.0
  - brisk walking (3 mph) 3.5, fast walking (4 mph) 5.0
  - weight training (light/moderate) 3.5, cycling 10 mph 4.0
  - jogging 4.5 mph 6.0, rowing 7.0, swimming laps 8.0
  - running 6 mph 10.0, running 8 mph 13.5
- MET values are approximations with considerable individual variation.
- Correction to the illustrative numbers in the request: for a 70 kg adult, one hour of
  brisk walking is ≈245 kcal (3.5 METs), not 300; one hour of moderate weight training is
  ≈245 kcal (3.5 METs), and even vigorous resistance work at ~6 METs is ≈420 kcal, not
  900 kcal. The app shows the MET-derived estimate and explains the assumption.

## Meal timing

- Academy of Nutrition and Dietetics/EatRight: fuel 1–4 hours before exercise; include
  carbs plus some protein; eat within an hour after intense exercise with carbs and
  protein; recovery examples include low-fat chocolate milk, yogurt with berries and a
  turkey whole-grain wrap:
  https://www.eatright.org/fitness/physical-activity/exercise-nutrition/timing-your-pre-and-post-workout-nutrition
- Healthline: complete meal 2–3 h before, smaller carb+protein meal 1–1.5 h before, quick
  options (banana, shake) 45–60 min before:
  https://www.healthline.com/nutrition/eat-before-workout
- Pre-workout suggestions (EatRight): PB/banana or PBJ sandwich, Greek yogurt with berries,
  oatmeal with low-fat milk and fruit, apple with nut butter, nuts and raisins.

## Micronutrients and food suggestions

- Iron: MedlinePlus https://medlineplus.gov/ency/article/002422.htm and Harvard
  https://nutritionsource.hsph.harvard.edu/iron/
  - Heme: oysters/clams/mussels, liver, lean red meat, poultry dark meat, fish/tuna.
  - Non-heme: fortified cereals, beans, lentils, dark chocolate, spinach, potato skin,
    nuts/seeds, enriched grains.
  - Vitamin C improves absorption; tea/coffee tannins, phytates and bran reduce it.
  - RDA: adult men 8 mg, women 19–50 18 mg, pregnancy 27 mg.
- Zinc: MedlinePlus https://medlineplus.gov/ency/article/002416.htm
  - Beef, pork, lamb > fish; chicken dark meat; nuts, whole grains, legumes, yeast.
- Copper: Harvard https://nutritionsource.hsph.harvard.edu/copper/
  - RDA adults 900 mcg; sources: organ meats, shellfish, fish, nuts, seeds, whole grains,
    chickpeas, potatoes, spinach, dark chocolate.
- Vitamin B12: Harvard https://nutritionsource.hsph.harvard.edu/vitamin-b12/
  - RDA 2.4 mcg for adults; animal foods naturally; fortified foods/supplement for
    vegetarians and vegans.
- Vitamin D: Harvard https://nutritionsource.hsph.harvard.edu/vitamin-d/
  - Few natural food sources; sunlight is primary, fortified foods/supplements often needed.
- Calcium: Harvard https://nutritionsource.hsph.harvard.edu/calcium/
  - Important for bone health; adult upper limit 2,000–2,500 mg/day from food and supplements.
- Folate: MedlinePlus https://medlineplus.gov/ency/article/002092.htm
  - 400 mcg/day recommended for women who may become pregnant.

## Diets

- Mediterranean diet: MedlinePlus https://medlineplus.gov/ency/patientinstructions/000110.htm
  - Plant-forward meals, whole grains, fruit/vegetables, nuts, legumes, fish/seafood and
    olive oil as the main fat; limit red meat, sweets, eggs and butter.
  - Note possible lower iron and calcium intake — pair with the micronutrient suggestions.
  - Harvard Healthy Eating Plate: https://nutritionsource.hsph.harvard.edu/healthy-eating-plate/
- Elimination diet: Cleveland Clinic https://health.clevelandclinic.org/elimination-diet
  - Should be guided by a professional; common triggers: milk, eggs, fish, shellfish,
    peanuts, tree nuts, soy, wheat/gluten, lactose, nightshades, alcohol, corn, histamines,
    sulfites; remove for ~4–8 weeks, keep a journal, reintroduce one food at a time.
- Athlete plans: combine the protein target above with carbohydrate-focused fuel around
  training; adjust total energy using BMR × activity factor.

## Implementation rules derived from research

1. Label every macro, micronutrient, burn and target value as an estimate (`≈`).
2. Never present the illustrative 300/900 kcal numbers as verified; show MET math.
3. Show a medical/professional-guidance note for elimination diets and deficiency claims.
4. Keep Arabic-first UI, English toggle, integer-piastor money, sticky-note cards,
   `currentColor` monochrome icons, and no slot underlines.
