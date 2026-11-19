<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { page } from '$app/state';
	import { getLang, t, toggleLang, onLang, setLang } from '$lib/i18n.js';
	import { loadAuth, isAuthed, getAuth } from '$lib/api.js';
	import '../app.css';

	let { children } = $props();

	// Restore tokens before any child page mounts, otherwise a hard refresh
	// would let the dashboard redirect to /login despite a valid session.
	if (browser) loadAuth();

	let lang = $state(getLang());
	let authed = $state(isAuthed());

	// Navigation (login/logout/goto) re-evaluates auth without a full reload.
	$effect(() => {
		page.url.pathname;
		authed = isAuthed();
	});

	onMount(() => {
		setLang(getLang());
		onLang((l) => (lang = l));
	});

	// Pass `lang` so Svelte tracks the label as language-dependent; `t` itself
	// reads the module-level language, which the compiler cannot see as a dependency.
	function label(key, _lang) {
		return t(key);
	}

	const nav = [
		{ href: '/', icon: '🏠', key: 'home' },
		{ href: '/plan', icon: '📅', key: 'plan' },
		{ href: '/recipes', icon: '🍲', key: 'recipes' },
		{ href: '/shopping', icon: '🛒', key: 'shopping' },
		{ href: '/inventory', icon: '📦', key: 'inventory' },
		{ href: '/more', icon: '⋯', key: 'more' }
	];
</script>

<svelte:head>
	<title>بالهنا — Belhana</title>
</svelte:head>

{#key lang}
	{@render children()}
{/key}

{#if authed}
<nav class="bottomnav" dir={lang === 'ar' ? 'rtl' : 'ltr'}>
	{#each nav as n}
		<a href={n.href} class:active={page.url.pathname === n.href} onclick={() => (authed = isAuthed())}>
			<span class="icon">{n.icon}</span>{label(n.key, lang)}
		</a>
	{/each}
</nav>
{/if}
