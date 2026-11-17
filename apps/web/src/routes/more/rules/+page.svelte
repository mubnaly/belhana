<script>
	import { onMount } from 'svelte';
	import { api } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let rules = [];
	let err = '';
	let loading = true;
	let lang = getLang();
	$: lang = $langStore;
	let busy = false;
	let showAdd = false;

	let form = { rule_type: 'meal_repeat_limit', priority: 'hard', maximum_occurrences: 2, period_days: 7, product_id: '', category_id: '', minimum_per_week: 2 };

	const ruleTypes = [
		{ value: 'meal_repeat_limit', ar: 'تكرار وجبة', en: 'Meal repeat limit', need: 'counts' },
		{ value: 'dish_frequency_limit', ar: 'حد ظهور طبق', en: 'Dish frequency limit', need: 'counts' },
		{ value: 'dish_consecutive_limit', ar: 'طبق في أيام متتالية', en: 'Consecutive-days limit', need: 'target' },
		{ value: 'min_legume_meals', ar: 'حد أدنى للبقوليات', en: 'Min legume meals', need: 'min' },
		{ value: 'use_freezer_first', ar: 'استخدم الفريزر أولًا', en: 'Use freezer first', need: 'none' }
	];

	async function load() {
		loading = true;
		try {
			rules = await api.get('/api/rules');
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	function paramsFor() {
		const need = ruleTypes.find((r) => r.value === form.rule_type)?.need;
		const params = {};
		if (need === 'counts') {
			params.maximum_occurrences = form.maximum_occurrences;
			params.period_days = form.period_days;
		} else if (need === 'target') {
			if (form.product_id) params.product_id = form.product_id;
			else params.category_id = form.category_id || 'fish';
		} else if (need === 'min') {
			params.minimum_per_week = form.minimum_per_week;
		}
		return params;
	}

	async function addRule() {
		busy = true;
		err = '';
		try {
			await api.post('/api/rules', { rule_type: form.rule_type, priority: form.priority, params: paramsFor() });
			showAdd = false;
			await load();
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	async function toggle(r) {
		try {
			await api.put(`/api/rules/${r.id}/enabled`, { enabled: !r.enabled });
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function remove(id) {
		if (!confirm(t('delete') + '?')) return;
		try {
			await api.del(`/api/rules/${id}`);
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	function typeName(rt) {
		const def = ruleTypes.find((r) => r.value === rt);
		return def ? (lang === 'ar' ? def.ar : def.en) : rt;
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">📏 {t('rules')}</div>
		<div style="display:flex; gap:.5rem">
			<button class="btn small" onclick={() => (showAdd = !showAdd)}>+ {t('addRule')}</button>
			<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
		</div>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	<p class="muted">{lang === 'ar' ? 'قواعد أسرك الصارمة والمفضلة — الخطة لا تتجاوز الصارمة أبدًا.' : 'Hard and soft household rules — the planner never violates hard rules.'}</p>

	{#if showAdd}
		<div class="card">
			<label>{t('ruleType')}</label>
			<select bind:value={form.rule_type}>
				{#each ruleTypes as rt}<option value={rt.value}>{lang === 'ar' ? rt.ar : rt.en}</option>{/each}
			</select>
			<label>{t('priority')}</label>
			<select bind:value={form.priority}>
				<option value="hard">{t('hard')}</option>
				<option value="soft">{t('soft')}</option>
			</select>

			{#if form.rule_type === 'meal_repeat_limit' || form.rule_type === 'dish_frequency_limit'}
				<div class="field-row">
					<div><label>{lang === 'ar' ? 'أقصى تكرار' : 'Max occurrences'}</label><input type="number" min="1" bind:value={form.maximum_occurrences} /></div>
					<div><label>{lang === 'ar' ? 'خلال أيام' : 'Within days'}</label><input type="number" min="1" bind:value={form.period_days} /></div>
				</div>
			{:else if form.rule_type === 'dish_consecutive_limit'}
				<label>{lang === 'ar' ? 'معرّف المنتج (مثال: سمك)' : 'Product id (e.g. fish)'}</label>
				<input bind:value={form.product_id} placeholder="product_id" />
			{:else if form.rule_type === 'min_legume_meals'}
				<label>{lang === 'ar' ? 'أدنى عدد أسبوعي' : 'Minimum per week'}</label>
				<input type="number" min="1" bind:value={form.minimum_per_week} />
			{/if}

			<button class="btn" style="margin-top:.8rem" onclick={addRule} disabled={busy}>{t('add')}</button>
		</div>
	{/if}

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else}
		{#each rules as r}
			<div class="card row">
				<div>
					<b>{typeName(r.rule_type)}</b>
					<span class="chip {r.priority === 'hard' ? 'bad' : ''}">{t(r.priority)}</span>
					{#if r.notes}<span class="muted">· {r.notes}</span>{/if}
				</div>
				<div style="display:flex; gap:.4rem">
					<button class="btn secondary small" onclick={() => toggle(r)}>{r.enabled ? '✅' : '⬜'} {t('enabled')}</button>
					<button class="btn secondary small" onclick={() => remove(r.id)}>🗑</button>
				</div>
			</div>
		{/each}
		{#if rules.length === 0}<p class="muted">{t('empty')}</p>{/if}
	{/if}
</div>
