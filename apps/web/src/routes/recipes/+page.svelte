<script>
	import { onMount } from 'svelte';
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { api, money, formatQty, getAuth, setHousehold } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import { kcalText } from '$lib/format.js';
	import MacroFacts from '$lib/MacroFacts.svelte';
	import Icon from '$lib/Icon.svelte';

	let recipes = [];
	let err = '';
	let loading = true;
	let searching = false;
	let lang = getLang();
	$: lang = $langStore;
	let search = '';
	let searchedTerm = '';
	let detail = null;
	let servingsInput = '4';
	let householdSize = null;
	let costTimer;
	let costSeq = 0;
	let showForm = false;
	let savedOnly = false;
	let saved = {};
	let searchTimer;
	let loadSeq = 0;
	let busy = false;
	let selectedDiet = null;

	const SAVED_KEY = 'belhana.savedRecipes';

	const dietLabels = {
		mediterranean: { ar: 'حمية البحر المتوسط', en: 'Mediterranean diet' },
		tayyibat: { ar: 'نظام الطيبات', en: 'Tayyibat system' },
		keto: { ar: 'الكيتو (منخفض الكربوهيدرات)', en: 'Keto (low-carbohydrate)' },
		elimination: { ar: 'نظام الاستبعاد', en: 'Elimination diet' },
		balanced_family: { ar: 'نمط متوازن للأسرة', en: 'Balanced family pattern' }
	};
	$: activeDietName = selectedDiet
		? (lang === 'ar' ? dietLabels[selectedDiet]?.ar : dietLabels[selectedDiet]?.en) || selectedDiet
		: '';

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

	function clampServings(value) {
		const n = Math.floor(Number(value));
		return Number.isFinite(n) ? Math.min(20, Math.max(1, n)) : 1;
	}

	function scaleMacros(macros, factor) {
		if (!macros) return macros;
		const out = {};
		for (const [key, value] of Object.entries(macros)) {
			out[key] = typeof value === 'number' ? Math.round(value * factor * 10) / 10 : value;
		}
		return out;
	}

	// The number of family members is the household size. The count can also be
	// overridden by a deep link (?open=id&servings=n) from the weekly plan.
	async function open(r, overrideServings) {
		try {
			const fresh = await api.get(`/api/recipes/${r.id}`);
			detail = fresh;
			detail.cost = null;
			const cachedSize = Number(getAuth().household?.household_size || householdSize || 0);
			const initial = Number(overrideServings || cachedSize || fresh.recipe.default_servings || 1);
			servingsInput = String(clampServings(initial));
			await refreshCost();
			void syncHousehold();
		} catch (e) {
			err = e.message;
		}
	}

	async function refreshCost() {
		if (!detail) return;
		const seq = ++costSeq;
		const recipeId = detail.recipe.id;
		const n = servings;
		try {
			const data = await api.get(`/api/recipes/${recipeId}/cost?servings=${n}`);
			if (seq !== costSeq || !detail || detail.recipe.id !== recipeId) return;
			detail.cost = data;
		} catch (e) {
			if (seq === costSeq) err = e.message;
		}
	}

	async function syncHousehold() {
		const hh = getAuth().household;
		if (!hh?.id || Number(hh.household_size) === servings) return;
		try {
			setHousehold(await api.put('/api/household', { household_size: servings }));
		} catch {
			// Keep the local preview even when the household save is unavailable.
		}
	}

	function scheduleDetailRefresh() {
		clearTimeout(costTimer);
		costTimer = setTimeout(() => {
			void refreshCost();
			void syncHousehold();
		}, 180);
	}

	function onServingsInput(e) {
		servingsInput = e.currentTarget.value;
		scheduleDetailRefresh();
	}

	function onServingsBlur() {
		servingsInput = String(servings);
		clearTimeout(costTimer);
		void refreshCost();
		void syncHousehold();
	}

	onMount(() => {
		loadSaved();
		const openId = page.url.searchParams.get('open');
		const openServings = page.url.searchParams.get('servings');
		(async () => {
			// Read the household preference first so the very first recipe list
			// and the banner already reflect the selected diet.
			try {
				const hh = await api.get('/api/household');
				householdSize = hh?.household_size || householdSize;
				selectedDiet = hh?.selected_diet_id || null;
			} catch {
				// The list still works; the API applies the household filter.
			}
			await load();
			if (openId) await open({ id: openId }, openServings);
		})();
		return () => {
			clearTimeout(searchTimer);
			clearTimeout(costTimer);
		};
	});

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

	// Legacy Svelte reactivity: every displayed number follows the member count
	// immediately; only the priced cost needs a short debounced API refresh.
	$: servings = clampServings(servingsInput);
	$: defaultServings = Math.max(1, Number(detail?.recipe?.default_servings) || 1);
	$: scale = detail ? servings / defaultServings : 1;
	$: shownIngredients = detail
		? detail.ingredients.map((ing) => ({
				...ing,
				quantity: Number(ing.quantity || 0) * scale,
				kcal: ing.kcal == null ? ing.kcal : Number(ing.kcal) * scale,
				macros: ing.macros ? scaleMacros(ing.macros, scale) : ing.macros
			}))
		: [];
	$: totalKcal = detail?.recipe?.kcal_per_serving
		? Number(detail.recipe.kcal_per_serving) * servings
		: null;
	$: totalMacros = detail?.recipe?.macros_per_serving
		? scaleMacros(detail.recipe.macros_per_serving, servings)
		: null;
	$: visible = savedOnly ? recipes.filter((r) => saved[r.id]) : recipes;
	$: savedCount = Object.keys(saved).length;
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="pot" size={16} /> {t('recipes')}</div>
		<div style="display:flex; gap:.5rem">
			<button class="btn small" onclick={() => (showForm = !showForm)}>+ {t('addRecipe')}</button>
			<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
		</div>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if selectedDiet}
		<div class="card" style="padding:.6rem; margin-bottom:.7rem; background:var(--surface-2)">
			<div class="row" style="justify-content:space-between; gap:.5rem; flex-wrap:wrap">
				<div>
					<span class="chip ok"><Icon name="diet" size={14} /> {t('selectedDiet')}: {activeDietName}</span>
					<p class="muted" style="margin:.35rem 0 0">{t('dietFilterNote')}</p>
				</div>
				<a class="btn secondary small" href="{base}/more/diets"><Icon name="pencil" size={14} /> {t('changeDiet')}</a>
			</div>
		</div>
	{/if}

	{#if showForm}
		<div class="card">
			<h3>{t('addRecipe')}</h3>
			<label for="belhana-field-1">{t('name')} (عربي)</label>
			<input bind:value={form.name_ar} id="belhana-field-1"/>
			<div class="field-row">
				<div>
					<label for="belhana-field-2">{t('prepTime')}</label>
					<input type="number" bind:value={form.prep_minutes} id="belhana-field-2"/>
				</div>
				<div>
					<label for="belhana-field-3">{t('cookTime')}</label>
					<input type="number" bind:value={form.cook_minutes} id="belhana-field-3"/>
				</div>
				<div>
					<label for="belhana-field-4">{t('servings')}</label>
					<input type="number" bind:value={form.default_servings} id="belhana-field-4"/>
				</div>
			</div>
			<div class="field-label">{t('mealTypes')}</div>
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
					<div style="flex:2"><label for="belhana-field-5">{t('product')}</label><input bind:value={ing.product_name} id="belhana-field-5"/></div>
					<div><label for="belhana-field-6">{t('quantity')}</label><input type="number" step="0.01" bind:value={ing.quantity} id="belhana-field-6"/></div>
					<div><label for="belhana-field-7">{t('unit')}</label><input bind:value={ing.unit} id="belhana-field-7"/></div>
					<button class="btn secondary small" onclick={() => form.ingredients.splice(i, 1)} title={t('delete')} aria-label={t('delete')}><Icon name="x" size={14} /></button>
				</div>
			{/each}
			<button class="btn secondary small" style="margin:.4rem 0" onclick={() => form.ingredients.push({ product_name: '', quantity: 1, unit: 'kg' })}>+ {t('addIngredient')}</button>

			<h3 style="margin-top:1rem">{t('steps')}</h3>
			{#each form.steps as st, i}
				<div class="field-row" style="align-items:end">
					<textarea rows="2" bind:value={st.text_ar}></textarea>
					<button class="btn secondary small" onclick={() => form.steps.splice(i, 1)} title={t('delete')} aria-label={t('delete')}><Icon name="x" size={14} /></button>
				</div>
			{/each}
			<button class="btn secondary small" style="margin:.4rem 0" onclick={() => form.steps.push({ text_ar: '' })}>+ {t('addStep')}</button>

			<button class="btn block" style="margin-top:.8rem" onclick={create} disabled={busy}>{t('save')}</button>
		</div>
	{/if}

	<div class="card" style="margin-bottom:.8rem">
		<label style="margin-top:0" for="belhana-field-8">{t('searchMeals')}</label>
		<div style="display:flex; gap:.5rem">
			<input
				value={search}
				oninput={(e) => onSearchInput(e.currentTarget.value)}
				onkeydown={(e) => e.key === 'Enter' && submitSearch()}
				placeholder={t('searchMeals')}
				style="margin:0" id="belhana-field-8"/>
			<button class="btn small" onclick={submitSearch} aria-label={t('search')}><Icon name="search" size={17} /></button>
		</div>
		<div class="row" style="margin-top:.5rem; align-items:flex-start">
			<p class="muted" style="margin:0">{t('searchHint')}</p>
			<button class="btn small {savedOnly ? '' : 'secondary'}" onclick={toggleSavedOnly}>
				<Icon name="star" size={15} /> {savedOnly ? t('allRecipes') : `${t('savedOnly')} (${savedCount})`}
			</button>
		</div>
	</div>

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if searching}
		<p class="muted"><Icon name="clock" size={15} /> {t('searching')}</p>
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
						<Icon name={saved[r.id] ? 'star-filled' : 'star'} size={17} />
					</button>
					<div class="v" style="font-size:.95rem">{r.name_ar}</div>
					<div class="k">{r.meal_types.map((m) => t(m)).join(' · ')}</div>
					<div class="k">{r.difficulty} · {r.prep_minutes + r.cook_minutes}{lang === 'ar' ? ' د' : 'm'}</div>
					<div class="k kcal estimate">{kcalText(r.kcal_per_serving, lang)}</div>
					<MacroFacts macros={r.macros_per_serving} {lang} />
				</div>
			{/each}
		</div>

		{#if visible.length === 0}
			{#if searchedTerm && !savedOnly}
				<div class="card" style="text-align:center; margin-top:.8rem">
					<div class="empty-icon"><Icon name="plate" size={38} /></div>
					<h3 style="margin:.3rem 0">{t('mealNotFound')}</h3>
					<p class="muted">«{searchedTerm}»</p>
					<p class="muted">{t('mealNotFoundHint')}</p>
					<div style="display:flex; gap:.5rem; justify-content:center; flex-wrap:wrap">
						<button class="btn secondary" onclick={searchWeb}><Icon name="globe" size={16} /> {t('searchWeb')}</button>
						<button class="btn" onclick={addMissing}>+ {t('addMissing')}</button>
					</div>
					<p class="muted" style="margin-top:.5rem">{t('addMissingHint')}</p>
				</div>
			{:else}
				{#if selectedDiet && !savedOnly}
					<div class="card" style="text-align:center; margin-top:.8rem">
						<div class="empty-icon"><Icon name="diet" size={38} /></div>
						<h3 style="margin:.3rem 0">{t('noDietMeals')}</h3>
						<p class="muted">{activeDietName}</p>
						<a class="btn small" href="{base}/more/diets"><Icon name="pencil" size={14} /> {t('changeDiet')}</a>
					</div>
				{:else}
					<p class="muted">{savedOnly ? t('noSaved') : t('empty')}</p>
				{/if}
			{/if}
		{/if}
	{/if}

	{#if detail}
		<div
		class="modal-bg"
		role="presentation"
		onclick={(e) => (e.target === e.currentTarget ? (detail = null) : null)}
		onkeydown={(e) => e.key === 'Escape' && (detail = null)}
	>
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
							<Icon name={saved[detail.recipe.id] ? 'star-filled' : 'star'} size={17} />
						</button>
						<button class="btn secondary small" onclick={() => (detail = null)} aria-label={t('cancel')}><Icon name="x" size={16} /></button>
					</div>
				</div>
				{#if detail.recipe.description}<p class="muted">{detail.recipe.description}</p>{/if}

				<div class="row" style="margin:.5rem 0">
					<label style="margin:0" for="belhana-field-9">{t('householdSize')} / {t('servings')}</label>
					<input
						type="number"
						min="1"
						max="20"
						value={servingsInput}
						oninput={onServingsInput}
						onblur={onServingsBlur}
						style="width:80px"
						id="belhana-field-9"/>
				</div>
				<p class="muted" style="margin:-.2rem 0 .5rem">{t('liveServingsHint')}</p>

				<div class="kcal-detail">
					<span class="kcal estimate" title={t('kcalEstimate')}>{kcalText(totalKcal, lang)}</span>
					{#if !totalKcal}<span class="muted">{t('kcalUnknown')}</span>{/if}
					<span class="muted">{t('total')} · {servings} {t('servings')}</span>
					<MacroFacts macros={totalMacros} {lang} compact={false} showFiber={true} />
				</div>
				<p class="muted" style="margin:.1rem 0 .5rem">
					{t('perServing')}: {kcalText(detail.recipe.kcal_per_serving, lang)}
					<MacroFacts macros={detail.recipe.macros_per_serving} {lang} />
				</p>

				{#if detail.cost}
					<div class="grid2" style="margin:.6rem 0">
						<div class="stat"><div class="v">{money(detail.cost.total_minor)}</div><div class="k">{t('total')}</div></div>
						<div class="stat"><div class="v">{money(detail.cost.per_serving_minor)}</div><div class="k">{t('perServing')}</div></div>
					</div>
					{#if detail.cost.unknown_count > 0}
						<p class="muted"><Icon name="warn" size={15} /> {detail.cost.unknown_count} {lang === 'ar' ? 'مكونات بدون سعر معروف' : 'ingredients without known price'}</p>
					{/if}
				{/if}

				<h3>{t('ingredients')}</h3>
				<ul class="ingredient-list">
					{#each shownIngredients as ing}
						<li>
							<span>{ing.raw_name_ar ?? ing.product_id} — {formatQty(ing.quantity, ing.unit)}{ing.optional ? ' (' + t('optional') + ')' : ''}</span>
							{#if ing.kcal}<span class="kcal estimate">{kcalText(ing.kcal, lang)}</span>{/if}
							<MacroFacts macros={ing.macros} {lang} />
						</li>
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

	.empty-icon {
		display: inline-flex;
		color: var(--muted);
		margin-top: 0.3rem;
	}

	.kcal-detail {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		flex-wrap: wrap;
		background: var(--surface-2);
		border-radius: 10px;
		padding: 0.35rem 0.6rem;
		margin-bottom: 0.3rem;
	}

	.ingredient-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.ingredient-list li {
		display: grid;
		grid-template-columns: minmax(0, 1fr) auto;
		align-items: baseline;
		column-gap: 0.5rem;
		row-gap: 0.1rem;
		padding: 0.2rem 0;
		border-bottom: 1px solid var(--border);
	}

	.ingredient-list li > span:first-child {
		min-width: 0;
		overflow-wrap: normal;
		word-break: normal;
	}

	.ingredient-list li .kcal {
		grid-column: 2;
		white-space: nowrap;
	}

	.ingredient-list li :global(.macro-facts) {
		grid-column: 1 / -1;
		margin-top: 0;
	}

	.ingredient-list li:last-child {
		border-bottom: none;
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
