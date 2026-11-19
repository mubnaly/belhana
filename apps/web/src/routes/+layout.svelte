<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { getLang, t, onLang, setLang, langStore } from '$lib/i18n.js';
	import { loadAuth, isAuthed } from '$lib/api.js';
	import Icon from '$lib/Icon.svelte';
	import '../app.css';

	let { children } = $props();

	// Restore tokens before any child page mounts, otherwise a hard refresh
	// would let the dashboard redirect to /login despite a valid session.
	if (browser) loadAuth();

	let lang = $state(getLang());
	let authed = $state(isAuthed());
	let showSplash = $state(false);

	// Navigation (login/logout/goto) re-evaluates auth without a full reload.
	$effect(() => {
		page.url.pathname;
		authed = isAuthed();
	});

	onMount(() => {
		setLang(getLang());
		showSplash = true;
		const timer = setTimeout(() => (showSplash = false), 650);
		const offLang = onLang((l) => (lang = l));
		return () => {
			clearTimeout(timer);
			offLang();
		};
	});

	// Pass `lang` so Svelte tracks the label as language-dependent; `t` itself
	// reads the module-level language, which the compiler cannot see as a dependency.
	function label(key, _lang) {
		return t(key);
	}

	const nav = [
		{ href: '/', icon: 'home', key: 'home' },
		{ href: '/plan', icon: 'calendar', key: 'plan' },
		{ href: '/recipes', icon: 'pot', key: 'recipes' },
		{ href: '/shopping', icon: 'cart', key: 'shopping' },
		{ href: '/more/prices', icon: 'tag', key: 'prices' },
		{ href: '/more', icon: 'dots', key: 'more' }
	];
</script>

<svelte:head>
	<title>بالهنا — Belhana</title>
</svelte:head>

{#key lang}
	{@render children()}
{/key}

{#if showSplash}
	<div class="bootsplash" aria-hidden="true">
		<img src="{base}/logo.png" alt="" width="112" height="112" />
		<span>بالهنا</span>
	</div>
{/if}

{#if authed}
	<nav class="bottomnav" dir={lang === 'ar' ? 'rtl' : 'ltr'}>
		{#each nav as n}
			<a href={base + n.href} class:active={page.url.pathname === base + n.href} onclick={() => (authed = isAuthed())}>
				<span class="icon"><Icon name={n.icon} size={19} /></span>
				{label(n.key, lang)}
			</a>
		{/each}
	</nav>
{/if}

<style>
	.bootsplash {
		position: fixed;
		inset: 0;
		z-index: 999;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		background: var(--bg, #f3ead8);
		color: var(--primary-dark, #14543f);
		animation: splash-out 0.65s ease forwards;
	}
	.bootsplash img {
		width: 112px;
		height: 112px;
	}
	.bootsplash span {
		font-size: 1.4rem;
		font-weight: 700;
	}
	:global([data-theme='dark']) .bootsplash {
		background: #12191b;
	}
	@keyframes splash-out {
		0%, 70% { opacity: 1; }
		100% { opacity: 0; }
	}
</style>
