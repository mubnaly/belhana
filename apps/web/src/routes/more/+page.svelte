<script>
	import { toggleLang, getLang, t, langStore } from '$lib/i18n.js';
	import Icon from '$lib/Icon.svelte';

	let lang = getLang();
	$: lang = $langStore;

	const items = [
		{ href: '/more/budget', icon: 'money', key: 'budget' },
		{ href: '/more/prices', icon: 'tag', key: 'prices' },
		{ href: '/more/family', icon: 'family', key: 'family' },
		{ href: '/more/rules', icon: 'ruler', key: 'rules' },
		{ href: '/more/occasions', icon: 'sparkles', key: 'occasions' },
		{ href: '/more/cooking', icon: 'pot', key: 'cooking' },
		{ href: '/more/tips', icon: 'bulb', key: 'tips' },
		{ href: '/more/assistant', icon: 'robot', key: 'assistant' },
		{ href: '/more/imports', icon: 'inbox', key: 'imports' },
		{ href: '/more/settings', icon: 'settings', key: 'settings' }
	];
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="dots" size={16} /> {t('more')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	<div class="grid2">
		{#each items as item}
			<a class="stat" href={item.href} style="text-align:center">
				<div style="display:inline-flex; color:var(--primary-dark)"><Icon name={item.icon} size={26} /></div>
				<div class="k" style="font-size:.85rem">{t(item.key)}</div>
			</a>
		{/each}
	</div>
</div>
