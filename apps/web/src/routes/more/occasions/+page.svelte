<script>
	import { onMount } from 'svelte';
	import { api, money } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let events = [];
	let err = '';
	let loading = true;
	let lang = getLang();
	$: lang = $langStore;
	let busy = false;
	let showAdd = false;

	let form = {
		name: '',
		occasion_type: 'family_visit',
		date: '',
		adults: 6,
		children: 2,
		meal_type: 'lunch'
	};
	let menusByEvent = {};

	async function load() {
		loading = true;
		try {
			events = await api.get('/api/events');
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	async function create() {
		busy = true;
		err = '';
		try {
			await api.post('/api/events', {
				...form,
				date: form.date || undefined
			});
			showAdd = false;
			form = { name: '', occasion_type: 'family_visit', date: '', adults: 6, children: 2, meal_type: 'lunch' };
			await load();
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	async function generateMenus(ev) {
		busy = true;
		err = '';
		try {
			const menus = await api.post(`/api/events/${ev.id}/menus`, {});
			menusByEvent[ev.id] = menus;
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	async function approve(ev, menu) {
		try {
			await api.post(`/api/events/${ev.id}/menus/approve/${menu.id}`, {});
			const menus = await api.get(`/api/events/${ev.id}/menus`);
			menusByEvent[ev.id] = menus;
		} catch (e) {
			err = e.message;
		}
	}

	async function eventShopping(ev) {
		try {
			await api.post(`/api/events/${ev.id}/shopping`, {});
			err = '';
			alert(lang === 'ar' ? 'تم إنشاء قائمة المشتريات للمناسبة' : 'Event shopping list created');
		} catch (e) {
			err = e.message;
		}
	}

	async function actualCost(ev) {
		const amount = parseFloat(prompt(lang === 'ar' ? 'المبلغ الفعلي (قرش)' : 'Actual amount (piasters)', '10000'));
		if (!amount || amount <= 0) return;
		try {
			await api.post(`/api/events/${ev.id}/actual-costs`, { category: 'groceries', amount_minor: Math.round(amount) });
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function pva(ev) {
		try {
			const v = await api.get(`/api/events/${ev.id}/planned-vs-actual`);
			alert(
				(lang === 'ar' ? 'المخطط: ' : 'Planned: ') +
					money(v.planned_minor) +
					'\n' +
					(lang === 'ar' ? 'الفعلي: ' : 'Actual: ') +
					money(v.actual_minor)
			);
		} catch (e) {
			err = e.message;
		}
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">🎉 {t('occasions')}</div>
		<div style="display:flex; gap:.5rem">
			<button class="btn small" onclick={() => (showAdd = !showAdd)}>+ {t('createEvent')}</button>
			<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
		</div>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if showAdd}
		<div class="card">
			<label>{t('eventName')}</label>
			<input bind:value={form.name} />
			<div class="field-row">
				<div><label>{t('date')}</label><input type="date" bind:value={form.date} /></div>
				<div>
					<label>{lang === 'ar' ? 'النوع' : 'Type'}</label>
					<select bind:value={form.occasion_type}>
						<option value="family_visit">{lang === 'ar' ? 'زيارة أهل' : 'Family visit'}</option>
						<option value="eid">{lang === 'ar' ? 'عيد' : 'Eid'}</option>
						<option value="invitation">{lang === 'ar' ? 'عزومة' : 'Invitation'}</option>
						<option value="birthday">{lang === 'ar' ? 'عيد ميلاد' : 'Birthday'}</option>
						<option value="other">{lang === 'ar' ? 'أخرى' : 'Other'}</option>
					</select>
				</div>
			</div>
			<div class="field-row">
				<div><label>{t('adults')}</label><input type="number" min="1" bind:value={form.adults} /></div>
				<div><label>{t('children')}</label><input type="number" min="0" bind:value={form.children} /></div>
			</div>
			<button class="btn block" style="margin-top:.8rem" onclick={create} disabled={busy}>{t('save')}</button>
		</div>
	{/if}

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else}
		{#each events as ev}
			<div class="card">
				<div class="row">
					<div>
						<b>{ev.name}</b> <span class="chip">{ev.status}</span><br />
						<span class="muted">👥 {ev.adults} {t('adults')} · {ev.children} {t('children')}{ev.date ? ' · ' + ev.date : ''}</span>
					</div>
					<div style="display:flex; gap:.3rem; flex-wrap:wrap">
						<button class="btn small" onclick={() => generateMenus(ev)} disabled={busy}>✨ {t('generateMenus')}</button>
						<button class="btn secondary small" onclick={() => eventShopping(ev)}>🛒</button>
						<button class="btn secondary small" onclick={() => actualCost(ev)}>💸</button>
						<button class="btn secondary small" onclick={() => pva(ev)}>📊</button>
					</div>
				</div>

				{#if menusByEvent[ev.id]}
					<div class="section">
						{#each menusByEvent[ev.id] as menu}
							<div class="row" style="padding:.4rem 0; border-top:1px solid var(--border)">
								<div>
									<span class="chip {menu.selected ? 'ok' : ''}">{t(menu.tier)}</span>
									<b>{menu.title}</b>
									<span class="muted">· {money(menu.total_cost_minor)}{menu.cost_per_guest_minor ? ' · ' + money(menu.cost_per_guest_minor) + ' ' + (lang === 'ar' ? '/ضيف' : '/guest') : ''}</span>
								</div>
								{#if !menu.selected}
									<button class="btn small" onclick={() => approve(ev, menu)}>{t('approve')}</button>
								{:else}
									<span class="chip ok">✔</span>
								{/if}
							</div>
							<details style="margin-bottom:.5rem">
								<summary class="muted" style="cursor:pointer">{menu.dishes.length} {lang === 'ar' ? 'أطباق' : 'dishes'}</summary>
								<ul>
									{#each menu.dishes as d}
										<li>{d.recipe_name_ar} — {d.course} · {d.servings}</li>
									{/each}
								</ul>
							</details>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
		{#if events.length === 0}<p class="muted">{t('empty')}</p>{/if}
	{/if}
</div>
