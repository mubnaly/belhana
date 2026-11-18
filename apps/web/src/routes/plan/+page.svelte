<script>
	import { onMount } from 'svelte';
	import { api, money, formatQty } from '$lib/api.js';
	import { t, getLang, langStore } from '$lib/i18n.js';

	let plans = [];
	let current = null;
	let err = '';
	let loading = true;
	let busy = false;
	let lang = getLang();
	$: lang = $langStore;

	// generator form
	let days = 7;
	let servings = 4;
	let startDate = new Date().toISOString().slice(0, 10);
	let breakfast = true;
	let lunch = true;
	let dinner = true;
	let lockedSlots = [];

	const allPeriods = ['breakfast', 'lunch', 'dinner'];

	async function reload() {
		loading = true;
		try {
			plans = await api.get('/api/meal-plans');
			if (plans.length > 0) {
				current = await api.get(`/api/meal-plans/${plans[0].id}`);
			}
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(reload);

	async function generate() {
		busy = true;
		err = '';
		const periods = [];
		if (breakfast) periods.push('breakfast');
		if (lunch) periods.push('lunch');
		if (dinner) periods.push('dinner');
		if (periods.length === 0) periods.push('lunch');
		try {
			const res = await api.post('/api/meal-plans', {
				days,
				servings_per_meal: servings,
				start_date: startDate,
				periods_enabled: periods,
				save: true
			});
			current = res.plan;
			plans = await api.get('/api/meal-plans');
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	async function markPrepared(slot) {
		try {
			await api.post(`/api/planned-meals/${slot.id}/prepared`, {});
			slot.prepared = true;
		} catch (e) {
			err = e.message;
		}
	}

	async function makeShoppingList() {
		if (!current || !plans.length) return;
		busy = true;
		try {
			await api.post('/api/shopping-lists', { plan_id: plans[0].id });
			window.location.href = '/shopping';
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	function periodName(p) {
		return t(p);
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">📅 {t('plan')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	<div class="card">
		<h3>{t('generatePlan')}</h3>
		<div class="field-row">
			<div>
				<label for="days">{t('days')}</label>
				<select id="days" bind:value={days}>
					{#each [1, 2, 3, 4, 5, 6, 7] as d}<option value={d}>{d}</option>{/each}
				</select>
			</div>
			<div>
				<label for="servings">{t('servings')}</label>
				<input id="servings" type="number" min="1" max="20" bind:value={servings} />
			</div>
			<div>
				<label for="start">{t('startDate')}</label>
				<input id="start" type="date" bind:value={startDate} />
			</div>
		</div>
		<div style="display:flex; gap:.8rem; margin-top:.7rem">
			<label style="display:flex; align-items:center; gap:.3rem; margin:0"><input type="checkbox" bind:checked={breakfast} style="width:auto" /> {t('breakfast')}</label>
			<label style="display:flex; align-items:center; gap:.3rem; margin:0"><input type="checkbox" bind:checked={lunch} style="width:auto" /> {t('lunch')}</label>
			<label style="display:flex; align-items:center; gap:.3rem; margin:0"><input type="checkbox" bind:checked={dinner} style="width:auto" /> {t('dinner')}</label>
		</div>
		<button class="btn" style="margin-top:1rem" onclick={generate} disabled={busy}>
			{busy ? t('loading') : '✨ ' + t('generate')}
		</button>
	</div>

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if current}
		<div class="card row">
			<span>
				<b>{t('total')}: {money(current.total_cost_minor)}</b>
				{#if current.conflicts.length > 0}
					<div><span class="chip bad">⚠ {current.conflicts.length}</span></div>
				{/if}
			</span>
			<button class="btn secondary small" onclick={makeShoppingList}>🛒 {t('shopping')}</button>
		</div>

		{#each current.slots as slot}
			<div class="card" style="padding:.8rem">
				<div class="row">
					<div>
						<span class="chip">{periodName(slot.period)}</span>
						<span class="muted">{slot.date}</span>
						<h3 style="margin:.3rem 0 0">{slot.recipe_name_ar}</h3>
						<p class="muted" style="margin:.2rem 0 0">{slot.servings} × {t('perServing')} · {money(slot.cost_minor)}</p>
						{#if slot.missing.length > 0}
							<p class="muted" style="margin:.2rem 0 0">🛒 {slot.missing.length} {lang === 'ar' ? 'عناصر للمشتريات' : 'items to buy'}</p>
						{/if}
					</div>
					<button class="btn small" onclick={() => markPrepared(slot)}>✔ {t('markPrepared')}</button>
				</div>
				<details style="margin-top:.4rem">
					<summary class="muted" style="cursor:pointer">{lang === 'ar' ? 'لماذا هذه الوجبة؟' : 'Why this meal?'}</summary>
					<p class="muted" style="margin:.3rem 0 0">{slot.explanation}</p>
					{#if slot.inventory_used.length > 0}
						<p class="muted" style="margin:.2rem 0 0">
							📦 {slot.inventory_used.map((u) => `${u.name_ar} ${formatQty(u.quantity, u.unit)}`).join('، ')}
						</p>
					{/if}
				</details>
			</div>
		{/each}
	{:else}
		<p class="muted">{t('empty')}</p>
	{/if}

	{#if plans.length > 0}
		<div class="section">
			<h3>{lang === 'ar' ? 'الخطط السابقة' : 'Previous plans'}</h3>
			<table class="table">
				<thead><tr><th>{t('startDate')}</th><th>{t('days')}</th><th>{t('total')}</th></tr></thead>
				<tbody>
					{#each plans.slice(0, 10) as p}
						<tr>
							<td>{p.start_date}</td>
							<td>{p.days}</td>
							<td>{money(p.estimated_total_cost_minor)}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
