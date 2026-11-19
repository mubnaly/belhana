<script>
	import { onMount } from 'svelte';
	import { base } from '$app/paths';
	import { api } from '$lib/api.js';
	import { t, toggleLang, getLang, langStore } from '$lib/i18n.js';
	import { burnText, gramText, kcalText, num } from '$lib/format.js';
	import Icon from '$lib/Icon.svelte';

	let data = null;
	let plan = null;
	let err = '';
	let loading = true;
	let busy = false;
	let lang = getLang();
	$: lang = $langStore;

	let form = {
		sex: 'male',
		age: 30,
		height_cm: 175,
		weight_kg: 70,
		activity: 'moderate',
		goal: 'maintain',
		exercise_id: 'walking_brisk',
		minutes: 60
	};

	async function load() {
		loading = true;
		err = '';
		try {
			data = await api.get('/api/nutrition-guide');
			await calculate();
		} catch (e) {
			err = e.message;
		}
		loading = false;
	}

	async function calculate() {
		busy = true;
		err = '';
		try {
			plan = await api.post('/api/nutrition/athlete-plan', {
				sex: form.sex,
				age: Number(form.age),
				height_cm: Number(form.height_cm),
				weight_kg: Number(form.weight_kg),
				activity: form.activity,
				goal: form.goal,
				exercise_id: form.exercise_id,
				minutes: Number(form.minutes)
			});
		} catch (e) {
			err = e.message;
			plan = null;
		}
		busy = false;
	}

	onMount(load);

	function txt(v) {
		if (!v) return '';
		return lang === 'ar' ? v.ar : v.en;
	}

	function selectedExercise() {
		return data?.exercises?.find((e) => e.id === form.exercise_id);
	}
</script>

<div class="page">
	<header class="topbar" style="margin:-1rem -1rem 1rem; width:calc(100% + 2rem)">
		<div class="brand"><Icon name="activity" size={16} /> {t('athletes')}</div>
		<button class="btn small" onclick={toggleLang}>{lang === 'ar' ? 'EN' : 'عربي'}</button>
	</header>

	<div class="row" style="margin-bottom:.6rem">
		<a href="{base}/more" class="muted">← {t('more')}</a>
		<a href="{base}/more/diets" class="btn secondary small"><Icon name="diet" size={14} /> {t('diets')}</a>
	</div>

	{#if err}<div class="error-msg">{err}</div>{/if}
	<p class="muted">{t('athletesHint')}</p>
	<div class="notice"><Icon name="info" size={15} /> {t('burnCorrection')}</div>
	<p class="muted"><Icon name="warn" size={14} /> {t('guidanceNote')}</p>

	<div class="card note" style="padding:.9rem">
		<h3><Icon name="target" size={17} /> {t('athletes')}</h3>
		<div class="athlete-form">
			<div>
				<label for="ath-sex">{t('sex')}</label>
				<select id="ath-sex" bind:value={form.sex}>
					<option value="male">{t('male')}</option>
					<option value="female">{t('female')}</option>
				</select>
			</div>
			<div>
				<label for="ath-age">{t('age')}</label>
				<input id="ath-age" type="number" min="12" max="100" bind:value={form.age} />
			</div>
			<div>
				<label for="ath-height">{t('height')}</label>
				<input id="ath-height" type="number" min="100" max="250" bind:value={form.height_cm} />
			</div>
			<div>
				<label for="ath-weight">{t('weight')}</label>
				<input id="ath-weight" type="number" min="25" max="300" bind:value={form.weight_kg} />
			</div>
			<div>
				<label for="ath-activity">{t('activity')}</label>
				<select id="ath-activity" bind:value={form.activity}>
					<option value="sedentary">{t('actSedentary')}</option>
					<option value="light">{t('actLight')}</option>
					<option value="moderate">{t('actModerate')}</option>
					<option value="active">{t('actActive')}</option>
					<option value="very_active">{t('actVeryActive')}</option>
				</select>
			</div>
			<div>
				<label for="ath-goal">{t('goal')}</label>
				<select id="ath-goal" bind:value={form.goal}>
					<option value="maintain">{t('goalMaintain')}</option>
					<option value="muscle">{t('goalMuscle')}</option>
					<option value="endurance">{t('goalEndurance')}</option>
					<option value="cut">{t('goalCut')}</option>
				</select>
			</div>
			<div>
				<label for="ath-exercise">{t('exercise')}</label>
				<select id="ath-exercise" bind:value={form.exercise_id}>
					{#each data?.exercises ?? [] as ex}
						<option value={ex.id}>{txt(ex.name)} · {t('met')} {ex.met}</option>
					{/each}
				</select>
			</div>
			<div>
				<label for="ath-minutes">{t('duration')} ({t('minutes')})</label>
				<input id="ath-minutes" type="number" min="1" max="600" bind:value={form.minutes} />
			</div>
		</div>
		<button class="btn block" style="margin-top:.8rem" onclick={calculate} disabled={busy || loading}>
			<Icon name="flame" size={16} /> {busy ? t('loading') : t('calculate')}
		</button>
	</div>

	{#if plan}
		<h2><Icon name="chart" size={18} /> {t('result')}</h2>
		<div class="card note" style="padding:.9rem">
			<div class="grid2">
				<div class="stat"><div class="v">{kcalText(plan.bmr, lang)}</div><div class="k">{t('bmr')}</div></div>
				<div class="stat"><div class="v">{kcalText(plan.tdee, lang)}</div><div class="k">{t('tdee')}</div></div>
				<div class="stat"><div class="v">{kcalText(plan.target_kcal, lang)}</div><div class="k">{t('targetKcal')}</div></div>
				<div class="stat"><div class="v">{burnText(plan.burn_kcal, lang)}</div><div class="k">{t('burn')} · {t('met')} {plan.met}</div></div>
			</div>
			<p style="margin-top:.7rem">
				<span class="chip ok">{t('protein')} {gramText(plan.protein_g, lang)} · {num(plan.protein_per_kg, lang)} {t('perKg')}</span>
				<span class="chip">{t('carbs')} {gramText(plan.carbs_g, lang)}</span>
				<span class="chip">{t('fat')} {gramText(plan.fat_g, lang)}</span>
			</p>
			<p style="line-height:1.8">{txt(plan.summary)}</p>
			<p class="muted"><Icon name="info" size={14} /> {txt(plan.disclaimer)}</p>
		</div>
	{/if}

	{#if data}
		<h2 class="section"><Icon name="clock" size={18} /> {t('timing')}</h2>
		{#each data.meal_timing as w}
			<details class="card note" style="padding:.8rem; margin-bottom:.5rem">
				<summary style="cursor:pointer; font-weight:700">{txt(w.when)}</summary>
				<p style="line-height:1.8">{txt(w.body)}</p>
				<ul>
					{#each w.options as o}<li>{txt(o)}</li>{/each}
				</ul>
			</details>
		{/each}

		<details class="card" style="padding:.7rem; margin-top:.7rem">
			<summary style="cursor:pointer">{t('sources')}</summary>
			<ul>
				{#each data.sources as s}
					<li><a href={s.url} target="_blank" rel="noreferrer">{s.label}</a></li>
				{/each}
			</ul>
		</details>
	{/if}
</div>
