<script>
	import { onMount } from 'svelte';
	import { api, money } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let recipes = [];
	let err = '';
	let loading = true;
	let lang = getLang();
	$: lang = $langStore;
	let search = '';
	let detail = null;
	let servings = 4;
	let showForm = false;

	let form = {
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

	async function load() {
		loading = true;
		try {
			recipes = await api.get('/api/recipes?limit=100');
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

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
			form = {
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
			await load();
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
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	$: filtered = search
		? recipes.filter((r) => r.name_ar.includes(search) || (r.name_en ?? '').toLowerCase().includes(search.toLowerCase()))
		: recipes;

	let busy = false;
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

	<input placeholder="🔍 {t('search')}…" bind:value={search} style="margin-bottom:.8rem" />

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else}
		<div class="grid2">
			{#each filtered as r}
				<button class="stat" style="cursor:pointer; text-align:{lang === 'ar' ? 'right' : 'left'}" onclick={() => open(r)}>
					<div class="v" style="font-size:.95rem">{r.name_ar}</div>
					<div class="k">{r.meal_types.map((m) => t(m)).join(' · ')}</div>
					<div class="k">{r.difficulty} · {r.prep_minutes + r.cook_minutes}{lang === 'ar' ? ' د' : 'm'}</div>
				</button>
			{/each}
		</div>
		{#if filtered.length === 0}<p class="muted">{t('empty')}</p>{/if}
	{/if}

	{#if detail}
		<div class="modal-bg" onclick={(e) => (e.target === e.currentTarget ? (detail = null) : null)}>
			<div class="modal">
				<div class="row">
					<h2>{detail.recipe.name_ar}</h2>
					<button class="btn secondary small" onclick={() => (detail = null)}>×</button>
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
						<li>{ing.raw_name_ar ?? ing.product_id} — {ing.quantity} {ing.unit}{ing.optional ? ' (' + t('optional') + ')' : ''}</li>
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
