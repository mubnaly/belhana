<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { getLang, t, toggleLang, langStore } from '$lib/i18n.js';
	import { isAuthed, getAuth, api, logout } from '$lib/api.js';

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

		<h2>{t('todayMeals')}</h2>
		<div class="card">
			{#if data.today_meals.length === 0}
				<p class="muted">{t('empty')} — <a href="/plan">{t('generatePlan')}</a></p>
			{:else}
				{#each data.today_meals as m}
					<div class="row" style="padding:.35rem 0">
						<span><span class="chip">{t(m.period)}</span> {m.recipe_name_ar}</span>
						{#if m.prepared}<span class="chip ok">{t('cooked')}</span>{/if}
					</div>
				{/each}
			{/if}
		</div>

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
						<span>{lot.product_name_ar} — {lot.quantity} {lot.unit}</span>
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
								<td>{p.amount_minor != null ? money(p.amount_minor) : '—'} {p.unit ?? ''}</td>
								<td><span class="chip {p.review_status === 'verified' ? 'ok' : 'warn'}">{p.review_status}</span> {p.observation_date ?? ''}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	{/if}
</div>
