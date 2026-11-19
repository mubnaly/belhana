<script>
	// Compact, estimate-labelled macro chips shared by product and meal cards.
	import { t } from '$lib/i18n.js';
	import { gramText } from '$lib/format.js';
	import Icon from '$lib/Icon.svelte';

	let {
		macros = null,
		lang = 'ar',
		compact = true,
		showFiber = false
	} = $props();

	let has = $derived(
		!!macros && (macros.protein_g != null || macros.carbs_g != null || macros.fat_g != null || (showFiber && macros.fiber_g != null))
	);

	function short(key) {
		if (!compact) return t(key);
		if (lang === 'ar') return { protein: 'بروتين', carbs: 'كربوهيدرات', fat: 'دهون', fiber: 'ألياف' }[key];
		return { protein: 'protein', carbs: 'carbs', fat: 'fat', fiber: 'fibre' }[key];
	}
</script>

{#if has}
	<span class="macro-facts" title={t('estimateNote')}>
		<Icon name="leaf" size={12} />
		{#if macros.protein_g != null}<span>{short('protein')} {gramText(macros.protein_g, lang)}</span>{/if}
		{#if macros.carbs_g != null}<span>{short('carbs')} {gramText(macros.carbs_g, lang)}</span>{/if}
		{#if macros.fat_g != null}<span>{short('fat')} {gramText(macros.fat_g, lang)}</span>{/if}
		{#if showFiber && macros.fiber_g != null}<span>{short('fiber')} {gramText(macros.fiber_g, lang)}</span>{/if}
	</span>
{/if}
