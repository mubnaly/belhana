// Light/dark theme store. The initial value comes from the early script in
// app.html so the page never flashes the wrong palette; this module keeps the
// toggle in sync and persists the choice.

import { browser } from '$app/environment';
import { writable } from 'svelte/store';

const KEY = 'belhana.theme';

function initial() {
	if (!browser) return 'light';
	const saved = window.localStorage.getItem(KEY);
	if (saved === 'light' || saved === 'dark') return saved;
	return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

let theme = initial();

export const themeStore = writable(theme);

export function getTheme() {
	return theme;
}

export function setTheme(next) {
	if (next !== 'light' && next !== 'dark') return;
	theme = next;
	if (browser) {
		window.localStorage.setItem(KEY, next);
		document.documentElement.dataset.theme = next;
		const meta = document.querySelector('meta[name="theme-color"]');
		if (meta) meta.setAttribute('content', next === 'dark' ? '#12191b' : '#f3ead8');
	}
	themeStore.set(next);
}

export function toggleTheme() {
	setTheme(theme === 'dark' ? 'light' : 'dark');
}
