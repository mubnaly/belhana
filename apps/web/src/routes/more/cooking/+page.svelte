<script>
	import { onMount } from 'svelte';
	import { api, formatQty } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let guides = [];
	let err = '';
	let loading = true;
	let lang = getLang();
	$: lang = $langStore;

	async function load() {
		loading = true;
		try {
			guides = await api.get('/api/batch-guides');
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	async function convert(g) {
		const qty = parseFloat(prompt(lang === 'ar' ? 'الكمية الرئيسية للمخزن:' : 'Main quantity to stock:', '1'));
		if (!qty) return;
		try {
			// The guide's primary ingredient is used as the stock item.
			const ing = (g.ingredients?.[0]) || {};
			const prods = await api.get(`/api/catalog/products?search=${encodeURIComponent(ing.name_ar ?? '')}&limit=1`);
			const p = prods[0];
			if (!p) {
				err = lang === 'ar' ? 'المنتج غير موجود في الكتالوج' : 'Product not found';
				return;
			}
			await api.post(`/api/batch-guides/${g.id}/convert`, {
				product_id: p.id,
				quantity: qty,
				unit: ing.unit ?? 'kg'
			});
			alert(lang === 'ar' ? 'أُضيف للمخزن ✔' : 'Added to stock ✔');
		} catch (e) {
			err = e.message;
		}
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">🫕 {t('cooking')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	<p class="muted">{lang === 'ar' ? 'حضّر كمية كبيرة مرة واحدة، وزّعها على أسبوع. كل دليل يذكر التخزين والتجميد بأمان.' : 'Cook once in a big batch, eat all week. Every guide includes safe storage and freezing guidance.'}</p>

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else}
		{#each guides as g}
			<div class="card">
				<div class="row">
					<div>
						<b>{g.title_ar}</b>
						<span class="muted">· {g.batch_size_text ?? ''}{g.portions ? ' · ' + g.portions + (lang === 'ar' ? ' حصة' : ' portions') : ''}</span>
					</div>
					<button class="btn secondary small" onclick={() => convert(g)}>📦 {t('convertToInventory')}</button>
				</div>
				<details style="margin-top:.4rem">
					<summary class="muted" style="cursor:pointer">{lang === 'ar' ? 'التفاصيل' : 'Details'}</summary>
					<h3 style="margin-top:.5rem">{t('ingredients')}</h3>
					<ul>
						{#each g.ingredients ?? [] as ing}<li>{ing.name_ar} — {formatQty(ing.quantity, ing.unit)}</li>{/each}
					</ul>
					<h3>{t('steps')}</h3>
					<ol>
						{#each g.steps ?? [] as st}<li>{st.text_ar}</li>{/each}
					</ol>
					{#if g.storage}
						<h3>{lang === 'ar' ? 'التخزين والسلامة' : 'Storage & safety'}</h3>
						<ul>
							<li>{lang === 'ar' ? 'حفظ' : 'Container'}: {g.storage.container}</li>
							<li>{lang === 'ar' ? 'تبريد' : 'Refrigerate'}: {g.storage.refrigerate}</li>
							<li>{lang === 'ar' ? 'تجميد' : 'Freeze'}: {g.storage.freeze}</li>
							<li>{lang === 'ar' ? 'إذابة' : 'Defrost'}: {g.storage.defrost}</li>
							<li>{lang === 'ar' ? 'إعادة تسخين' : 'Reheat'}: {g.storage.reheat}</li>
						</ul>
						{#if g.safety_source}
							<p class="muted">📚 {g.safety_source} ({g.safety_source_date})</p>
						{/if}
					{/if}
				</details>
			</div>
		{/each}
	{/if}
</div>
