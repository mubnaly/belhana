<script>
	import { goto } from '$app/navigation';
	import { api, setAuth } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let email = '';
	let password = '';
	let displayName = '';
	let householdName = '';
	let err = '';
	let busy = false;
	let lang = getLang();
	$: lang = $langStore;

	async function submit() {
		busy = true;
		err = '';
		try {
			const data = await api.post('/api/auth/register', {
				email,
				password,
				display_name: displayName || undefined,
				household_name: householdName || undefined
			});
			setAuth(data);
			goto('/');
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}
</script>

<div class="page" style="max-width:420px; padding-top:6vh">
	<header style="text-align:center; margin-bottom:1.5rem">
		<div style="font-size:3rem">🌿</div>
		<h1>{t('register')}</h1>
	</header>

	<div class="card">
		<form onsubmit={(e) => { e.preventDefault(); submit(); }}>
			<label for="name">{t('name')}</label>
			<input id="name" bind:value={displayName} />

			<label for="hh">{t('householdName')}</label>
			<input id="hh" bind:value={householdName} placeholder={lang === 'ar' ? 'مثال: أسرة أحمد' : 'e.g. Ahmed family'} />

			<label for="email">{t('email')}</label>
			<input id="email" type="email" bind:value={email} required autocomplete="email" />

			<label for="password">{t('password')}</label>
			<input id="password" type="password" bind:value={password} required minlength="3" autocomplete="new-password" />

			{#if err}<div class="error-msg">{err}</div>{/if}

			<button class="btn block" type="submit" disabled={busy} style="margin-top:1rem">{busy ? t('loading') : t('register')}</button>
		</form>
	</div>

	<p style="text-align:center">
		<a href="/login">{t('login')} →</a>
	</p>
	<p style="text-align:center">
		<button class="btn secondary small" onclick={toggleLang}>{lang === 'ar' ? 'English' : 'عربي'}</button>
	</p>
</div>
