<script>
	import { onMount } from 'svelte';
	import { api } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import Icon from '$lib/Icon.svelte';

	let data = null;
	let err = '';
	let loading = true;
	let tab = 'micro';
	let lang = getLang();
	$: lang = $langStore;

	async function load() {
		loading = true;
		err = '';
		try {
			data = await api.get('/api/nutrition-guide');
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	function txt(v) {
		if (!v) return '';
		return lang === 'ar' ? v.ar : v.en;
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="diet" size={16} /> {t('diets')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	<div class="row" style="margin-bottom:.6rem">
		<a href="/more" class="muted">← {t('more')}</a>
		<span class="chip warn"><Icon name="info" size={13} /> {t('estimateNote')}</span>
	</div>

	{#if err}<div class="error-msg">{err}</div>{/if}
	<p class="muted">{t('dietsHint')}</p>
	<p class="muted"><Icon name="warn" size={14} /> {t('guidanceNote')}</p>

	<div class="card" style="padding:.6rem">
		<div style="display:flex; gap:.3rem; flex-wrap:wrap">
			<button class="chip {tab === 'micro' ? 'ok' : ''}" style="cursor:pointer" onclick={() => (tab = 'micro')}>{t('micronutrients')}</button>
			<button class="chip {tab === 'diets' ? 'ok' : ''}" style="cursor:pointer" onclick={() => (tab = 'diets')}>{t('dietPlans')}</button>
			<button class="chip {tab === 'sources' ? 'ok' : ''}" style="cursor:pointer" onclick={() => (tab = 'sources')}>{t('sources')}</button>
		</div>
	</div>

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else if data}
		{#if tab === 'micro'}
			<h2><Icon name="heart" size={18} /> {t('micronutrients')}</h2>
			{#each data.micronutrients as n}
				<details class="card note" style="padding:.8rem; margin-bottom:.5rem">
					<summary style="cursor:pointer; font-weight:700">
						{txt(n.name)} <span class="chip">{t('foods')}: {n.foods.length}</span>
					</summary>
					<p class="muted" style="line-height:1.8">{txt(n.summary)}</p>
					<h4>{t('foods')}</h4>
					<ul>
						{#each n.foods as f}
							<li><b>{txt(f.food)}</b> — {txt(f.reason)}</li>
						{/each}
					</ul>
					<p><Icon name="bulb" size={14} /> <b>{t('advice')}:</b> {txt(n.advice)}</p>
					<p class="muted"><Icon name="warn" size={14} /> <b>{t('caution')}:</b> {txt(n.caution)}</p>
				</details>
			{/each}
		{:else if tab === 'diets'}
			<h2><Icon name="diet" size={18} /> {t('dietPlans')}</h2>
			{#each data.diets as d}
				<details class="card note" style="padding:.8rem; margin-bottom:.5rem" open={d.id === 'mediterranean'}>
					<summary style="cursor:pointer; font-weight:700">{txt(d.name)}</summary>
					<p style="line-height:1.8">{txt(d.summary)}</p>
					<div class="diet-cols">
						<div>
							<h4><Icon name="check" size={14} /> {t('include')}</h4>
							<ul>
								{#each d.include as item}<li>{txt(item)}</li>{/each}
							</ul>
						</div>
						<div>
							<h4><Icon name="ban" size={14} /> {t('limit')}</h4>
							<ul>
								{#each d.limit as item}<li>{txt(item)}</li>{/each}
							</ul>
						</div>
					</div>
					<h4><Icon name="plate" size={14} /> {t('mealIdeas')}</h4>
					<ul>
						{#each d.meals as meal}<li>{txt(meal)}</li>{/each}
					</ul>
					<p class="muted"><Icon name="warn" size={14} /> <b>{t('caution')}:</b> {txt(d.caution)}</p>
				</details>
			{/each}
		{:else}
			<h2><Icon name="book" size={18} /> {t('sources')}</h2>
			{#each data.sources as s}
				<div class="card" style="padding:.65rem; margin-bottom:.4rem">
					<a href={s.url} target="_blank" rel="noreferrer">{s.label}</a>
				</div>
			{/each}
		{/if}

		{#each data.notes as note}
			<p class="muted" style="margin-top:.7rem"><Icon name="info" size={14} /> {txt(note)}</p>
		{/each}
	{/if}
</div>
