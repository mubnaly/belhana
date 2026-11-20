<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { getLang, t, toggleLang, langStore } from '$lib/i18n.js';
	import { isAuthed, getAuth, api, logout, formatQty } from '$lib/api.js';
	import { kcalText } from '$lib/format.js';
	import MacroFacts from '$lib/MacroFacts.svelte';
	import Icon from '$lib/Icon.svelte';

	let lang = getLang();
	$: lang = $langStore;
	let data = null;
	let err = '';
	let loading = true;

	$: authed = isAuthed();

	onMount(async () => {
		if (!authed) {
			goto(base + '/login');
			return;
		}
		try {
			data = await api.get('/api/dashboard');
		} catch (e) {
			if (e.status === 401) {
				logout();
				goto(base + '/login');
			} else {
				err = e.message;
			}
		}
		loading = false;
	});

	function money(minor) {
		if (minor == null) return '—';
		return `${(minor / 100).toLocaleString(lang === 'ar' ? 'ar-EG' : 'en')} ${lang === 'ar' ? 'ج.م' : 'EGP'}`;
	}

	const periodNames = { breakfast: 'breakfast', lunch: 'lunch', dinner: 'dinner', snack: 'snack', dessert: 'dessert', juice: 'juice' };

	const unitNames = {
		kg: 'كجم',
		g: 'جم',
		l: 'لتر',
		ml: 'مل',
		piece: 'قطعة',
		dozen: 'دستة',
		bundle: 'حزمة',
		pack: 'عبوة',
		carton: 'كرتونة'
	};

	function unitLabel(unit) {
		if (!unit) return '';
		return lang === 'ar' ? unitNames[unit] ?? unit : unit;
	}

	function roleIcon(role, name = '') {
		if (role === 'bread') return 'bread';
		if (role === 'salad') return 'salad';
		if (role === 'staple') return 'rice';
		if (role === 'snack') return name.includes('خضار') || name.includes('Vegetable') ? 'sprout' : 'apple';
		if (role === 'adama') return 'bowl';
		return 'plate';
	}

	// A course's calories are the sum of its dish and sides (estimates).
	function courseKcal(course) {
		const parts = [course.main, ...(course.sides ?? [])].filter(Boolean);
		const total = parts.reduce((sum, p) => sum + (p.kcal_per_serving ?? 0), 0);
		return total;
	}
</script>

<div class="page">
	<header class="topbar" style="margin: -1rem -1rem 1rem; width: calc(100% + 2rem)">
		<div class="brand"><img src="{base}/logo.svg" alt="" width="22" height="22" /> {t('appName')}</div>
		<div style="display:flex; gap:.5rem; align-items:center">
			<button class="btn secondary small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
			{#if authed}
				<button
					class="btn secondary small"
					onclick={() => {
						logout();
						goto(base + '/login');
					}}>{t('logout')}</button
				>
			{/if}
		</div>
	</header>

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if err}
		<div class="error-msg">{err}</div>
	{:else if data}
		{#if data.suggestion}
			<div class="notice"><Icon name="bulb" size={18} /><span>{data.suggestion}</span></div>
		{/if}

		<h2>{t('todayMenu')}</h2>
		<p class="muted kcal-hint">{t('kcalEstimate')}</p>
		{#if data.today_table.length === 0}
			<div class="card">
				<p class="muted">{t('empty')} — <a href="{base}/plan">{t('generatePlan')}</a></p>
			</div>
		{:else}
			<div class="notes today-notes">
				{#each data.today_table as course}
					<section class="note today-note">
						<div class="note-head">
							<span class="chip">{t(course.key)}</span>
							{#if course.prepared}<span class="chip ok"><Icon name="check" size={13} /> {t('cooked')}</span>{/if}
							{#if course.suggested}<span class="chip warn">{t('suggested')}</span>{/if}
							<span class="kcal estimate">{kcalText(courseKcal(course), lang)}</span>
						</div>
						{#if course.main}
							<div class="course-line">
								<span class="course-role"><Icon name={roleIcon(course.main.role, course.main.name_ar)} size={18} /></span>
								{#if course.main.recipe_id}
									<a class="dish-name" href={`${base}/recipes?open=${course.main.recipe_id}`}>{course.main.name_ar}</a>
								{:else}
									<span class="dish-name">{course.main.name_ar}</span>
								{/if}
								<span class="kcal estimate">{kcalText(course.main.kcal_per_serving, lang)}</span>
								<MacroFacts macros={course.main.macros_per_serving} {lang} />
							</div>
						{/if}
						{#each course.sides as side}
							<div class="course-line side">
								<span class="course-role"><Icon name={roleIcon(side.role, side.name_ar)} size={18} /></span>
								{#if side.recipe_id}
									<a class="dish-name" href={`${base}/recipes?open=${side.recipe_id}`}>{side.name_ar}</a>
								{:else}
									<span class="dish-name">{side.name_ar}</span>
								{/if}
								<span class="kcal estimate">{kcalText(side.kcal_per_serving, lang)}</span>
								<MacroFacts macros={side.macros_per_serving} {lang} />
							</div>
						{/each}
					</section>
				{/each}
			</div>
			<p class="muted" style="margin:.15rem 0 .5rem">{t('sideSuggestHint')}</p>
			<div class="row" style="margin-top:.7rem">
				<span class="muted">{t('tapDishHint')}</span>
				<a href="{base}/recipes" class="btn small">{t('recipes')}</a>
			</div>
		{/if}

		{#if data.quick_basics?.length}
			<h2>{t('quickBasics')}</h2>
			<p class="muted" style="margin-top:-.3rem">{t('basicsHint')}</p>
			<div class="card">
				<div class="basics-grid">
					{#each data.quick_basics as b}
						<a class="basic-item" href={`${base}/recipes?open=${b.recipe_id}`}>
							<span class="basic-icon"><Icon name={roleIcon(b.role, b.name_ar)} size={22} /></span>
							<span class="basic-name">{b.name_ar}</span>
							<span class="kcal estimate">{kcalText(b.kcal_per_serving, lang)}</span>
							<MacroFacts macros={b.macros_per_serving} {lang} />
						</a>
					{/each}
				</div>
				<a href="{base}/recipes" class="btn secondary small" style="margin-top:.7rem">{t('allRecipes')}</a>
			</div>
		{/if}

		{#if data.budget}
			<h2>{t('budgetStatus')}</h2>
			<div class="card">
				<div class="grid2">
					<div class="stat"><div class="v">{money(data.budget.budget_minor)}</div><div class="k">{t('weeklyBudget')}</div></div>
					<div class="stat"><div class="v">{money(data.budget.actual_spent_minor)}</div><div class="k">{t('spent')}</div></div>
					<div class="stat"><div class="v">{money(data.budget.planned_cost_minor)}</div><div class="k">{t('planned')}</div></div>
					<div class="stat"><div class="v">{money(data.budget.remaining_minor)}</div><div class="k">{t('remaining')}</div></div>
				</div>
				{#if data.budget.budget_minor > 0}
					{@const pct = Math.min(100, Math.round(((data.budget.actual_spent_minor + data.budget.planned_cost_minor) / data.budget.budget_minor) * 100))}
					<div class="progress" class:over={pct >= 100}><div style="width:{pct}%"></div></div>
					<p class="muted">{pct}%</p>
				{/if}
			</div>
		{/if}

		{#if data.shopping_list_status}
			<h2>{t('nextShopping')}</h2>
			<div class="card row">
				<span>{data.shopping_list_status.purchased_items}/{data.shopping_list_status.total_items} {t('purchased')}</span>
				<span><b>{money(data.shopping_list_status.estimated_total_minor)}</b></span>
				<a href="{base}/shopping" class="btn small">{t('shopping')}</a>
			</div>
		{/if}

		{#if data.expiring_stock.length > 0}
			<h2>{t('expiring')}</h2>
			<div class="card">
				{#each data.expiring_stock as lot}
					<div class="row" style="padding:.3rem 0">
						<span>
							{lot.product_name_ar} — {formatQty(lot.quantity, lot.unit)}
							{#if lot.estimated_kcal}<br /><span class="kcal estimate">{kcalText(lot.estimated_kcal, lang)}</span>{/if}
							<MacroFacts macros={lot.estimated_macros} {lang} />
						</span>
						<span class="chip warn">{lot.expiry_date}</span>
					</div>
				{/each}
				<a href="{base}/inventory">{t('inventory')} ←</a>
			</div>
		{/if}

		{#if data.recent_prices.length > 0}
			<h2>{t('recentPrices')}</h2>
			<div class="card">
				<div class="table-wrap">
					<table class="table">
						<thead><tr><th>{t('product')}</th><th>{t('amount')}</th><th>{t('observationDate')}</th></tr></thead>
						<tbody>
							{#each data.recent_prices as p}
								<tr>
									<td>{p.product_name_ar}</td>
									<td>{p.amount_minor != null ? money(p.amount_minor) : '—'} {unitLabel(p.unit)}</td>
									<td><span class="chip {p.review_status === 'verified' ? 'ok' : 'warn'}">{t(p.review_status)}</span> {p.observation_date ?? ''}</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		{/if}
	{/if}
</div>

<style>
	.kcal-hint {
		margin-top: -0.45rem;
	}

	.today-note {
		display: flex;
		flex-direction: column;
		gap: 0.15rem;
	}

	.course-line {
		display: grid;
		grid-template-columns: auto minmax(0, 1fr) auto;
		align-items: start;
		column-gap: 0.4rem;
		row-gap: 0.1rem;
		padding: 0.12rem 0;
		min-width: 0;
	}

	/* Wrap only between real words: never split a dish name mid-word just
	   because the note is narrow. */
	.course-line .dish-name {
		min-width: 0;
		overflow-wrap: normal;
		word-break: normal;
		hyphens: none;
	}

	.course-line .kcal {
		grid-column: 3;
		white-space: nowrap;
	}

	.course-line :global(.macro-facts) {
		grid-column: 2 / -1;
		margin-top: 0;
	}

	.course-line.side {
		font-size: 0.92rem;
	}

	.course-role {
		grid-column: 1;
		display: inline-flex;
		flex-shrink: 0;
		line-height: 1.2;
		color: var(--primary-dark);
	}

	.basics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(130px, 1fr));
		gap: 0.5rem;
	}

	.basic-item {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 0.1rem;
		padding: 0.55rem 0.65rem;
		border: 2px solid var(--border-strong);
		border-radius: 8px 14px 8px 13px;
		background: var(--surface-2);
		color: var(--text);
		text-decoration: none;
		box-shadow: 2px 3px 0 rgba(96, 76, 45, 0.12);
	}

	:global([data-theme='dark']) .basic-item {
		box-shadow: 2px 3px 0 rgba(0, 0, 0, 0.3);
	}

	.basic-item:hover {
		border-color: var(--primary);
	}

	.basic-icon {
		display: inline-flex;
		line-height: 1.1;
	}

	.basic-name {
		font-weight: 600;
		font-size: 0.9rem;
		overflow-wrap: normal;
		word-break: normal;
	}
</style>
