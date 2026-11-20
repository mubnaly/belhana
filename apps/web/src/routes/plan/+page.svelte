<script>
	import { onMount } from 'svelte';
	import { base } from '$app/paths';
	import { api, money, formatQty } from '$lib/api.js';
	import { t, getLang, langStore, toggleLang } from '$lib/i18n.js';
	import { kcalText } from '$lib/format.js';
	import MacroFacts from '$lib/MacroFacts.svelte';
	import Icon from '$lib/Icon.svelte';

	let plans = [];
	let current = null;
	let err = '';
	let loading = true;
	let busy = false;
	let selectedDiet = null;
	let lang = getLang();
	$: lang = $langStore;

	function localText(v) {
		return v ? (lang === 'ar' ? v.ar : v.en) : '';
	}
	$: planDietNote = current?.diet_note ? localText(current.diet_note) : '';

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
			const hh = await api.get('/api/household').catch(() => null);
			selectedDiet = hh?.selected_diet_id || null;
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
			window.location.href = base + '/shopping';
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	function periodName(p) {
		return t(p);
	}

	function dayLabel(date) {
		if (!date) return t('plan');
		const d = new Date(`${date}T00:00:00`);
		if (Number.isNaN(d.getTime())) return date;
		try {
			return new Intl.DateTimeFormat(lang === 'ar' ? 'ar-EG' : 'en-US', {
				weekday: 'long'
			}).format(d);
		} catch {
			return date;
		}
	}

	// Estimated calories for the whole planned slot (per serving × servings).
	function slotKcal(slot) {
		if (!slot.kcal_per_serving) return '';
		return slot.kcal_per_serving * (slot.servings || 1);
	}

	$: dayGroups = current
		? Object.values(
				current.slots.reduce((acc, slot) => {
					const key = slot.date || 'unknown';
					if (!acc[key]) acc[key] = { date: key, slots: [] };
					acc[key].slots.push(slot);
					return acc;
				}, {})
			)
				.map((group) => ({
					...group,
					slots: group.slots.sort(
						(a, b) => allPeriods.indexOf(a.period) - allPeriods.indexOf(b.period)
					),
					total: group.slots.reduce((sum, s) => sum + (s.cost_minor || 0), 0)
				}))
				.sort((a, b) => a.date.localeCompare(b.date))
		: [];
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="calendar" size={16} /> {t('plan')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if selectedDiet || current?.selected_diet_id}
		<div class="card" style="padding:.6rem; margin-bottom:.7rem; background:var(--surface-2)">
			<div class="row" style="justify-content:space-between; gap:.5rem; flex-wrap:wrap">
				<div>
					<span class="chip ok"><Icon name="diet" size={14} /> {t('selectedDiet')}</span>
					<p class="muted" style="margin:.35rem 0 0">{planDietNote || t('dietPlanNote')}</p>
				</div>
				<a class="btn secondary small" href="{base}/more/diets"><Icon name="pencil" size={14} /> {t('changeDiet')}</a>
			</div>
		</div>
	{/if}

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
			{#if busy}
				{t('loading')}
			{:else}
				<Icon name="sparkles" size={16} /> {t('generate')}
			{/if}
		</button>
	</div>

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if current}
		<div class="card row">
			<span>
				<b>{t('total')}: {money(current.total_cost_minor)}</b>
				{#if current.conflicts.length > 0}
					<div><span class="chip bad"><Icon name="warn" size={14} /> {current.conflicts.length}</span></div>
				{/if}
			</span>
			<button class="btn secondary small" onclick={makeShoppingList}><Icon name="cart" size={16} /> {t('shopping')}</button>
		</div>

		{#each dayGroups as group}
			<section class="card day-card">
				<div class="row day-head">
					<div>
						<h3 style="margin:0">{dayLabel(group.date)}</h3>
						<span class="muted">{group.date}</span>
					</div>
					<span class="chip"><Icon name="money" size={14} /> {money(group.total)}</span>
				</div>

				<div class="day-meals">
					{#each group.slots as slot}
						<article class="meal-note">
							<div class="row meal-head">
								<span class="chip">{periodName(slot.period)}</span>
								{#if slot.locked}<span class="chip warn">{t('locked')}</span>{/if}
								{#if slot.prepared}<span class="chip ok"><Icon name="check" size={13} /> {t('cooked')}</span>{/if}
							</div>

							<a class="meal-name" href={`${base}/recipes?open=${slot.recipe_id}&servings=${slot.servings}`}>
								{slot.recipe_name_ar}
							</a>

							<p class="muted meal-meta">{slot.servings} × {t('perServing')} · {money(slot.cost_minor)}</p>
							<p class="muted meal-meta">
								<span class="kcal estimate" title={t('kcalEstimate')}>{kcalText(slotKcal(slot), lang)}</span>
								{t('kcal')}
							</p>
							<MacroFacts macros={slot.macros_per_serving} {lang} />

							{#if slot.missing.length > 0}
								<p class="muted meal-meta"><Icon name="cart" size={14} /> {slot.missing.length} {lang === 'ar' ? 'عناصر للمشتريات' : 'items to buy'}</p>
							{/if}

							<div class="meal-actions">
								<a class="btn secondary small" href={`${base}/recipes?open=${slot.recipe_id}&servings=${slot.servings}`}>
									<Icon name="plate" size={14} /> {t('details')}
								</a>
								{#if !slot.prepared}
									<button class="btn small" onclick={() => markPrepared(slot)}><Icon name="check" size={15} /> {t('markPrepared')}</button>
								{/if}
							</div>

							<details style="margin-top:.4rem">
								<summary class="muted" style="cursor:pointer">{lang === 'ar' ? 'لماذا هذه الوجبة؟' : 'Why this meal?'}</summary>
								<p class="muted" style="margin:.3rem 0 0">{slot.explanation}</p>
								{#if slot.inventory_used.length > 0}
									<p class="muted" style="margin:.2rem 0 0">
										<Icon name="box" size={14} /> {slot.inventory_used.map((u) => `${u.name_ar} ${formatQty(u.quantity, u.unit)}`).join('، ')}
									</p>
								{/if}
							</details>
						</article>
					{/each}
				</div>
			</section>
		{/each}
	{:else}
		<p class="muted">{t('empty')}</p>
	{/if}

	{#if plans.length > 0}
		<div class="section">
			<h3>{lang === 'ar' ? 'الخطط السابقة' : 'Previous plans'}</h3>
			<div class="table-wrap">
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
		</div>
	{/if}
</div>

<style>
	.day-card {
		padding: 0.9rem;
	}

	.day-head {
		padding-bottom: 0.5rem;
		border-bottom: 2px dashed var(--border);
		margin-bottom: 0.7rem;
	}

	/* Meals for one day stay on the same row when there is room. On phones the
	   columns collapse to one card per row so nothing is squeezed or split. */
	.day-meals {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 210px), 1fr));
		gap: 0.7rem;
		align-items: stretch;
	}

	.meal-note {
		min-width: 0;
		padding: 0.7rem;
		background: var(--surface-2);
		border: 2px solid var(--border-strong);
		border-radius: 8px 14px 8px 13px;
		display: flex;
		flex-direction: column;
		gap: 0.15rem;
	}

	.meal-head {
		justify-content: flex-start;
		flex-wrap: wrap;
	}

	.meal-name {
		display: block;
		color: var(--text);
		font-weight: 700;
		line-height: 1.35;
		overflow-wrap: normal;
		word-break: normal;
		hyphens: none;
	}

	.meal-name:hover {
		color: var(--accent);
	}

	.meal-meta {
		margin: 0.05rem 0;
	}

	.meal-actions {
		display: flex;
		gap: 0.4rem;
		flex-wrap: wrap;
		margin-top: 0.5rem;
	}

	.meal-note details {
		margin-top: auto;
		padding-top: 0.4rem;
	}

	@media (max-width: 560px) {
		.day-meals {
			grid-template-columns: 1fr;
		}
	}
</style>
