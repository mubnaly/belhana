<script>
	import { onMount } from 'svelte';
	import { api, money, formatQty } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import { kcalText, qty } from '$lib/format.js';
	import MacroFacts from '$lib/MacroFacts.svelte';
	import Icon from '$lib/Icon.svelte';

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
			const body = { actual_price_minor: item.actual_price_minor };
			if (!item.buy_by_amount_minor) body.purchased_qty = item.purchased_qty;
			list = await api.post(`/api/shopping-lists/${list.id}/items/${item.id}`, body);
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
		<div class="brand"><Icon name="cart" size={16} /> {t('shopping')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if !list}
		<div class="card">
			<p>{t('empty')}</p>
			<p class="muted">{lang === 'ar' ? 'ولّد خطة أسبوعية أولًا، ثم أنشئ قائمة المشتريات منها.' : 'Generate a weekly plan first, then create the shopping list from it.'}</p>
			<a href="/plan" class="btn"><Icon name="arrowRight" size={16} /> {t('plan')}</a>
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
								{#if item.buy_by_amount_minor}
									<span class="chip">{t('buyByAmount')}</span> ~{moneyf(item.buy_by_amount_minor)}
								{:else}
									{formatQty(item.shopping_qty, item.shopping_unit)}
								{/if}
								{#if item.estimated_total_minor != null} · ~{moneyf(item.estimated_total_minor)}{/if}
								{#if item.estimated_kcal}<br /><span class="kcal estimate">{kcalText(item.estimated_kcal, lang)}</span>{/if}
								<MacroFacts macros={item.estimated_macros} {lang} />
								{#if item.price_freshness === 'stale'}<span class="chip warn">{t('stale')}</span>{/if}
							</span>
						</span>
					</label>
					<button class="btn secondary small" onclick={() => (showQty[item.id] = !showQty[item.id])} aria-label={t('edit')}><Icon name="pencil" size={15} /></button>
				</div>

				{#if showQty[item.id]}
					<div class="field-row" style="margin-top:.5rem">
						{#if !item.buy_by_amount_minor}
							<div>
								<label for="belhana-field-1">{t('purchasedQty')}</label>
								<input type="number" step="0.01" bind:value={item.purchased_qty} placeholder={qty(item.shopping_qty)} id="belhana-field-1"/>
							</div>
						{/if}
						<div>
							<label for="belhana-field-2">{t('actualPrice')} ({lang === 'ar' ? 'قرش' : 'piasters'})</label>
							<input type="number" bind:value={item.actual_price_minor} placeholder={String(item.estimated_total_minor ?? '')} id="belhana-field-2"/>
						</div>
					</div>
					<button class="btn small" style="margin-top:.5rem" onclick={() => updateItem(item)}>{t('save')}</button>
				{/if}
			</div>
		{/each}

		<button class="btn block" onclick={completeTrip} disabled={busy}><Icon name="flag" size={17} /> {t('completeTrip')}</button>
	{/if}
</div>
