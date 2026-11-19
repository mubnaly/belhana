<script>
	import { onMount } from 'svelte';
	import { api, money, formatQty } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import { kcalText } from '$lib/format.js';
	import MacroFacts from '$lib/MacroFacts.svelte';
	import Icon from '$lib/Icon.svelte';

	let lots = [];
	let locations = [];
	let products = [];
	let expiring = [];
	let leftovers = [];
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
			const [l, loc, prods, exp, val, left] = await Promise.all([
				api.get('/api/inventory/lots'),
				api.get('/api/inventory/locations'),
				api.get('/api/catalog/products?limit=200'),
				api.get('/api/inventory/expiring?days=7'),
				api.get('/api/inventory/valuation'),
				api.get('/api/inventory/leftovers')
			]);
			lots = l;
			locations = loc;
			products = prods;
			expiring = exp;
			valuationMinor = val.valuation_minor;
			leftovers = left;
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

	async function toggleCarry(lot) {
		try {
			await api.post(`/api/inventory/lots/${lot.id}/carry-over`, { carry_over: !lot.carry_over });
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
		<div class="brand"><Icon name="box" size={16} /> {t('inventory')}</div>
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
					<label for="belhana-field-1">{t('quantity')}</label>
					<input type="number" step="0.01" min="0" bind:value={form.quantity} id="belhana-field-1"/>
				</div>
				<div>
					<label for="belhana-field-2">{t('unit')}</label>
					<input bind:value={form.unit} placeholder="kg / l / piece" id="belhana-field-2"/>
				</div>
			</div>
			<div class="field-row">
				<div>
					<label for="belhana-field-3">{t('locations')}</label>
					<select bind:value={form.location_id} id="belhana-field-3">
						<option value="">—</option>
						{#each locations as loc}<option value={loc.id}>{loc.name_ar}</option>{/each}
					</select>
				</div>
				<div>
					<label for="belhana-field-4">{t('expiryDate')}</label>
					<input type="date" bind:value={form.expiry_date} id="belhana-field-4"/>
				</div>
			</div>
			<label for="belhana-field-5">{t('amount')} ({lang === 'ar' ? 'قرش' : 'piasters'})</label>
			<input type="number" bind:value={form.estimated_cost_minor} id="belhana-field-5"/>
			<button class="btn" style="margin-top:.8rem" onclick={addLot} disabled={busy}>{t('add')}</button>
		</div>
	{/if}

	{#if expiring.length > 0}
		<div class="section">
			<h3><Icon name="warn" size={17} /> {t('expiring')}</h3>
			{#each expiring as lot}
				<div class="card row" style="padding:.7rem">
					<span>
						{lot.product_name_ar} · {formatQty(lot.quantity, lot.unit)}
						{#if lot.estimated_kcal}<br /><span class="kcal estimate">{kcalText(lot.estimated_kcal, lang)}</span>{/if}
					<MacroFacts macros={lot.estimated_macros} {lang} />
					</span>
					<span class="chip warn">{lot.expiry_date}</span>
				</div>
			{/each}
		</div>
	{/if}

	<div class="section">
		<h3><Icon name="calendar" size={17} /> {t('leftovers')}</h3>
		<p class="muted">{lang === 'ar' ? 'ما لا يُستخدم هذا الأسبوع يبقى متاحًا للأسبوع القادم، وتستخدمه الخطة قبل الشراء مجددًا.' : 'What you do not use this week stays available next week, and the planner uses it before buying more.'}</p>
		{#each leftovers as lot}
			<div class="card row" style="padding:.7rem">
				<span>
					<b>{lot.product_name_ar}</b> · {formatQty(lot.quantity, lot.unit)}
					<span class="chip ok"><Icon name="check" size={13} /> {t('useFirst')}</span>
					{#if lot.estimated_kcal}<br /><span class="kcal estimate">{kcalText(lot.estimated_kcal, lang)}</span>{/if}
					<MacroFacts macros={lot.estimated_macros} {lang} />
				</span>
				<button class="btn secondary small" onclick={() => toggleCarry(lot)}>{t('stopCarryOver')}</button>
			</div>
		{/each}
		{#if leftovers.length === 0}<p class="muted">{t('empty')}</p>{/if}
	</div>

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
					<span class="muted">· {formatQty(lot.quantity, lot.unit)}{lot.location_name ? ' · ' + lot.location_name : ''}</span>
					{#if lot.carry_over}<span class="chip ok">{t('carryOver')}</span>{/if}<br />
					{#if lot.expiry_date}<span class="muted">{t('expiryDate')}: {lot.expiry_date}</span>{/if}
					{#if lot.estimated_cost_minor}<span class="muted"> · {money(lot.estimated_cost_minor)}</span>{/if}
					{#if lot.estimated_kcal}<br /><span class="kcal estimate">{kcalText(lot.estimated_kcal, lang)}</span>{/if}
					<MacroFacts macros={lot.estimated_macros} {lang} />
				</span>
				<span style="display:flex; gap:.3rem">
					<button class="btn secondary small" onclick={() => consume(lot)}>{t('consume')}</button>
					<button class="btn secondary small" onclick={() => wasteLot(lot)}>{t('waste')}</button>
				</span>
			</div>
		</div>
	{/each}
</div>
