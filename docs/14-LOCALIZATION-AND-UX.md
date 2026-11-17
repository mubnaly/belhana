# 14 — Localization and UX

## Language and direction
- `apps/web/src/app.html` defaults to `lang="ar" dir="rtl"` and title «بالهنا».
- `src/lib/i18n.js` holds Arabic and English dictionaries plus a language store; `setLang`/`toggleLang`
  update the document direction. Every screen has an EN/عربي toggle in its header.
- Domain data is bilingual where available (product `name_ar`/`name_en`); merchant/recipe names seeded
  from the Arabic source stay Arabic.

## Navigation
Bottom navigation on small screens: home dashboard, plan, shopping, inventory, recipes, more.
`/more` indexes budget, prices, family, rules, occasions, cooking, assistant, imports and settings.

## Screens
Dashboard (budget/plan/expiry/recent prices), login/register, plan (generate/save/conflicts),
shopping (review/complete), inventory, recipes (cost modal), eight `/more` tools plus imports and
settings. Design system: `app.css` CSS variables, cards, chips, stats, progress, tables, modal,
bottom nav. No Tailwind.

## UX rules
- Money always appears formatted as EGP in the UI while the wire format remains integer piasters.
- Review statuses are shown as coloured chips; ambiguous data is never hidden.
- Loading/error/empty states are explicit on each screen.
- Known limitation: Svelte emits non-blocking a11y warnings for unassociated form labels; functionality
  is unaffected but a full RTL/a11y audit remains open.
