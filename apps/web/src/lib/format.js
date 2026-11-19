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

export function num(value, lang) {
	if (value == null) return '';
	return Number(value).toLocaleString(lang === 'ar' ? 'ar-EG' : 'en', {
		maximumFractionDigits: 1
	});
}

// Grams of a macro, always marked as an estimate.
export function gramText(value, lang) {
	if (value == null) return '';
	return `≈ ${num(value, lang)} ${lang === 'ar' ? 'ج' : 'g'}`;
}

// Exercise energy uses the same approximate wording as food calories.
export function burnText(value, lang) {
	if (value == null || value <= 0) return '';
	return `≈ ${num(value, lang)} ${lang === 'ar' ? 'سعرة' : 'kcal'}`;
}
