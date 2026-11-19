<script>
	import { onMount } from 'svelte';
	import { base } from '$app/paths';
	import { api, money } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import Icon from '$lib/Icon.svelte';

	let status = null;
	let history = [];
	let maxV = 1;
	let whatif = null;
	let err = '';
	let loading = true;
	let lang = getLang();
	$: lang = $langStore;
	let newBudget = 700;
	let busy = false;

	async function load() {
		loading = true;
		try {
			const [s, h] = await Promise.all([
				api.get('/api/budget/status?period=weekly').catch(() => null),
				api.get('/api/budget/history?days=30').catch(() => [])
			]);
			status = s;
			history = h;
			maxV = Math.max(...h.map((x) => x.amount_minor), 1);
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	async function runWhatIf() {
		err = '';
		busy = true;
		try {
			whatif = await api.post('/api/budget/what-if', { new_budget_minor: Math.round(newBudget * 100) });
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	function moneyf(m) {
		return money(m);
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="money" size={16} /> {t('budget')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if !status}
		<div class="card">
			<p>{lang === 'ar' ? 'لا توجد ميزانية أسبوعية مضبوطة بعد.' : 'No weekly budget configured yet.'}</p>
			<a href="{base}/more/settings" class="btn"><Icon name="arrowRight" size={16} /> {t('settings')}</a>
		</div>
	{:else}
		<div class="grid2">
			<div class="stat"><div class="v">{moneyf(status.budget_minor)}</div><div class="k">{t('weeklyBudget')}</div></div>
			<div class="stat"><div class="v">{moneyf(status.actual_spent_minor)}</div><div class="k">{t('spent')}</div></div>
			<div class="stat"><div class="v">{moneyf(status.planned_cost_minor)}</div><div class="k">{t('planned')}</div></div>
			<div class="stat"><div class="v">{moneyf(status.remaining_minor)}</div><div class="k">{t('remaining')}</div></div>
		</div>
		{#if status.budget_minor > 0}
			{@const pct = Math.min(100, Math.round(((status.actual_spent_minor + status.planned_cost_minor) / status.budget_minor) * 100))}
			<div class="progress" class:over={pct >= 100} style="margin-top:.8rem"><div style="width:{pct}%"></div></div>
		{/if}

		{#if status.actual_detail.length > 0}
			<div class="card section">
				<h3>{lang === 'ar' ? 'الصرف حسب التصنيف' : 'Spending by category'}</h3>
				<table class="table">
					<thead><tr><th>{t('category')}</th><th>{t('amount')}</th></tr></thead>
					<tbody>
						{#each status.actual_detail as d}
							<tr><td>{d.category}</td><td>{moneyf(d.amount_minor)}</td></tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}

		{#if history.length > 0}
			<div class="card">
				<h3>{lang === 'ar' ? 'آخر 30 يوم' : 'Last 30 days'}</h3>
				<div style="display:flex; align-items:flex-end; gap:2px; height:80px">
					{#each history as h}
						<div style="flex:1; background:var(--primary); border-radius:2px 2px 0 0; height:{Math.max(2, Math.round((h.amount_minor / maxV) * 100))}%"></div>
					{/each}
				</div>
				<p class="muted">{history[0].date} ← {history[history.length - 1].date}</p>
			</div>
		{/if}

		<div class="card">
			<h3><Icon name="chart" size={17} /> {t('whatIf')}</h3>
			<div class="field-row">
				<div>
					<label for="belhana-field-1">{t('newBudget')} ({lang === 'ar' ? 'جنيه' : 'EGP'})</label>
					<input type="number" bind:value={newBudget} min="0" id="belhana-field-1"/>
				</div>
				<div style="align-self:end">
					<button class="btn" onclick={runWhatIf} disabled={busy}>{t('generate')}</button>
				</div>
			</div>
			{#if whatif}
				<div class="notice">
					{lang === 'ar' ? 'تكلفة الخطة المتوقعة لـ' : 'Estimated plan cost for'} {whatif.days} {t('days')}:
					<b>{moneyf(whatif.estimated_total_minor)}</b> —
					{#if whatif.fits_budget}
						<Icon name="check" size={15} /> {lang === 'ar' ? 'داخل الميزانية' : 'fits the budget'}
					{:else}
						<Icon name="warn" size={15} /> {lang === 'ar' ? 'يتجاوز الميزانية' : 'exceeds the budget'}
					{/if}
				</div>
			{/if}
		</div>
	{/if}
</div>
