<script>
	import { onMount } from 'svelte';
	import { api } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let tips = [];
	let categories = [];
	let cat = '';
	let err = '';
	let loading = true;
	let lang = getLang();
	$: lang = $langStore;

	async function load() {
		loading = true;
		err = '';
		try {
			const [list, cats] = await Promise.all([
				api.get(`/api/meal-tips${cat ? `?category=${encodeURIComponent(cat)}` : ''}`),
				api.get('/api/meal-tips/categories')
			]);
			tips = list;
			categories = cats;
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	function catName(c) {
		const names = {
			general: lang === 'ar' ? 'عام' : 'General',
			rice: lang === 'ar' ? 'الأرز' : 'Rice',
			bread: lang === 'ar' ? 'الخبز' : 'Bread',
			spices: lang === 'ar' ? 'التوابل' : 'Spices',
			leftovers: lang === 'ar' ? 'البقايا' : 'Leftovers',
			legumes: lang === 'ar' ? 'البقوليات' : 'Legumes',
			batch: lang === 'ar' ? 'الطهي بالكميات' : 'Batch cooking',
			prep: lang === 'ar' ? 'التحضير' : 'Prep',
			budget: lang === 'ar' ? 'الميزانية' : 'Budget',
			planning: lang === 'ar' ? 'التخطيط' : 'Planning'
		};
		return names[c] ?? c;
	}

	function title(tip) {
		return lang === 'ar' ? tip.title_ar : tip.title_en;
	}

	function body(tip) {
		return lang === 'ar' ? tip.body_ar : tip.body_en;
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">💡 {t('tips')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}
	<p class="muted">{t('tipsHint')}</p>

	<div class="card" style="padding:.6rem">
		<div style="display:flex; gap:.3rem; flex-wrap:wrap">
			<button class="chip {cat === '' ? 'ok' : ''}" style="cursor:pointer" onclick={() => { cat = ''; load(); }}>{t('allTips')}</button>
			{#each categories as c}
				<button class="chip {cat === c ? 'ok' : ''}" style="cursor:pointer" onclick={() => { cat = c; load(); }}>{catName(c)}</button>
			{/each}
		</div>
	</div>

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if tips.length === 0}
		<p class="muted">{t('empty')}</p>
	{:else}
		{#each tips as tip}
			<details class="card" style="padding:.8rem; margin-bottom:.5rem">
				<summary style="cursor:pointer; font-weight:600">
					{title(tip)} <span class="chip">{catName(tip.category)}</span>
				</summary>
				<p style="margin-top:.7rem; line-height:1.8">{body(tip)}</p>
			</details>
		{/each}
	{/if}
</div>
