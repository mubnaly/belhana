<script>
	import { onMount } from 'svelte';
	import { api } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import Icon from '$lib/Icon.svelte';

	let members = [];
	let prefsByMember = {};
	let allergiesByMember = {};
	let exclusions = [];
	let err = '';
	let loading = true;
	let lang = getLang();
	$: lang = $langStore;
	let busy = false;
	let showAdd = false;

	let form = { display_name: '', age_band: 'adult' };
	let prefForm = { member_id: '', kind: 'like', text_value: '', strength: 'normal' };
	let allergyForm = { member_id: '', allergen: '', severity: 'high' };
	let exclForm = { product_name: '', reason: '' };

	async function load() {
		loading = true;
		try {
			const [m, e] = await Promise.all([
				api.get('/api/family/members'),
				api.get('/api/family/exclusions')
			]);
			members = m;
			exclusions = e;
			const prefMap = {};
			const allergyMap = {};
			for (const mem of members) {
				try {
					prefMap[mem.id] = await api.get(`/api/family/members/${mem.id}/preferences`);
					allergyMap[mem.id] = await api.get(`/api/family/members/${mem.id}/allergies`);
				} catch {
					prefMap[mem.id] = [];
					allergyMap[mem.id] = [];
				}
			}
			prefsByMember = prefMap;
			allergiesByMember = allergyMap;
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	onMount(load);

	async function addMember() {
		busy = true;
		err = '';
		try {
			await api.post('/api/family/members', { display_name: form.display_name, age_band: form.age_band });
			form = { display_name: '', age_band: 'adult' };
			showAdd = false;
			await load();
		} catch (e) {
			err = e.message;
		}
		busy = false;
	}

	async function removeMember(id) {
		if (!confirm(t('delete') + '?')) return;
		try {
			await api.del(`/api/family/members/${id}`);
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function addPref(memberId) {
		if (!prefForm.text_value) return;
		try {
			await api.post(`/api/family/members/${memberId}/preferences`, {
				kind: prefForm.kind,
				ref_type: 'text',
				text_value: prefForm.text_value,
				strength: prefForm.strength
			});
			prefForm.text_value = '';
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function addAllergy(memberId) {
		if (!allergyForm.allergen) return;
		try {
			await api.post(`/api/family/members/${memberId}/allergies`, {
				allergen: allergyForm.allergen,
				severity: allergyForm.severity
			});
			allergyForm.allergen = '';
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function addExclusion() {
		if (!exclForm.product_name) return;
		try {
			// resolve product by name (falls back to mapping page)
			const prods = await api.get(`/api/catalog/products?search=${encodeURIComponent(exclForm.product_name)}&limit=1`);
			const p = prods[0];
			if (!p) {
				err = lang === 'ar' ? 'المنتج غير موجود في الكتالوج' : 'Product not found in catalog';
				return;
			}
			await api.post('/api/family/exclusions', { product_id: p.id, reason: exclForm.reason || undefined });
			exclForm = { product_name: '', reason: '' };
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function removeExclusion(id) {
		try {
			await api.del(`/api/family/exclusions/${id}`);
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function removePref(memberId, prefId) {
		try {
			await api.del(`/api/family/members/${memberId}/preferences/${prefId}`);
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function removeAllergy(memberId, allergyId) {
		try {
			await api.del(`/api/family/members/${memberId}/allergies/${allergyId}`);
			await load();
		} catch (e) {
			err = e.message;
		}
	}

	async function memberDetails(id) {
		try {
			return {
				prefs: await api.get(`/api/family/members/${id}/preferences`),
				allergies: await api.get(`/api/family/members/${id}/allergies`)
			};
		} catch {
			return { prefs: [], allergies: [] };
		}
	}
	memberDetails; // (kept for future use)
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="family" size={16} /> {t('family')}</div>
		<div style="display:flex; gap:.5rem">
			<button class="btn small" onclick={() => (showAdd = !showAdd)}>+ {t('addMember')}</button>
			<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
		</div>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if showAdd}
		<div class="card">
			<label for="belhana-field-1">{t('name')}</label>
			<input bind:value={form.display_name} id="belhana-field-1"/>
			<label for="belhana-field-2">{t('ageBand')}</label>
			<select bind:value={form.age_band} id="belhana-field-2">
				<option value="child">child</option>
				<option value="teen">teen</option>
				<option value="adult">adult</option>
				<option value="senior">senior</option>
			</select>
			<button class="btn" style="margin-top:.8rem" onclick={addMember} disabled={busy}>{t('add')}</button>
		</div>
	{/if}

	{#if loading}
		<p class="muted">{t('loading')}</p>
	{:else}
		{#each members as m (m.id)}
			<div class="card">
				<div class="row">
					<span><b>{m.display_name}</b> <span class="muted">· {m.age_band}</span></span>
					<button class="btn secondary small" onclick={() => removeMember(m.id)} title={t('delete')} aria-label={t('delete')}><Icon name="trash" size={15} /></button>
				</div>

				<!-- preferences -->
				<h3 style="margin-top:.7rem"><Icon name="thumbsUp" size={17} /> {t('preferences')}</h3>
				{#each prefsByMember[m.id] ?? [] as p}
					<div class="row" style="padding:.2rem 0">
						<span class="chip"><Icon name={p.kind === 'like' ? 'thumbsUp' : 'thumbsDown'} size={14} /> {p.text_value}</span>
						<button class="btn secondary small" onclick={() => removePref(m.id, p.id)} title={t('delete')} aria-label={t('delete')}><Icon name="x" size={14} /></button>
					</div>
				{/each}
				<div class="field-row" style="margin-top:.3rem">
					<select bind:value={prefForm.kind} style="flex:0 0 auto; width:auto" aria-label={t('preferences')}>
						<option value="like">{t('like')}</option>
						<option value="dislike">{t('dislike')}</option>
					</select>
					<input placeholder={t('preferences') + '…'} bind:value={prefForm.text_value} />
					<button class="btn small" onclick={() => addPref(m.id)}>+</button>
				</div>

				<!-- allergies -->
				<h3 style="margin-top:.7rem"><Icon name="warn" size={17} /> {t('allergies')}</h3>
				{#each allergiesByMember[m.id] ?? [] as a}
					<div class="row" style="padding:.2rem 0">
						<span class="chip bad">{a.allergen} ({a.severity})</span>
						<button class="btn secondary small" onclick={() => removeAllergy(m.id, a.id)} title={t('delete')} aria-label={t('delete')}><Icon name="x" size={14} /></button>
					</div>
				{/each}
				<div class="field-row" style="margin-top:.3rem">
					<input placeholder={lang === 'ar' ? 'مثال: سمسم، فول سوداني' : 'e.g. sesame, peanut'} bind:value={allergyForm.allergen} />
					<button class="btn small" onclick={() => addAllergy(m.id)}>+</button>
				</div>
			</div>
		{/each}

		{#if members.length === 0}<p class="muted">{t('empty')}</p>{/if}

		<div class="section">
			<h3><Icon name="ban" size={17} /> {t('exclusions')}</h3>
			<div class="card">
				{#each exclusions as x}
					<div class="row" style="padding:.2rem 0">
						<span>{x.product_id} {x.reason ? '· ' + x.reason : ''}</span>
						<button class="btn secondary small" onclick={() => removeExclusion(x.id)} title={t('delete')} aria-label={t('delete')}><Icon name="x" size={14} /></button>
					</div>
				{/each}
				<div class="field-row" style="margin-top:.4rem">
					<input placeholder={t('product')} bind:value={exclForm.product_name} />
					<input placeholder={lang === 'ar' ? 'السبب' : 'reason'} bind:value={exclForm.reason} />
					<button class="btn small" onclick={addExclusion}>+</button>
				</div>
			</div>
		</div>
	{/if}
</div>
