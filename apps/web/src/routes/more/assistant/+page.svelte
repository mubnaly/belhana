<script>
	import { onMount } from 'svelte';
	import { api } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';

	let messages = [];
	let input = '';
	let busy = false;
	let err = '';
	let assistantEnabled = true;
	let proposals = [];
	let lang = getLang();
	$: lang = $langStore;

	onMount(async () => {
		try {
			const s = await api.get('/api/ai/status');
			assistantEnabled = s.assistant_enabled;
		} catch {
			assistantEnabled = false;
		}
		try {
			proposals = await api.get('/api/ai/proposals?status=pending');
		} catch {
			proposals = [];
		}
	});

	async function send() {
		const text = input.trim();
		if (!text || busy) return;
		input = '';
		messages = [...messages, { role: 'user', content: text }];
		busy = true;
		err = '';
		try {
			const res = await api.post('/api/ai/chat', {
				messages: messages.filter((m) => m.role !== 'error').map((m) => ({ role: m.role, content: m.content }))
			});
			messages = [...messages, { role: 'assistant', content: res.reply }];
			proposals = await api.get('/api/ai/proposals?status=pending').catch(() => proposals);
		} catch (e) {
			messages = [...messages, { role: 'error', content: e.message }];
		}
		busy = false;
	}

	async function resolve(p, decision) {
		try {
			await api.post(`/api/ai/proposals/${p.id}/resolve`, { decision });
			proposals = await api.get('/api/ai/proposals?status=pending');
		} catch (e) {
			err = e.message;
		}
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand">🤖 {t('assistant')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	{#if err}<div class="error-msg">{err}</div>{/if}

	{#if !assistantEnabled}
		<div class="notice">ℹ️ {t('assistantDisabled')}</div>
	{/if}

	<div class="card" style="min-height:300px; max-height:55vh; overflow-y:auto">
		{#if messages.length === 0}
			<p class="muted">{t('assistantHint')}</p>
			<div style="display:flex; gap:.4rem; flex-wrap:wrap; margin-top:.5rem">
				<button class="btn secondary small" onclick={() => (input = lang === 'ar' ? 'كم سعر كيلو الطماطم الآن؟' : 'What is the current tomato price?')}>{lang === 'ar' ? 'كم سعر الطماطم؟' : 'Tomato price?'}</button>
				<button class="btn secondary small" onclick={() => (input = lang === 'ar' ? 'اشرح لي خطة الأسبوع' : 'Explain this week plan')}>{lang === 'ar' ? 'اشرح الخطة' : 'Explain plan'}</button>
				<button class="btn secondary small" onclick={() => (input = lang === 'ar' ? 'ما وضع الميزانية؟' : 'Budget status?')}>{lang === 'ar' ? 'وضع الميزانية' : 'Budget?'}</button>
			</div>
		{/if}
		{#each messages as m}
			<div style="margin:.5rem 0; display:flex; flex-direction:column; align-items:{m.role === 'user' ? 'flex-end' : 'flex-start'}">
				<div style="background:{m.role === 'user' ? 'var(--primary-soft)' : m.role === 'error' ? '#fdecea' : 'var(--surface)'}; border:1px solid var(--border); border-radius:12px; padding:.55rem .8rem; max-width:90%; white-space:pre-wrap">
					{m.content}
				</div>
			</div>
		{/each}
		{#if busy}<p class="muted">⏳ {t('loading')}</p>{/if}
	</div>

	<div class="field-row" style="margin-top:.6rem">
		<input placeholder={lang === 'ar' ? 'اكتب سؤالك…' : 'Type a question…'} bind:value={input} onkeydown={(e) => e.key === 'Enter' && send()} />
		<button class="btn" onclick={send} disabled={busy} style="flex:0 0 auto">{t('send')}</button>
	</div>

	{#if proposals.length > 0}
		<div class="section">
			<h3>📝 {t('proposals')}</h3>
			{#each proposals as p}
				<div class="card row">
					<div>
						<span class="chip">{p.action_type}</span>
						{#if p.summary}<span class="muted">{p.summary}</span>{/if}
						<details><summary class="muted" style="cursor:pointer">{lang === 'ar' ? 'التفاصيل' : 'Details'}</summary><pre style="font-size:.75rem">{JSON.stringify(p.payload, null, 2)}</pre></details>
					</div>
					<div style="display:flex; gap:.3rem">
						<button class="btn small" onclick={() => resolve(p, 'approve')}>{t('approveProposal')}</button>
						<button class="btn danger small" onclick={() => resolve(p, 'reject')}>{t('rejectProposal')}</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
