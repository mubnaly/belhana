<script>
	import { onMount } from 'svelte';
	import { api, money } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let lots = [];
	let locations = [];
	let products = [];
	let expiring = [];
	let valuationMinor = 0;
	let err = '';
	let loading = true;
	let busy = false;
	let lang = getLang();
	$: lang = $langStore;
	let filterLoc = '';
	let showAdd = false;

	let form = {
		product_id: '',
		quantity: 1,
		unit: 'kg',
		location_id: '',
		expiry_date: '',
		estimated_cost_minor: null
	};
	let productSearch = '';
	let wasteForm = { product_id: '', qty: 0, unit: 'kg', reason: '' };

	async function load() {
		loading = true;
		err = '';
		try {
			const [l, loc, prods, exp, val] = await Promise.all([
				api.get('/api/inventory/lots'),
				api.get('/api/inventory/locations'),
				api.get('/api/catalog/products?limit=200'),
				api.get('/api/inventory/expiring?days=7'),
				api.get('/api/inventory/valuation')
			]);
			lots = l;
			locations = loc;
			products = prods;
			expiring = exp;
			valuationMinor = val.valuation_minor;
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	async function addLot() {
		if (!form.product_id) return;
		busy = true;
		err = '';
		try {
			await api.post('/api/inventory/lots', {
				...form,
				location_id: form.location_id || null,
				expiry_date: form.expiry_date || null,
				estimated_cost_minor: form.estimated_cost_minor || null
			});
			showAdd = false;
			form = { product_id: '', quantity: 1, unit: 'kg', location_id: '', expiry_date: '', estimated_cost_minor: null };
			await load();
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	async function wasteLot(lot) {
		const qty = parseFloat(prompt(`${t('waste')} — ${t('quantity')} (${lot.unit}):`, '0'));
		if (!qty || qty <= 0) return;
		try {
			await api.post('/api/inventory/waste', { product_id: lot.product_id, qty, unit: lot.unit });
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function consume(lot) {
		const qty = parseFloat(prompt(`${t('consume')} — ${t('quantity')} (${lot.unit}):`, '0'));
		if (!qty || qty <= 0) return;
		try {
			await api.post('/api/inventory/consume', { product_id: lot.product_id, qty, unit: lot.unit });
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	$: filtered = filterLoc ? lots.filter((l) => l.location_id === filterLoc) : lots;
	$: searchResults = productSearch
		? products.filter((p) => (p.name_ar + (p.name_en ?? '')).toLowerCase().includes(productSearch.toLowerCase())).slice(0, 8)
		: [];
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">📦 {t('inventory')}</div>
		<div style="display:flex; gap:.5rem">
			<button class="btn small" onclick={() => (showAdd = !showAdd)}>+ {t('addLot')}</button>
			<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
		</div>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	<div class="grid2">
		<div class="stat"><div class="v">{money(valuationMinor)}</div><div class="k">{t('stockValue')}</div></div>
		<div class="stat"><div class="v">{expiring.length}</div><div class="k">{t('expiring')}</div></div>
	</div>

	{#if showAdd}
		<div class="card" style="margin-top:1rem">
			<h3>{t('addLot')}</h3>
			<input placeholder={t('search') + '…'} bind:value={productSearch} />
			{#if searchResults.length > 0}
				<div style="margin:.4rem 0">
					{#each searchResults as p}
						<button
							type="button"
							class="btn secondary small"
							style="margin:.15rem"
							onclick={() => { form.product_id = p.id; form.unit = p.default_unit ?? 'kg'; productSearch = p.name_ar; }}
						>{p.name_ar}</button>
					{/each}
				</div>
			{/if}
			<div class="field-row">
				<div>
					<label>{t('quantity')}</label>
					<input type="number" step="0.01" min="0" bind:value={form.quantity} />
				</div>
				<div>
					<label>{t('unit')}</label>
					<input bind:value={form.unit} placeholder="kg / l / piece" />
				</div>
			</div>
			<div class="field-row">
				<div>
					<label>{t('locations')}</label>
					<select bind:value={form.location_id}>
						<option value="">—</option>
						{#each locations as loc}<option value={loc.id}>{loc.name_ar}</option>{/each}
					</select>
				</div>
				<div>
					<label>{t('expiryDate')}</label>
					<input type="date" bind:value={form.expiry_date} />
				</div>
			</div>
			<label>{t('amount')} ({lang === 'ar' ? 'قرش' : 'piasters'})</label>
			<input type="number" bind:value={form.estimated_cost_minor} />
			<button class="btn" style="margin-top:.8rem" onclick={addLot} disabled={busy}>{t('add')}</button>
		</div>
	{/if}

	{#if expiring.length > 0}
		<div class="section">
			<h3>⚠ {t('expiring')}</h3>
			{#each expiring as lot}
				<div class="card row" style="padding:.7rem">
					<span>{lot.product_name_ar} · {lot.quantity} {lot.unit}</span>
					<span class="chip warn">{lot.expiry_date}</span>
				</div>
			{/each}
		</div>
	{/if}

	<div class="section">
		<h3>{t('locations')}</h3>
		<div style="display:flex; gap:.4rem; flex-wrap:wrap">
			<button class="chip {filterLoc === '' ? 'ok' : ''}" style="cursor:pointer" onclick={() => (filterLoc = '')}>{lang === 'ar' ? 'الكل' : 'All'}</button>
			{#each locations as loc}
				<button class="chip {filterLoc === loc.id ? 'ok' : ''}" style="cursor:pointer" onclick={() => (filterLoc = loc.id)}>{loc.name_ar}</button>
			{/each}
		</div>
	</div>

	{#each filtered as lot}
		<div class="card" style="padding:.7rem">
			<div class="row">
				<span>
					<b>{lot.product_name_ar}</b>
					<span class="muted">· {lot.quantity} {lot.unit}{lot.location_name ? ' · ' + lot.location_name : ''}</span><br />
					{#if lot.expiry_date}<span class="muted">{t('expiryDate')}: {lot.expiry_date}</span>{/if}
					{#if lot.estimated_cost_minor}<span class="muted"> · {money(lot.estimated_cost_minor)}</span>{/if}
				</span>
				<span style="display:flex; gap:.3rem">
					<button class="btn secondary small" onclick={() => consume(lot)}>{t('consume')}</button>
					<button class="btn secondary small" onclick={() => wasteLot(lot)}>{t('waste')}</button>
				</span>
			</div>
		</div>
	{/each}
</div>
