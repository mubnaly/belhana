<script>
	import { onMount } from 'svelte';
	import { api, money } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let list = null;
	let err = '';
	let loading = true;
	let busy = false;
	let lang = getLang();
	$: lang = $langStore;
	let showQty = {};

	onMount(load);

	async function load() {
		loading = true;
		err = '';
		try {
			list = await api.get('/api/shopping-lists/active');
		} catch (e) {
			if (e.status !== 404) err = e.message;
			list = null;
		}
		loading = false;
	}

	function moneyf(minor) {
		return money(minor);
	}

	async function setPurchased(item, purchased) {
		try {
			list = await api.post(`/api/shopping-lists/${list.id}/items/${item.id}`, { purchased });
			if (purchased) showQty[item.id] = false;
		} catch (e) {
			err = e.message;
		}
	}

	async function updateItem(item) {
		try {
			list = await api.post(`/api/shopping-lists/${list.id}/items/${item.id}`, {
				purchased_qty: item.purchased_qty,
				actual_price_minor: item.actual_price_minor
			});
			showQty[item.id] = false;
		} catch (e) {
			err = e.message;
		}
	}

	async function completeTrip() {
		if (!confirm(t('completeTrip') + '?')) return;
		busy = true;
		try {
			list = await api.post(`/api/shopping-lists/${list.id}/complete`, {});
			if (list.status === 'completed') list = null;
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">🛒 {t('shopping')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if !list}
		<div class="card">
			<p>{t('empty')}</p>
			<p class="muted">{lang === 'ar' ? 'ولّد خطة أسبوعية أولًا، ثم أنشئ قائمة المشتريات منها.' : 'Generate a weekly plan first, then create the shopping list from it.'}</p>
			<a href="/plan" class="btn">{t('plan')} →</a>
		</div>
	{:else}
		<div class="card row">
			<span>
				<b>{list.items.filter((i) => i.purchased).length}/{list.items.length}</b>
				{t('purchased')}
			</span>
			<span><b>{moneyf(list.estimated_total_minor)}</b></span>
		</div>

		{#each list.items as item}
			<div class="card" style="padding:.7rem; {item.purchased ? 'opacity:.55' : ''}">
				<div class="row">
					<label style="display:flex; align-items:center; gap:.5rem; margin:0; flex:1">
						<input
							type="checkbox"
							style="width:auto"
							checked={item.purchased}
							onchange={(e) => setPurchased(item, e.currentTarget.checked)}
						/>
						<span>
							<b>{item.product_name_ar}</b><br />
							<span class="muted">
								{item.shopping_qty} {item.shopping_unit}
								{#if item.estimated_total_minor != null} · ~{moneyf(item.estimated_total_minor)}{/if}
								{#if item.price_freshness === 'stale'}<span class="chip warn">{t('stale')}</span>{/if}
							</span>
						</span>
					</label>
					<button class="btn secondary small" onclick={() => (showQty[item.id] = !showQty[item.id])}>✎</button>
				</div>

				{#if showQty[item.id]}
					<div class="field-row" style="margin-top:.5rem">
						<div>
							<label>{t('purchasedQty')}</label>
							<input type="number" step="0.01" bind:value={item.purchased_qty} placeholder={String(item.shopping_qty)} />
						</div>
						<div>
							<label>{t('actualPrice')} ({lang === 'ar' ? 'قرش' : 'piasters'})</label>
							<input type="number" bind:value={item.actual_price_minor} placeholder={String(item.estimated_total_minor ?? '')} />
						</div>
					</div>
					<button class="btn small" style="margin-top:.5rem" onclick={() => updateItem(item)}>{t('save')}</button>
				{/if}
			</div>
		{/each}

		<button class="btn block" onclick={completeTrip} disabled={busy}>🏁 {t('completeTrip')}</button>
	{/if}
</div>
