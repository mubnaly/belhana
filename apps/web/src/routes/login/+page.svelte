<script>
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { api, setAuth, isAuthed, isDemoMode } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import Icon from '$lib/Icon.svelte';

	let email = '';
	let password = '';
	let err = '';
	let busy = false;
	let lang = getLang();
	$: lang = $langStore;

	async function submit() {
		busy = true;
		err = '';
		try {
			const data = await api.post('/api/auth/login', { email, password });
			setAuth(data);
			goto(base + '/');
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}
</script>

<div class="page" style="max-width:420px; padding-top:8vh">
	<header style="text-align:center; margin-bottom:1.5rem">
		<img src="{base}/logo.svg" alt="" width="72" height="72" style="display:block; margin:0 auto .6rem" />
		<h1>{t('appName')}</h1>
		<p class="muted">{t('tagline')}</p>
	</header>

	{#if isDemoMode()}
		<div class="notice" style="margin-bottom:1rem">
			{lang === 'ar'
				? 'وضع تجريبي بدون سيرفر — أدخل أي بريد وكلمة مرور لاستكشاف التطبيق.'
				: 'Offline demo — enter any email and password to explore the app.'}
		</div>
	{/if}

	<div class="card">
		<form onsubmit={(e) => { e.preventDefault(); submit(); }}>
			<label for="email">{t('email')}</label>
			<input id="email" type="email" bind:value={email} required autocomplete="email" />

			<label for="password">{t('password')}</label>
			<input id="password" type="password" bind:value={password} required autocomplete="current-password" />

			{#if err}<div class="error-msg">{err}</div>{/if}

			<button class="btn block" type="submit" disabled={busy} style="margin-top:1rem">{busy ? t('loading') : t('login')}</button>
		</form>
	</div>

	<p style="text-align:center">
		<a href="{base}/register">{t('register')} <Icon name="arrowRight" size={14} /></a>
	</p>
	<p style="text-align:center">
		<button class="btn secondary small" onclick={toggleLang}>{lang === 'ar' ? 'English' : 'عربي'}</button>
	</p>
</div>
