<script>
	import { onMount } from 'svelte';
	import { api, getApiBase, setApiBase, isDemoMode } from '$lib/api.js';
	import { resetDemo } from '$lib/demo.js';
	import { t, toggleLang, getLang, setLang, langStore } from '$lib/i18n.js';
	import { themeStore, setTheme, getTheme } from '$lib/theme.js';
	import Icon from '$lib/Icon.svelte';

	let hh = null;
	let err = '';
	let saved = false;
	let apiSaved = false;
	let busy = false;
	let apiBase = '';
	let demoMode = false;
	let lang = getLang();
	$: lang = $langStore;
	let theme = getTheme();
	$: theme = $themeStore;

	// editable form
	let name = '';
	let city = '';
	let householdSize = 4;
	let weeklyBudget = 0; // EGP
	let monthlyBudget = 0; // EGP
	let pricePolicy = 'freshest_verified';

	async function load() {
		try {
			hh = await api.get('/api/household');
			name = hh.name ?? '';
			city = hh.city ?? '';
			householdSize = hh.household_size ?? 4;
			weeklyBudget = (hh.weekly_budget_minor ?? 0) / 100;
			monthlyBudget = (hh.monthly_budget_minor ?? 0) / 100;
			pricePolicy = hh.price_policy ?? 'freshest_verified';
		} catch (e) {
			err = e.message;
		}
	}

	onMount(() => {
		apiBase = getApiBase();
		demoMode = isDemoMode();
		load();
	});

	function switchDemo(next) {
		window.localStorage.setItem('belhana.demoMode', next ? '1' : '0');
		window.location.reload();
	}

	function resetDemoData() {
		resetDemo();
		window.location.reload();
	}

	async function applyApiBase() {
		setApiBase(apiBase);
		apiSaved = true;
		err = '';
		await load();
	}

	async function save() {
		err = '';
		saved = false;
		busy = true;
		try {
			hh = await api.put('/api/household', {
				name,
				city,
				household_size: householdSize,
				weekly_budget_minor: Math.round(weeklyBudget * 100),
				monthly_budget_minor: Math.round(monthlyBudget * 100),
				price_policy: pricePolicy
			});
			saved = true;
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="settings" size={16} /> {t('settings')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}
	{#if saved}<div class="notice"><Icon name="check" size={16} /> {lang === 'ar' ? 'تم الحفظ' : 'Saved'}</div>{/if}

	<div class="card">
		<h3><Icon name={theme === 'dark' ? 'moon' : 'sun'} size={16} /> {t('theme')}</h3>
		<p class="muted" style="font-size:.8rem">{t('themeHint')}</p>
		<div style="display:flex; gap:.4rem; flex-wrap:wrap">
			<button
				class="chip {theme === 'light' ? 'ok' : ''}"
				style="cursor:pointer"
				onclick={() => setTheme('light')}
				aria-label={t('themeLight')}
			>
				<Icon name="sun" size={14} /> {t('themeLight')}
			</button>
			<button
				class="chip {theme === 'dark' ? 'ok' : ''}"
				style="cursor:pointer"
				onclick={() => setTheme('dark')}
				aria-label={t('themeDark')}
			>
				<Icon name="moon" size={14} /> {t('themeDark')}
			</button>
		</div>
	</div>

	<div class="card">
		<h3><Icon name="globe" size={16} /> {t('apiBase')}</h3>
		<p class="muted" style="font-size:.8rem">{t('apiBaseHint')}</p>
		<div style="display:flex; gap:.5rem; align-items:end; flex-wrap:wrap">
			<div style="flex:1; min-width:180px">
				<label for="belhana-api-base" style="margin-top:.3rem">{t('apiBase')}</label>
				<input
					id="belhana-api-base"
					inputmode="url"
					bind:value={apiBase}
					oninput={() => (apiSaved = false)}
					placeholder={t('apiBasePlaceholder')}
					style="margin:0"/>
			</div>
			<button class="btn small" onclick={applyApiBase}>{t('apply')}</button>
		</div>
		{#if apiSaved}<p class="muted" style="margin:.3rem 0 0">{t('applied')}</p>{/if}
	</div>

	<div class="card">
		<h3>{lang === 'ar' ? 'الوضع التجريبي (بدون سيرفر)' : 'Offline demo (no backend)'}</h3>
		<p class="muted" style="font-size:.8rem">
			{lang === 'ar'
				? 'يعمل التطبيق ببيانات جاهزة محفوظة في المتصفح عندما لا يوجد سيرفر. عطّله بعد إدخال عنوان API أعلاه.'
				: 'The app runs from a bundled sample dataset stored in your browser when no API server exists. Turn it off after setting an API address above.'}
		</p>
		<div style="display:flex; gap:.5rem; flex-wrap:wrap; margin-top:.5rem">
			<button class="chip {demoMode ? 'ok' : ''}" style="cursor:pointer" onclick={() => switchDemo(true)}>
				{lang === 'ar' ? 'تشغيل الوضع التجريبي' : 'Use demo data'}
			</button>
			<button class="chip {!demoMode ? 'ok' : ''}" style="cursor:pointer" onclick={() => switchDemo(false)}>
				{lang === 'ar' ? 'استخدام سيرفر API' : 'Use API server'}
			</button>
			<button class="btn secondary small" onclick={resetDemoData}>
				{lang === 'ar' ? 'إعادة ضبط البيانات التجريبية' : 'Reset demo data'}
			</button>
		</div>
	</div>

	{#if !hh && !err}
		<p class="muted">{t('loading')}</p>
	{:else if hh}
		<div class="card">
			<h3>{lang === 'ar' ? 'بيانات المنزل' : 'Household'}</h3>
			<label for="belhana-field-1">{lang === 'ar' ? 'اسم المنزل' : 'Household name'}</label>
			<input bind:value={name} placeholder={lang === 'ar' ? 'منزل العائلة' : 'Family home'} id="belhana-field-1"/>
			<div class="field-row">
				<div>
					<label for="belhana-field-2">{t('city')}</label>
					<input bind:value={city} placeholder={lang === 'ar' ? 'القاهرة' : 'Cairo'} id="belhana-field-2"/>
				</div>
				<div>
					<label for="belhana-field-3">{lang === 'ar' ? 'عدد الأفراد' : 'Household size'}</label>
					<input type="number" bind:value={householdSize} min="1" max="30" id="belhana-field-3"/>
				</div>
			</div>
		</div>

		<div class="card">
			<h3><Icon name="money" size={16} /> {lang === 'ar' ? 'الميزانيات' : 'Budgets'}</h3>
			<div class="field-row">
				<div>
					<label for="belhana-field-4">{t('weeklyBudget')} ({lang === 'ar' ? 'جنيه' : 'EGP'})</label>
					<input type="number" bind:value={weeklyBudget} min="0" step="10" id="belhana-field-4"/>
				</div>
				<div>
					<label for="belhana-field-5">{lang === 'ar' ? 'الميزانية الشهرية' : 'Monthly budget'} ({lang === 'ar' ? 'جنيه' : 'EGP'})</label>
					<input type="number" bind:value={monthlyBudget} min="0" step="50" id="belhana-field-5"/>
				</div>
			</div>
		</div>

		<div class="card">
			<h3><Icon name="tag" size={16} /> {lang === 'ar' ? 'سياسة الأسعار' : 'Price policy'}</h3>
			<p class="muted" style="font-size:.8rem">
				{lang === 'ar'
					? 'أي سعر يستخدم في حساب التكلفة؟ الأسعار أولاً الأحدث الموثوقة، أو الأرخص الموثوقة.'
					: 'Which price is used for costing? the freshest verified price, or the cheapest verified one.'}
			</p>
			<div>
				<button
					class="chip {pricePolicy === 'freshest_verified' ? 'ok' : ''}"
					style="cursor:pointer"
					onclick={() => (pricePolicy = 'freshest_verified')}
				>
					{lang === 'ar' ? 'الأحدث الموثوق' : 'Freshest verified'}
				</button>
				<button
					class="chip {pricePolicy === 'cheapest_verified' ? 'ok' : ''}"
					style="cursor:pointer"
					onclick={() => (pricePolicy = 'cheapest_verified')}
				>
					{lang === 'ar' ? 'الأرخص الموثوق' : 'Cheapest verified'}
				</button>
			</div>
		</div>

		<div class="card">
			<h3><Icon name="globe" size={16} /> {lang === 'ar' ? 'اللغة' : 'Language'}</h3>
			<div>
				<button class="chip {lang === 'ar' ? 'ok' : ''}" style="cursor:pointer" onclick={() => setLang('ar')}>العربية</button>
				<button class="chip {lang === 'en' ? 'ok' : ''}" style="cursor:pointer" onclick={() => setLang('en')}>English</button>
			</div>
		</div>

		<button class="btn block" style="margin-top:1rem" onclick={save} disabled={busy}>
			{busy ? t('loading') : t('save')}
		</button>
	{/if}
</div>
