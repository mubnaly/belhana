<script>
	import { onMount } from 'svelte';
	import { api, money, formatQty } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let recipes = [];
	let err = '';
	let loading = true;
	let searching = false;
	let lang = getLang();
	$: lang = $langStore;
	let search = '';
	let searchedTerm = '';
	let detail = null;
	let servings = 4;
	let showForm = false;
	let savedOnly = false;
	let saved = {};
	let searchTimer;
	let loadSeq = 0;
	let busy = false;

	const SAVED_KEY = 'belhana.savedRecipes';

	function emptyForm() {
		return {
			name_ar: '',
			name_en: '',
			description: '',
			meal_types: ['lunch'],
			difficulty: 'easy',
			prep_minutes: 15,
			cook_minutes: 30,
			default_servings: 4,
			ingredients: [{ product_name: '', quantity: 1, unit: 'kg' }],
			steps: [{ text_ar: '' }]
		};
	}

	let form = emptyForm();

	// Ask the server to search names, descriptions, ingredient names and tags.
	// The sequence number drops responses that arrive after a newer search.
	async function load(term = '') {
		const seq = ++loadSeq;
		const q = term.trim();
		if (q) searching = true;
		else loading = true;
		err = '';
		try {
			const data = await api.get(
				`/api/recipes?limit=200${q ? `&search=${encodeURIComponent(q)}` : ''}`
			);
			if (seq !== loadSeq) return;
			recipes = data;
			searchedTerm = q;
		} catch (e) {
			if (seq === loadSeq) err = e.message;
		}
		if (seq !== loadSeq) return;
		loading = false;
		searching = false;
	}

	function loadSaved() {
		try {
			const raw = window.localStorage.getItem(SAVED_KEY);
			saved = (raw && JSON.parse(raw)) || {};
		} catch {
			saved = {};
		}
	}

	function toggleSaved(r) {
		const next = { ...saved };
		if (next[r.id]) {
			delete next[r.id];
		} else {
			next[r.id] = {
				id: r.id,
				name_ar: r.name_ar,
				name_en: r.name_en,
				saved_at: Date.now()
			};
		}
		saved = next;
		try {
			window.localStorage.setItem(SAVED_KEY, JSON.stringify(next));
		} catch {
			/* storage unavailable */
		}
	}

	function onSearchInput(value) {
		search = value;
		clearTimeout(searchTimer);
		searchTimer = setTimeout(() => load(search), 300);
	}

	function submitSearch() {
		clearTimeout(searchTimer);
		load(search);
	}

	function searchWeb() {
		const q = searchedTerm || search.trim();
		const suffix = lang === 'ar' ? 'طريقة عمل' : 'recipe';
		const url = `https://www.google.com/search?q=${encodeURIComponent(`${q} ${suffix}`)}`;
		window.open(url, '_blank', 'noopener');
	}

	function addMissing() {
		const name = searchedTerm || search.trim();
		form = emptyForm();
		form.name_ar = name;
		showForm = true;
	}

	function toggleSavedOnly() {
		savedOnly = !savedOnly;
		// Saved-only should show every saved meal, not just the current query.
		if (savedOnly && searchedTerm) {
			clearTimeout(searchTimer);
			search = '';
			load('');
		}
	}

	onMount(() => {
		loadSaved();
		load();
		return () => clearTimeout(searchTimer);
	});

	async function open(r) {
		try {
			detail = await api.get(`/api/recipes/${r.id}`);
			servings = detail.recipe.default_servings;
			await cost();
		} catch (e) {
			err = e.message;
		}
	}

	async function cost() {
		if (!detail) return;
		try {
			detail.cost = await api.get(`/api/recipes/${detail.recipe.id}/cost?servings=${servings}`);
		} catch (e) {
			err = e.message;
		}
	}

	async function create() {
		err = '';
		busy = true;
		try {
			const body = {
				name_ar: form.name_ar,
				name_en: form.name_en || undefined,
				description: form.description || undefined,
				meal_types: form.meal_types,
				difficulty: form.difficulty,
				prep_minutes: form.prep_minutes,
				cook_minutes: form.cook_minutes,
				default_servings: form.default_servings,
				ingredients: form.ingredients
					.filter((i) => i.product_name.trim())
					.map((i) => ({ raw_name_ar: i.product_name, quantity: i.quantity, unit: i.unit })),
				steps: form.steps.filter((s) => s.text_ar.trim()).map((s) => ({ text_ar: s.text_ar }))
			};
			await api.post('/api/recipes', body);
			showForm = false;
			form = emptyForm();
			await load(search);
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	async function remove(r) {
		if (!confirm(t('delete') + '?')) return;
		try {
			await api.del(`/api/recipes/${r.id}`);
			if (detail?.recipe.id === r.id) detail = null;
			await load(search);
		} catch (e) {
			err = e.message;
		}
	}

	$: visible = savedOnly ? recipes.filter((r) => saved[r.id]) : recipes;
	$: savedCount = Object.keys(saved).length;
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">🍲 {t('recipes')}</div>
		<div style="display:flex; gap:.5rem">
			<button class="btn small" onclick={() => (showForm = !showForm)}>+ {t('addRecipe')}</button>
			<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
		</div>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if showForm}
		<div class="card">
			<h3>{t('addRecipe')}</h3>
			<label>{t('name')} (عربي)</label>
			<input bind:value={form.name_ar} />
			<div class="field-row">
				<div>
					<label>{t('prepTime')}</label>
					<input type="number" bind:value={form.prep_minutes} />
				</div>
				<div>
					<label>{t('cookTime')}</label>
					<input type="number" bind:value={form.cook_minutes} />
				</div>
				<div>
					<label>{t('servings')}</label>
					<input type="number" bind:value={form.default_servings} />
				</div>
			</div>
			<label>{t('mealTypes')}</label>
			<div style="display:flex; gap:.8rem; flex-wrap:wrap">
				{#each ['breakfast', 'lunch', 'dinner', 'snack', 'dessert'] as mt}
					<label style="display:flex; align-items:center; gap:.3rem; margin:0">
						<input type="checkbox" style="width:auto" checked={form.meal_types.includes(mt)} onchange={(e) => e.currentTarget.checked ? form.meal_types.push(mt) : (form.meal_types = form.meal_types.filter((x) => x !== mt))} />
						{t(mt)}
					</label>
				{/each}
			</div>

			<h3 style="margin-top:1rem">{t('ingredients')}</h3>
			{#each form.ingredients as ing, i}
				<div class="field-row" style="align-items:end">
					<div style="flex:2"><label>{t('product')}</label><input bind:value={ing.product_name} /></div>
					<div><label>{t('quantity')}</label><input type="number" step="0.01" bind:value={ing.quantity} /></div>
					<div><label>{t('unit')}</label><input bind:value={ing.unit} /></div>
					<button class="btn secondary small" onclick={() => form.ingredients.splice(i, 1)}>×</button>
				</div>
			{/each}
			<button class="btn secondary small" style="margin:.4rem 0" onclick={() => form.ingredients.push({ product_name: '', quantity: 1, unit: 'kg' })}>+ {t('addIngredient')}</button>

			<h3 style="margin-top:1rem">{t('steps')}</h3>
			{#each form.steps as st, i}
				<div class="field-row" style="align-items:end">
					<textarea rows="2" bind:value={st.text_ar}></textarea>
					<button class="btn secondary small" onclick={() => form.steps.splice(i, 1)}>×</button>
				</div>
			{/each}
			<button class="btn secondary small" style="margin:.4rem 0" onclick={() => form.steps.push({ text_ar: '' })}>+ {t('addStep')}</button>

			<button class="btn block" style="margin-top:.8rem" onclick={create} disabled={busy}>{t('save')}</button>
		</div>
	{/if}

	<div class="card" style="margin-bottom:.8rem">
		<label style="margin-top:0">{t('searchMeals')}</label>
		<div style="display:flex; gap:.5rem">
			<input
				value={search}
				oninput={(e) => onSearchInput(e.currentTarget.value)}
				onkeydown={(e) => e.key === 'Enter' && submitSearch()}
				placeholder="🔍 {t('searchMeals')}"
				style="margin:0"
			/>
			<button class="btn small" onclick={submitSearch} aria-label={t('search')}>🔍</button>
		</div>
		<div class="row" style="margin-top:.5rem; align-items:flex-start">
			<p class="muted" style="margin:0">{t('searchHint')}</p>
			<button class="btn small {savedOnly ? '' : 'secondary'}" onclick={toggleSavedOnly}>
				⭐ {savedOnly ? t('allRecipes') : `${t('savedOnly')} (${savedCount})`}
			</button>
		</div>
	</div>

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if searching}
		<p class="muted">⏳ {t('searching')}</p>
	{:else}
		{#if searchedTerm && !savedOnly}
			<p class="muted">{recipes.length} {t('resultsCount')}</p>
		{/if}

		<div class="grid2">
			{#each visible as r}
				<div
					class="stat meal-card"
					role="button"
					tabindex="0"
					onclick={() => open(r)}
					onkeydown={(e) => {
						if (e.key === 'Enter' || e.key === ' ') {
							e.preventDefault();
							open(r);
						}
					}}
					style="cursor:pointer; text-align:{lang === 'ar' ? 'right' : 'left'}"
				>
					<button
						class="star-btn"
						aria-pressed={Boolean(saved[r.id])}
						title={saved[r.id] ? t('removeSaved') : t('saveLocally')}
						aria-label={saved[r.id] ? t('removeSaved') : t('saveLocally')}
						onclick={(e) => {
							e.stopPropagation();
							toggleSaved(r);
						}}
						onkeydown={(e) => e.stopPropagation()}
					>
						{saved[r.id] ? '★' : '☆'}
					</button>
					<div class="v" style="font-size:.95rem">{r.name_ar}</div>
					<div class="k">{r.meal_types.map((m) => t(m)).join(' · ')}</div>
					<div class="k">{r.difficulty} · {r.prep_minutes + r.cook_minutes}{lang === 'ar' ? ' د' : 'm'}</div>
				</div>
			{/each}
		</div>

		{#if visible.length === 0}
			{#if searchedTerm && !savedOnly}
				<div class="card" style="text-align:center; margin-top:.8rem">
					<div style="font-size:2rem">🍽️</div>
					<h3 style="margin:.3rem 0">{t('mealNotFound')}</h3>
					<p class="muted">«{searchedTerm}»</p>
					<p class="muted">{t('mealNotFoundHint')}</p>
					<div style="display:flex; gap:.5rem; justify-content:center; flex-wrap:wrap">
						<button class="btn secondary" onclick={searchWeb}>🌐 {t('searchWeb')}</button>
						<button class="btn" onclick={addMissing}>+ {t('addMissing')}</button>
					</div>
					<p class="muted" style="margin-top:.5rem">{t('addMissingHint')}</p>
				</div>
			{:else}
				<p class="muted">{savedOnly ? t('noSaved') : t('empty')}</p>
			{/if}
		{/if}
	{/if}

	{#if detail}
		<div class="modal-bg" onclick={(e) => (e.target === e.currentTarget ? (detail = null) : null)}>
			<div class="modal">
				<div class="row">
					<h2>{detail.recipe.name_ar}</h2>
					<div style="display:flex; gap:.4rem">
						<button
							class="btn secondary small"
							aria-pressed={Boolean(saved[detail.recipe.id])}
							title={saved[detail.recipe.id] ? t('removeSaved') : t('saveLocally')}
							onclick={() => toggleSaved(detail.recipe)}
						>
							{saved[detail.recipe.id] ? '★' : '☆'}
						</button>
						<button class="btn secondary small" onclick={() => (detail = null)}>×</button>
					</div>
				</div>
				{#if detail.recipe.description}<p class="muted">{detail.recipe.description}</p>{/if}

				<div class="row" style="margin:.5rem 0">
					<label style="margin:0">{t('servings')}</label>
					<input type="number" min="1" max="20" bind:value={servings} style="width:80px" onchange={cost} />
				</div>

				{#if detail.cost}
					<div class="grid2" style="margin:.6rem 0">
						<div class="stat"><div class="v">{money(detail.cost.total_minor)}</div><div class="k">{t('total')}</div></div>
						<div class="stat"><div class="v">{money(detail.cost.per_serving_minor)}</div><div class="k">{t('perServing')}</div></div>
					</div>
					{#if detail.cost.unknown_count > 0}
						<p class="muted">⚠ {detail.cost.unknown_count} {lang === 'ar' ? 'مكونات بدون سعر معروف' : 'ingredients without known price'}</p>
					{/if}
				{/if}

				<h3>{t('ingredients')}</h3>
				<ul>
					{#each detail.ingredients as ing}
						<li>{ing.raw_name_ar ?? ing.product_id} — {formatQty(ing.quantity, ing.unit)}{ing.optional ? ' (' + t('optional') + ')' : ''}</li>
					{/each}
				</ul>

				<h3>{t('steps')}</h3>
				<ol>
					{#each detail.steps as st}
						<li>{st.text_ar}</li>
					{/each}
				</ol>

				{#if detail.recipe.household_id && !detail.recipe.is_system}
					<button class="btn danger small" onclick={() => remove(detail.recipe)}>{t('delete')}</button>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.meal-card {
		position: relative;
		padding-top: 1.7rem;
	}

	.star-btn {
		position: absolute;
		top: 0.35rem;
		inset-inline-end: 0.35rem;
		width: 1.9rem;
		height: 1.9rem;
		border-radius: 999px;
		border: 1px solid var(--border);
		background: var(--surface);
		color: var(--primary-dark);
		font-size: 1rem;
		line-height: 1;
		cursor: pointer;
	}

	.star-btn[aria-pressed='true'] {
		background: var(--primary-soft);
	}
</style>
