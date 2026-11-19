# Diet systems evidence notes (Tayyibat, keto and others)

These notes support the diet-filter feature. They record public web research done in
November 2026. The app treats every diet as a **user-selected food preference/filter**,
not as a medical protocol. It never diagnoses, promises treatment or tells anyone to
stop medication. All calorie and macro values remain labelled `≈` estimates.

## Why this framing matters

The existing nutrition evidence note (`docs/19-NUTRITION-AND-SPORT-EVIDENCE.md`) already
requires estimate labels and professional-guidance warnings. That rule is even more
important for named diet systems with strong claims. Selecting “Tayyibat”, “keto” or any
other plan in Belhana only changes which recipes are suggested and planned. It does not
claim that the plan is clinically proven or suitable for every person.

## نظام الطيبات (Tayyibat)

- Public descriptions attribute the system to the late Dr. Dhiaa Al-Awadi
  (د. ضياء العوضي). It is usually presented as a **whitelist** system: foods inside the
  allowed list are used; foods outside it are avoided. Published lists vary between
  followers.
- Broad categories commonly described as allowed: rice and potatoes, whole-grain toast,
  aged cheeses, natural butter/ghee/olive oil, dates and some fruits, sardines and predator
  sea fish, red meat (beef/lamb/camel/rabbit/pigeon) on an every-other-day pattern, green
  tea and Turkish coffee.
- Broad categories commonly described as avoided: white flour/bread/pasta, chicken and
  eggs, milk and fresh white cheese, most legumes, leafy greens, nightshades, fizzy and
  industrial drinks, hydrogenated fats, many fresh juices and watermelon/melon.
- Frequency markers used by followers: daily staples, 2–3 times/week, occasionally, and
  “a day yes, a day no” for animal protein.
- Sources:
  - الطبي, medical review with explicit safety warnings:
    https://altibbi.com/مقالات-طبية/تغذية/نظام-الطيبات-10915
  - A follower-maintained comparative food table (18 categories):
    https://altayebaat.com/articles/جدول-المسموح-والممنوع-في-نظام-الطيبات
  - الموسوعة overview and health-warning discussion:
    https://almawso3a.com/نظام-الطيبات-الغذائي-حقيقة-الادعاءات-والتحذيرات-الصحية
- Evidence status: the medical review reports **no published controlled clinical trials**
  establishing Tayyibat as a safe and effective treatment for chronic disease, and notes
  that there is no single fixed published protocol. Followers' lists differ, so the app
  uses a conservative common-denominator allow/avoid keyword set and labels it as an
  approximate interpretation, not an authoritative protocol.
- Safety wording used in the UI: it may reduce highly processed foods for some people,
  but very restrictive versions can risk nutrient gaps; do not stop or change medication;
  ask a doctor or dietitian first, especially for children, pregnancy, older adults,
  diabetes, kidney or heart disease.

## Ketogenic (keto)

- Harvard Nutrition Source: keto is a very-low-carbohydrate, high-fat plan; typical
  published ratios are roughly 70–80% fat, 5–10% carbohydrate and 10–20% protein, often
  under 50 g carbohydrate/day (sometimes near 20 g). It avoids grains, flour, starchy
  vegetables, most fruit, fruit juice and legumes; it emphasises fats, moderate protein,
  nuts/seeds, some dairy and most non-starchy vegetables.
- Short-term studies show weight and some metabolic-marker improvements, but long-term
  evidence is limited, adherence is difficult, LDL cholesterol may rise, and nutrient
  gaps are possible. Ketoacidosis is a serious risk for people with type 1 diabetes and
  has been reported rarely in non-diabetic people; medical supervision is advised.
- Source: https://nutritionsource.hsph.harvard.edu/healthy-weight/diet-reviews/ketogenic-diet/
- App interpretation: a food-preference filter that favours low-carb proteins, eggs,
  aged cheese, nuts, oils and non-starchy vegetables, and avoids rice, bread, pasta,
  potatoes, sugar, juice and most fruit/legumes. Macro estimates remain estimates; the
  app does **not** compute or claim a ketosis state.

## Other plans already documented

See `docs/19-NUTRITION-AND-SPORT-EVIDENCE.md` for Mediterranean, elimination and
balanced-family plans. Those sources remain the reference for their food lists. New
plans added to the picker should include a short source note here and an `≈`/advice label
in the UI.

## Implementation rules

1. A selected diet is stored on the household and used as a filter/steering signal by the
   recipe list and meal planner.
2. Filtering is keyword-based and explainable; if no recipe matches, show a clear
   “no matching meals” state instead of silently ignoring the diet.
3. Never show diet selection as diagnosis or treatment. Keep the professional-advice
   note visible on the diet page and on plans generated under a restrictive diet.
4. Keep Arabic-first RTL, English toggle, integer-piastor money, sticky-note cards,
   `currentColor` icons, and no floating theme toggle.
