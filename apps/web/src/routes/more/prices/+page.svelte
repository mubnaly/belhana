<script>
	import { onMount } from 'svelte';
	import { api, money } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import Icon from '$lib/Icon.svelte';

	let obs = [];
	let products = [];
	let merchants = [];
	let err = '';
	let loading = true;
	let lang = getLang();
	$: lang = $langStore;
	let busy = false;
	let statusFilter = '';
	let showForm = false;
	let editing = null;

	let form = {
		product_id: '',
		amount: '',
		unit: 'kg',
		package_size: '',
		package_unit: '',
		observation_date: new Date().toISOString().slice(0, 10),
		merchant_id: '',
		notes: ''
	};

	const statuses = ['confirmed', 'unreviewed', 'needs_unit_confirmation', 'needs_date_confirmation', 'needs_product_mapping', 'verified', 'stale', 'rejected'];

	async function load() {
		loading = true;
		try {
			const [list, prods, merch] = await Promise.all([
				api.get(`/api/prices?limit=100${statusFilter ? `&review_status=${statusFilter}` : ''}`),
				api.get('/api/catalog/products?limit=200').catch(() => []),
				api.get('/api/merchants').catch(() => [])
			]);
			obs = list;
			products = prods;
			merchants = merch;
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	function blankForm() {
		return { product_id: '', amount: '', unit: 'kg', package_size: '', package_unit: '', observation_date: new Date().toISOString().slice(0, 10), merchant_id: '', notes: '' };
	}

	function openAdd() {
		editing = null;
		form = blankForm();
		showForm = true;
		err = '';
	}

	function openEdit(o) {
		editing = o.id;
		form = {
			product_id: o.product_id ?? '',
			amount: o.amount_minor != null ? String(o.amount_minor / 100) : '',
			unit: o.unit ?? '',
			package_size: o.package_size != null ? String(o.package_size) : '',
			package_unit: o.package_unit ?? '',
			observation_date: o.observation_date ?? '',
			merchant_id: o.merchant_id ?? '',
			notes: o.notes ?? ''
		};
		showForm = true;
		err = '';
	}

	async function savePrice() {
		if (!form.product_id) {
			err = t('product');
			return;
		}
		if (!(Number(form.amount) > 0)) {
			err = t('amount');
			return;
		}
		busy = true;
		err = '';
		const body = {
			product_id: form.product_id,
			amount_minor: Math.round(Number(form.amount) * 100),
			unit: form.unit || null,
			package_size: form.package_size === '' ? null : Number(form.package_size),
			package_unit: form.package_unit || null,
			observation_date: form.observation_date || null,
			merchant_id: form.merchant_id || null,
			notes: form.notes || null
		};
		try {
			if (editing) {
				await api.put(`/api/prices/${editing}`, body);
			} else {
				await api.post('/api/prices', { ...body, source_type: 'manual', unit_status: 'confirmed' });
			}
			showForm = false;
			editing = null;
			await load();
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

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

	function productName(o) {
		return products.find((p) => p.id === o.product_id)?.name_ar ?? o.raw_product_label ?? o.product_id ?? '—';
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="tag" size={16} /> {t('prices')}</div>
		<div style="display:flex; gap:.5rem">
			<button class="btn small" onclick={openAdd}>+ {t('addManualPrice')}</button>
			<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
		</div>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}
	<p class="muted" style="margin:.1rem 0 .6rem"><Icon name="info" size={14} /> {t('confirmedHint')}</p>

	{#if showForm}
		<div class="card">
			<h3>{editing ? t('editPrice') : t('addManualPrice')}</h3>
			<label for="belhana-field-1">{t('product')}</label>
			<select bind:value={form.product_id} disabled={!!editing} id="belhana-field-1">
				<option value="">—</option>
				{#each products as p}<option value={p.id}>{p.name_ar}</option>{/each}
			</select>
			<div class="field-row">
				<div>
					<label for="belhana-field-2">{t('amount')} ({lang === 'ar' ? 'جنيه' : 'EGP'})</label>
					<input type="number" min="0.01" step="0.01" bind:value={form.amount} placeholder="130" id="belhana-field-2"/>
				</div>
				<div>
					<label for="belhana-field-3">{t('unit')}</label>
					<input bind:value={form.unit} placeholder="kg / l / piece / carton" id="belhana-field-3"/>
				</div>
			</div>
			<div class="field-row">
				<div>
					<label for="belhana-field-4">{t('packageSize')} ({lang === 'ar' ? 'اختياري' : 'optional'})</label>
					<input type="number" min="0.01" step="0.01" bind:value={form.package_size} placeholder="30" id="belhana-field-4"/>
				</div>
				<div>
					<label for="belhana-field-5">{t('packageUnit')}</label>
					<input bind:value={form.package_unit} placeholder="piece" id="belhana-field-5"/>
				</div>
			</div>
			<div class="field-row">
				<div>
					<label for="belhana-field-6">{t('observationDate')}</label>
					<input type="date" bind:value={form.observation_date} id="belhana-field-6"/>
				</div>
				<div>
					<label for="belhana-field-7">{t('merchant')}</label>
					<select bind:value={form.merchant_id} id="belhana-field-7">
						<option value="">—</option>
						{#each merchants as m}<option value={m.id}>{m.name}</option>{/each}
					</select>
				</div>
			</div>
			<label for="belhana-field-8">{t('description')}</label>
			<input bind:value={form.notes} id="belhana-field-8"/>
			<div style="display:flex; gap:.5rem; margin-top:.8rem">
				<button class="btn" onclick={savePrice} disabled={busy}>{t('savePrice')}</button>
				<button class="btn secondary" onclick={() => (showForm = false)}>{t('cancel')}</button>
			</div>
			<p class="muted" style="margin-top:.6rem">{lang === 'ar' ? 'مثال: 30 بيضة بـ130 جنيهًا → احجم العبوة 30 ووحدة العبوة piece. سيحسب التطبيق سعر البيضة للمقارنة.' : 'Example: 30 eggs for 130 EGP → package size 30, package unit piece. The app computes the per-egg price for comparison.'}</p>
		</div>
	{/if}

	<div class="card" style="padding:.6rem; margin-top:1rem">
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
			<div class="card" style="padding:.8rem; margin-bottom:.5rem">
				<div class="row">
					<div>
						<b>{productName(o)}</b>
						<span class="muted">· {moneyf(o.amount_minor)} {o.unit ?? ''}</span><br />
						{#if o.package_size}<span class="muted">{t('packageSize')}: {o.package_size} {o.package_unit ?? ''} · </span>{/if}
						<span class="chip {o.review_status === 'verified' || o.review_status === 'confirmed' ? 'ok' : o.review_status === 'rejected' ? 'bad' : 'warn'}">{t(o.review_status)}</span>
						{#if o.observation_date}<span class="muted">· {o.observation_date}</span>{/if}
						{#if o.unit_status === 'unresolved'}<span class="chip bad">{lang === 'ar' ? 'وحدة غير محسومة' : 'unit unresolved'}</span>{/if}
						{#if o.notes}<span class="muted">· {o.notes}</span>{/if}
					</div>
					<button class="btn secondary small" onclick={() => openEdit(o)}><Icon name="pencil" size={15} /> {t('edit')}</button>
				</div>
				{#if o.review_status !== 'verified' && o.review_status !== 'confirmed' && o.review_status !== 'rejected'}
					<div style="display:flex; gap:.4rem; margin-top:.5rem; flex-wrap:wrap">
						<button class="btn small" onclick={() => review(o, 'verify')} disabled={busy}><Icon name="check" size={15} /> {t('verify')}</button>
						<button class="btn danger small" onclick={() => review(o, 'reject')} disabled={busy}><Icon name="x" size={15} /> {t('reject')}</button>
						{#if o.unit_status === 'unresolved'}
							<button class="btn secondary small" onclick={() => { const u = prompt('unit (kg/l/piece):', 'kg'); if (u) review(o, 'confirm_unit', { unit: u }); }}><Icon name="ruler" size={15} /> {t('confirmUnit')}</button>
						{/if}
						{#if !o.observation_date}
							<button class="btn secondary small" onclick={() => { const d = prompt('YYYY-MM-DD', new Date().toISOString().slice(0, 10)); if (d) review(o, 'confirm_date', { observation_date: d }); }}><Icon name="calendar" size={15} /> {t('confirmDate')}</button>
						{/if}
					</div>
				{/if}
			</div>
		{/each}
	{/if}
</div>
