<script>
	import { onMount } from 'svelte';
	import { api, money } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let obs = [];
	let err = '';
	let loading = true;
	let lang = getLang();
	$: lang = $langStore;
	let busy = false;
	let statusFilter = '';

	const statuses = ['unreviewed', 'needs_unit_confirmation', 'needs_date_confirmation', 'needs_product_mapping', 'verified', 'stale', 'rejected'];

	async function load() {
		loading = true;
		try {
			obs = await api.get(`/api/prices?limit=100${statusFilter ? `&review_status=${statusFilter}` : ''}`);
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	async function review(o, action, extra = {}) {
		busy = true;
		err = '';
		try {
			await api.post(`/api/prices/${o.id}/review`, { action, ...extra });
			await load();
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	function moneyf(m) {
		return m == null ? '—' : money(m);
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">🏷️ {t('prices')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	<div class="card" style="padding:.6rem">
		<div style="display:flex; gap:.3rem; flex-wrap:wrap">
			<button class="chip {statusFilter === '' ? 'ok' : ''}" style="cursor:pointer" onclick={() => { statusFilter = ''; load(); }}>{lang === 'ar' ? 'الكل' : 'All'}</button>
			{#each statuses as s}
				<button class="chip {statusFilter === s ? 'ok' : ''}" style="cursor:pointer" onclick={() => { statusFilter = s; load(); }}>{t(s)}</button>
			{/each}
		</div>
	</div>

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if obs.length === 0}
		<p class="muted">{t('empty')}</p>
	{:else}
		{#each obs as o}
			<div class="card" style="padding:.8rem">
				<div class="row">
					<div>
						<b>{o.raw_product_label ?? o.product_id ?? '—'}</b>
						<span class="muted">· {moneyf(o.amount_minor)} {o.unit ?? ''}</span><br />
						<span class="chip {o.review_status === 'verified' ? 'ok' : o.review_status === 'rejected' ? 'bad' : 'warn'}">{t(o.review_status)}</span>
						{#if o.observation_date}<span class="muted">· {o.observation_date}</span>{/if}
						{#if o.unit_status === 'unresolved'}<span class="chip bad">{lang === 'ar' ? 'وحدة غير محسومة' : 'unit unresolved'}</span>{/if}
						{#if o.notes}<span class="muted">· {o.notes}</span>{/if}
					</div>
				</div>
				{#if o.review_status !== 'verified' && o.review_status !== 'rejected'}
					<div style="display:flex; gap:.4rem; margin-top:.5rem; flex-wrap:wrap">
						<button class="btn small" onclick={() => review(o, 'verify')} disabled={busy}>✔ {t('verify')}</button>
						<button class="btn danger small" onclick={() => review(o, 'reject')} disabled={busy}>✕ {t('reject')}</button>
						{#if o.unit_status === 'unresolved'}
							<button class="btn secondary small" onclick={() => { const u = prompt('unit (kg/l/piece):', 'kg'); if (u) review(o, 'confirm_unit', { unit: u }); }}>📐 {t('confirmUnit')}</button>
						{/if}
						{#if !o.observation_date}
							<button class="btn secondary small" onclick={() => { const d = prompt('YYYY-MM-DD', new Date().toISOString().slice(0, 10)); if (d) review(o, 'confirm_date', { observation_date: d }); }}>📅 {t('confirmDate')}</button>
						{/if}
					</div>
				{/if}
			</div>
		{/each}
	{/if}
</div>
