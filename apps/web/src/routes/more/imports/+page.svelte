<script>
	import { onMount } from 'svelte';
	import { api, money } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import Icon from '$lib/Icon.svelte';

	let lang = getLang();
	$: lang = $langStore;
	let err = '';
	let loading = true;

	// batch list
	let batches = [];

	// paste + parse
	let raw = '';
	let preview = null;
	let parseBusy = false;

	// selected batch
	let current = null;
	let sourcePost = null;
	let obs = [];
	let products = [];
	let obsFilter = '';

	async function load() {
		loading = true;
		try {
			batches = await api.get('/api/imports/batches');
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	async function runParse() {
		err = '';
		preview = null;
		if (!raw.trim()) return;
		parseBusy = true;
		try {
			preview = await api.post('/api/imports/parse', { raw_text: raw });
		} catch (e) {
			err = e.message;
		}
		parseBusy = false;
	}

	async function createBatch() {
		err = '';
		try {
			const res = await api.post('/api/imports/batches', { raw_text: raw });
			raw = '';
			preview = null;
			await load();
			await openBatch(res.batch.id);
		} catch (e) {
			err = e.message;
		}
	}

	async function openBatch(id) {
		err = '';
		try {
			const [b, o, p, sp] = await Promise.all([
				api.get('/api/imports/batches/' + id),
				api.get('/api/imports/batches/' + id + '/observations'),
				api.get('/api/catalog/products').catch(() => []),
				api.get('/api/imports/batches/' + id + '/source-post').catch(() => null)
			]);
			current = b;
			sourcePost = sp;
			obs = Array.isArray(o) ? o : [];
			products = Array.isArray(p) ? p : [];
			obsFilter = '';
		} catch (e) {
			err = e.message;
		}
	}

	function closeBatch() {
		current = null;
		obs = [];
	}

	async function review(o, action, extra = {}) {
		err = '';
		try {
			await api.post('/api/prices/' + o.id + '/review', { action, ...extra });
			await openBatch(current.id);
		} catch (e) {
			err = e.message;
		}
	}

	async function confirmDate(reject) {
		err = '';
		try {
			await api.post('/api/imports/batches/' + current.id + '/confirm-date', reject ? { reject_date: true } : {});
			await openBatch(current.id);
		} catch (e) {
			err = e.message;
		}
	}

	async function setStatus(status) {
		err = '';
		try {
			await api.post('/api/imports/batches/' + current.id + '/status', { status });
			current = await api.get('/api/imports/batches/' + current.id);
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	function filtered() {
		if (!obsFilter) return obs;
		return obs.filter((o) => o.review_status === obsFilter);
	}

	function flagName(f) {
		const map = {
			unit_not_stated: lang === 'ar' ? 'الوحدة غير مذكورة' : 'unit not stated',
			explicit_quantity_confirm: lang === 'ar' ? 'كمية صريحة — يؤكدها المستخدم' : 'explicit quantity — confirm',
			price_changed: lang === 'ar' ? 'سعر متغير' : 'price changed',
			previous_price: lang === 'ar' ? 'سعر سابق' : 'previous price'
		};
		return map[f] ?? f;
	}

	function statusName(s) {
		const map = {
			unreviewed: t('unreviewed'),
			needs_unit_confirmation: lang === 'ar' ? 'بحاجة لتأكيد الوحدة' : 'needs unit confirmation',
			needs_date_confirmation: lang === 'ar' ? 'بحاجة لتأكيد التاريخ' : 'needs date confirmation',
			needs_product_mapping: lang === 'ar' ? 'بحاجة لربط منتج' : 'needs product mapping',
			verified: t('verified'),
			rejected: t('rejected'),
			stale: t('stale'),
			estimate: lang === 'ar' ? 'تقديري' : 'estimate'
		};
		return map[s] ?? s;
	}

	function statusClass(s) {
		if (s === 'verified') return 'ok';
		if (s === 'rejected' || s === 'stale') return 'bad';
		return 'warn';
	}

	const unitChoices = ['kg', 'g', 'l', 'ml', 'piece', 'dozen', 'bundle', 'pack'];

	let showRaw = false;
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="inbox" size={16} /> {t('imports')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if current}
		<!-- ===== batch review view ===== -->
		<button class="btn small" onclick={closeBatch}><Icon name="arrowLeft" size={16} /> {lang === 'ar' ? 'رجوع' : 'Back'}</button>

		<div class="card" style="margin-top:.6rem">
			<h3>{lang === 'ar' ? 'دفعة أسعار' : 'Price batch'} <span class="chip {statusClass(current.batch_status)}">{current.batch_status}</span></h3>
			<p class="muted" style="font-size:.8rem">
				{lang === 'ar' ? 'أُضيفت' : 'Added'}: {current.created_at?.slice(0, 10) ?? ''} —
				{obs.length} {lang === 'ar' ? 'سعر' : 'prices'}
			</p>

			<div>
				<button class="chip" style="cursor:pointer" onclick={() => confirmDate(false)}>
					<Icon name="check" size={14} /> {lang === 'ar' ? 'تأكيد تاريخ البوست' : 'Confirm post date'}
				</button>
				<button class="chip bad" style="cursor:pointer" onclick={() => confirmDate(true)}>
					<Icon name="x" size={14} /> {lang === 'ar' ? 'رفض التاريخ' : 'Reject date'}
				</button>
			</div>

			<div style="margin-top:.6rem">
				<button class="chip ok" style="cursor:pointer" onclick={() => setStatus('approved')}>
					{t('approve')} ({lang === 'ar' ? 'إنهاء المراجعة' : 'close review'})
				</button>
				<button class="chip bad" style="cursor:pointer" onclick={() => setStatus('rejected')}>{t('reject')}</button>
				<button class="chip warn" style="cursor:pointer" onclick={() => setStatus('in_review')}>
					{lang === 'ar' ? 'قيد المراجعة' : 'In review'}
				</button>
			</div>

			<button class="btn small" style="margin-top:.6rem" onclick={() => (showRaw = !showRaw)}>
				{showRaw ? '▾' : '▸'} {lang === 'ar' ? 'نص البوست الأصلي' : 'Original post text'}
			</button>
			{#if showRaw}
				<pre style="white-space:pre-wrap; font-size:.8rem; background:var(--surface-2, #f6f4ef); padding:.6rem; border-radius:8px">{sourcePost?.raw_text ?? ''}</pre>
			{/if}
		</div>

		<div>
			{#each ['unreviewed', 'needs_unit_confirmation', 'needs_product_mapping', 'verified', 'rejected'] as s}
				<button class="chip {obsFilter === s ? 'ok' : ''}" style="cursor:pointer" onclick={() => (obsFilter = obsFilter === s ? '' : s)}>
					{statusName(s)}
				</button>
			{/each}
		</div>

		<div class="card" style="margin-top:.6rem">
			{#if filtered().length === 0}
				<p class="muted">{lang === 'ar' ? 'لا يوجد' : 'Nothing here'}</p>
			{:else}
				<table class="table">
					<thead>
						<tr><th>{lang === 'ar' ? 'المنتج' : 'Product'}</th><th>{t('price')}</th><th>{lang === 'ar' ? 'الحالة' : 'Status'}</th><th style="min-width:220px">{lang === 'ar' ? 'المراجعة' : 'Review'}</th></tr>
					</thead>
					<tbody>
						{#each filtered() as o}
							<tr>
								<td>
									{o.raw_product_label ?? o.raw_text ?? '—'}<br />
									{#if o.unit}<span class="muted">{o.unit}</span>{/if}
								</td>
								<td>{o.amount_minor == null ? '—' : money(o.amount_minor)}<br /><span class="muted" style="font-size:.75rem">{o.observation_date ?? ''}</span></td>
								<td><span class="chip {statusClass(o.review_status)}">{statusName(o.review_status)}</span></td>
								<td>
									<div class="row">
										<button class="chip ok" style="cursor:pointer" onclick={() => review(o, 'verify')}><Icon name="check" size={14} /> {t('verify')}</button>
										<button class="chip bad" style="cursor:pointer" onclick={() => review(o, 'reject')}><Icon name="x" size={14} /> {t('reject')}</button>
									</div>
									{#if o.review_status === 'needs_unit_confirmation'}
										<select
											class="chip"
											style="cursor:pointer; border:none; max-width:140px"
											onchange={(e) => e.target.value && review(o, 'confirm_unit', { unit: e.target.value })}
										>
											<option value="">{lang === 'ar' ? 'اختر الوحدة…' : 'pick unit…'}</option>
											{#each unitChoices as u}<option value={u}>{u}</option>{/each}
										</select>
									{/if}
									{#if o.review_status === 'needs_product_mapping'}
										<select
											class="chip"
											style="cursor:pointer; border:none; max-width:170px"
											onchange={(e) => e.target.value && review(o, 'map_product', { product_id: e.target.value })}
										>
											<option value="">{lang === 'ar' ? 'اربط بمنتج…' : 'map to product…'}</option>
											{#each products as p}
												<option value={p.id}>{lang === 'ar' ? p.name_ar : (p.name_en ?? p.name_ar)}</option>
											{/each}
										</select>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</div>
	{:else}
		<!-- ===== paste + list view ===== -->
		<div class="card">
			<h3>{lang === 'ar' ? 'لصق بوست أسعار' : 'Paste a price post'}</h3>
			<p class="muted" style="font-size:.8rem">
				{lang === 'ar'
					? 'انسخ نص بوست الأسعار (مثل بوست أولاد زين) هنا. كل سطر يُفصل سعره تلقائيًا. لا يُقبل أي سعر قبل مراجعتك.'
					: 'Paste community price post text (e.g. an Olad Zain post). Each line is parsed into a price observation. No price is trusted until you review it.'}
			</p>
			<textarea bind:value={raw} rows="8" style="width:100%" placeholder={lang === 'ar' ? 'أسعار اليوم…' : "Today's prices…"}></textarea>
			<div class="row" style="margin-top:.5rem">
				<button class="btn small" onclick={runParse} disabled={parseBusy || !raw.trim()}>
					{parseBusy ? t('loading') : (lang === 'ar' ? 'تحليل مبدئي' : 'Parse preview')}
				</button>
				<button class="btn small" onclick={createBatch} disabled={!raw.trim()}>
					{lang === 'ar' ? 'إضافة دفعة' : 'Add batch'}
				</button>
			</div>
		</div>

		{#if preview}
			<div class="card">
				<h3>{lang === 'ar' ? 'نتيجة التحليل' : 'Parse result'}</h3>
				<p style="font-size:.85rem">
					{lang === 'ar' ? 'التاريخ المقترح' : 'Proposed date'}:
					<b>{preview.proposed_date ?? (lang === 'ar' ? 'غير موجود' : 'not found')}</b>
					{#if preview.original_date_text}<span class="muted"> ({preview.original_date_text})</span>{/if}
				</p>
				{#if preview.weekday_mismatch}
					<div class="notice"><Icon name="warn" size={16} /> {lang === 'ar' ? 'يوم الأسبوع في النص لا يطابق التاريخ — يؤكدها المستخدم' : 'Weekday word does not match the date — confirm by hand'}</div>
				{/if}
				<table class="table">
					<thead><tr><th>{lang === 'ar' ? 'البند' : 'Item'}</th><th>{t('price')}</th><th>{lang === 'ar' ? 'ملاحظات' : 'Flags'}</th></tr></thead>
					<tbody>
						{#each preview.lines as l}
							<tr>
								<td>{l.label}</td>
								<td>{l.prices_minor.map((p) => money(p)).join(' · ')}</td>
								<td>
									{#each l.flags as f}<span class="chip warn">{flagName(f)}</span>{/each}
									{#if l.unit_word}<span class="muted">{l.unit_word}</span>{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}

		<div class="card" style="margin-top:.8rem">
			<h3>{lang === 'ar' ? 'الدفاعات المضافة' : 'Batches'}</h3>
			{#if loading}
				<p class="muted">{t('loading')}</p>
			{:else if batches.length === 0}
				<p class="muted">{lang === 'ar' ? 'لا توجد دفعات بعد' : 'No batches yet'}</p>
			{:else}
				<table class="table">
					<thead>
						<tr><th>{lang === 'ar' ? 'التاريخ' : 'Date'}</th><th>{lang === 'ar' ? 'الحالة' : 'Status'}</th><th></th></tr>
					</thead>
					<tbody>
						{#each batches as b}
							<tr>
								<td>{b.created_at?.slice(0, 10) ?? ''}</td>
								<td><span class="chip {statusClass(b.batch_status)}">{b.batch_status}</span></td>
								<td><button class="btn small" onclick={() => openBatch(b.id)}>{lang === 'ar' ? 'مراجعة' : 'Review'}</button></td>
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</div>
	{/if}
</div>
