<script>
	import { toggleLang, getLang, t, langStore } from '$lib/i18n.js';

	let lang = getLang();
	$: lang = $langStore;

	const items = [
		{ href: '/more/budget', icon: '💰', key: 'budget' },
		{ href: '/more/prices', icon: '🏷️', key: 'prices' },
		{ href: '/more/family', icon: '👨‍👩‍👧‍👦', key: 'family' },
		{ href: '/more/rules', icon: '📏', key: 'rules' },
		{ href: '/more/occasions', icon: '🎉', key: 'occasions' },
		{ href: '/more/cooking', icon: '🫕', key: 'cooking' },
		{ href: '/more/tips', icon: '💡', key: 'tips' },
		{ href: '/more/assistant', icon: '🤖', key: 'assistant' },
		{ href: '/more/imports', icon: '📥', key: 'imports' },
		{ href: '/more/settings', icon: '⚙️', key: 'settings' }
	];
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">{t('more')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	<div class="grid2">
		{#each items as item}
			<a class="stat" href={item.href} style="text-align:center">
				<div style="font-size:1.6rem">{item.icon}</div>
				<div class="k" style="font-size:.85rem">{t(item.key)}</div>
			</a>
		{/each}
	</div>
</div>
