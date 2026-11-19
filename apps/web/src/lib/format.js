// Small display helpers shared by the pages.

// Estimated calories are always shown with an "approximately" sign and a note
// elsewhere that they are editorial estimates, never verified lab values.
export function kcalText(value, lang) {
	if (value == null || value <= 0) return '';
	const digits = Number(value).toLocaleString(lang === 'ar' ? 'ar-EG' : 'en');
	return `≈ ${digits} ${lang === 'ar' ? 'سعرة' : 'kcal'}`;
}

// Quantities from SQLite arithmetic carry floating-point noise (1.5999…).
export function qty(n) {
	if (n == null) return '';
	return String(Math.round(Number(n) * 1000) / 1000);
}
