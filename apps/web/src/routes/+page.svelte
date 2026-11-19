<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { getLang, t, toggleLang, langStore } from '$lib/i18n.js';
	import { isAuthed, getAuth, api, logout, formatQty } from '$lib/api.js';

	let lang = getLang();
	$: lang = $langStore;
	let data = null;
	let err = '';
	let loading = true;

	$: authed = isAuthed();

	onMount(async () => {
		if (!authed) {
			goto('/login');
			return;
		}
		try {
			data = await api.get('/api/dashboard');
		} catch (e) {
			if (e.status === 401) {
				logout();
				goto('/login');
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
		if (role === 'bread') return '🥖';
		if (role === 'salad') return '🥗';
		if (role === 'staple') return '🍚';
		if (role === 'snack') return name.includes('خضار') || name.includes('Vegetable') ? '🥕' : '🍎';
		if (role === 'adama') return '🥣';
		return '🍽️';
	}
</script>

<div class="page">
	<header class="topbar" style="margin: -1rem -1rem 1rem; width: calc(100% + 2rem)">
		<div class="brand">🌿 {t('appName')}</div>
		<div style="display:flex; gap:.5rem; align-items:center">
			<button class="btn secondary small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
			{#if authed}
				<button
					class="btn secondary small"
					onclick={() => {
						logout();
						goto('/login');
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
			<div class="notice">💡 {data.suggestion}</div>
		{/if}

		<h2>{t('todayMenu')}</h2>
		<div class="card">
			{#if data.today_table.length === 0}
				<p class="muted">{t('empty')} — <a href="/plan">{t('generatePlan')}</a></p>
			{:else}
				{#each data.today_table as course}
					<div class="course">
						<div class="course-head">
							<span class="chip">{t(course.key)}</span>
							{#if course.prepared}<span class="chip ok">{t('cooked')}</span>{/if}
							{#if course.suggested}<span class="chip warn">{t('suggested')}</span>{/if}
						</div>
						{#if course.main}
							<div class="course-line">
								<span class="course-role">{roleIcon(course.main.role, course.main.name_ar)}</span>
								{#if course.main.recipe_id}
									<a href={`/recipes?open=${course.main.recipe_id}`}>{course.main.name_ar}</a>
								{:else}
									<span>{course.main.name_ar}</span>
								{/if}
								{#if course.main.role !== 'dish'}<span class="muted">· {t(course.main.role)}</span>{/if}
							</div>
						{/if}
						{#each course.sides as side}
							<div class="course-line side">
								<span class="course-role">{roleIcon(side.role, side.name_ar)}</span>
								{#if side.recipe_id}
									<a href={`/recipes?open=${side.recipe_id}`}>{side.name_ar}</a>
								{:else}
									<span>{side.name_ar}</span>
								{/if}
								<span class="muted">· {t(side.role)}</span>
							</div>
						{/each}
					</div>
				{/each}
				<p class="muted" style="margin:.15rem 0 .5rem">{t('sideSuggestHint')}</p>
				<div class="row" style="margin-top:.7rem">
					<span class="muted">{t('tapDishHint')}</span>
					<a href="/recipes" class="btn small">{t('recipes')}</a>
				</div>
			{/if}
		</div>

		{#if data.quick_basics?.length}
			<h2>{t('quickBasics')}</h2>
			<p class="muted" style="margin-top:-.3rem">{t('basicsHint')}</p>
			<div class="card">
				<div class="basics-grid">
					{#each data.quick_basics as b}
						<a class="basic-item" href={`/recipes?open=${b.recipe_id}`}>
							<span class="basic-icon">{roleIcon(b.role, b.name_ar)}</span>
							<span class="basic-name">{b.name_ar}</span>
							<span class="muted">{t(b.role)}</span>
						</a>
					{/each}
				</div>
				<a href="/recipes" class="btn secondary small" style="margin-top:.7rem">{t('allRecipes')}</a>
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
				<a href="/shopping" class="btn small">{t('shopping')}</a>
			</div>
		{/if}

		{#if data.expiring_stock.length > 0}
			<h2>{t('expiring')}</h2>
			<div class="card">
				{#each data.expiring_stock as lot}
					<div class="row" style="padding:.3rem 0">
						<span>{lot.product_name_ar} — {formatQty(lot.quantity, lot.unit)}</span>
						<span class="chip warn">{lot.expiry_date}</span>
					</div>
				{/each}
				<a href="/inventory">{t('inventory')} ←</a>
			</div>
		{/if}

		{#if data.recent_prices.length > 0}
			<h2>{t('recentPrices')}</h2>
			<div class="card">
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
		{/if}
	{/if}
</div>

<style>
	.course {
		padding: 0.55rem 0;
		border-bottom: 1px dashed var(--border);
	}

	.course:last-of-type {
		border-bottom: none;
	}

	.course-head {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		flex-wrap: wrap;
		margin-bottom: 0.25rem;
	}

	.course-line {
		display: flex;
		align-items: baseline;
		gap: 0.4rem;
		padding: 0.12rem 0;
	}

	.course-line.side {
		font-size: 0.92rem;
	}

	.course-role {
		font-size: 1rem;
		line-height: 1.2;
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
		border: 1px solid var(--border);
		border-radius: 12px;
		background: var(--surface);
		color: var(--text);
		text-decoration: none;
	}

	.basic-item:hover {
		border-color: var(--primary);
	}

	.basic-icon {
		font-size: 1.3rem;
		line-height: 1.1;
	}

	.basic-name {
		font-weight: 600;
		font-size: 0.9rem;
	}
</style>
