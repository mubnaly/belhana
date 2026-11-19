// Belhana API client. The base URL is the backend server; during `vite dev`
// requests go through the Vite proxy, in production to the configured origin.

// Inside the Tauri WebView there is no Vite proxy; talk to the API directly.
// Desktop uses 127.0.0.1; the Android emulator reaches the host on 10.0.2.2.
// A physical phone can override this from Settings (belhana.apiBase).
export function isTauri() {
	if (typeof window === 'undefined') return false;
	return Boolean(
		window.__TAURI_INTERNALS__ ||
			location.protocol === 'tauri:' ||
			location.hostname === 'tauri.localhost'
	);
}

function isAndroid() {
	return typeof navigator !== 'undefined' && /Android/i.test(navigator.userAgent);
}

const TAURI_BASE = isTauri()
	? isAndroid()
		? 'http://10.0.2.2:8080'
		: 'http://127.0.0.1:8080'
	: '';

function storedApiBase() {
	if (typeof window === 'undefined') return '';
	return window.localStorage.getItem('belhana.apiBase') || '';
}

export function getApiBase() {
	return (
		(typeof window !== 'undefined' && window.__BELHANA_API_BASE__) ||
		storedApiBase() ||
		import.meta.env?.BELHANA_API_BASE ||
		TAURI_BASE ||
		''
	);
}

export function setApiBase(url) {
	if (typeof window === 'undefined') return;
	const value = String(url || '').trim().replace(/\/$/, '');
	if (value) window.localStorage.setItem('belhana.apiBase', value);
	else window.localStorage.removeItem('belhana.apiBase');
}

// Resolved on every request so Settings can switch the server without a reload.

let accessToken = null;
let refreshToken = null;
let household = null;
let user = null;

const KEY = 'belhana.auth';

export function loadAuth() {
	if (typeof window === 'undefined') return;
	try {
		const raw = window.localStorage.getItem(KEY);
		if (raw) {
			const data = JSON.parse(raw);
			accessToken = data.accessToken;
			refreshToken = data.refreshToken;
			household = data.household;
			user = data.user;
		}
	} catch {
		/* ignore */
	}
}

function persist() {
	if (typeof window === 'undefined') return;
	if (accessToken) {
		window.localStorage.setItem(
			KEY,
			JSON.stringify({ accessToken, refreshToken, household, user })
		);
	} else {
		window.localStorage.removeItem(KEY);
	}
}

export function getAuth() {
	return { accessToken, refreshToken, household, user };
}

export function setHousehold(next) {
	household = next;
	persist();
}

export function isAuthed() {
	return Boolean(accessToken);
}

export function setAuth(data) {
	accessToken = data.access_token;
	refreshToken = data.refresh_token ?? refreshToken;
	household = data.household ?? household;
	user = data.user ?? user;
	persist();
}

export function logout() {
	accessToken = null;
	refreshToken = null;
	household = null;
	user = null;
	persist();
}

export class ApiError extends Error {
	constructor(status, message) {
		super(message);
		this.status = status;
	}
}

async function request(method, path, body, opts = {}) {
	const headers = { ...(opts.headers || {}) };
	if (body !== undefined) headers['Content-Type'] = 'application/json';
	if (accessToken) headers['Authorization'] = `Bearer ${accessToken}`;
	let res;
	try {
		res = await fetch(`${getApiBase()}${path}`, {
			method,
			headers,
			body: body === undefined ? undefined : JSON.stringify(body)
		});
	} catch (e) {
		throw new ApiError(0, 'network_error');
	}
	if (res.status === 401 && refreshToken && !opts.noRetry) {
		// try once to refresh the access token
		const refreshed = await refresh();
		if (refreshed) return request(method, path, body, { ...opts, noRetry: true });
		logout();
	}
	const text = await res.text();
	let data = null;
	try {
		data = text ? JSON.parse(text) : null;
	} catch {
		/* non-JSON */
	}
	if (!res.ok) {
		const msg = data?.error?.message || `HTTP ${res.status}`;
		throw new ApiError(res.status, msg);
	}
	return data;
}

export const api = {
	get: (path) => request('GET', path),
	post: (path, body = {}) => request('POST', path, body),
	put: (path, body = {}) => request('PUT', path, body),
	del: (path) => request('DELETE', path, {})
};

async function refresh() {
	if (!refreshToken) return false;
	try {
		const res = await fetch(`${getApiBase()}/api/auth/refresh`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ refresh_token: refreshToken })
		});
		if (!res.ok) return false;
		const data = await res.json();
		accessToken = data.access_token;
		refreshToken = data.refresh_token ?? refreshToken;
		persist();
		return true;
	} catch {
		return false;
	}
}

// ---------- formatting helpers ----------

export function money(minor) {
	if (minor == null) return '—';
	const egp = minor / 100;
	return `${egp.toLocaleString('ar-EG', { maximumFractionDigits: 2 })} ج.م`;
}

// Quantities come back from SQLite REAL arithmetic (e.g. 1.5999999999999999 kg).
// Trim floating-point noise for display without changing the stored value, and
// show weights below one kilogram in grams (0.5 kg → "500 g") because grams are
// easier to weigh and read for small amounts.
export function formatQty(value, unit) {
	if (value == null || value === '') return '';
	const n = Number(value);
	if (Number.isNaN(n)) return '';
	let amount = Math.round(n * 1000) / 1000;
	let u = unit ?? '';
	if (u === 'kg' && amount > 0 && amount < 1) {
		amount = Math.round(amount * 1000 * 1000) / 1000;
		u = 'g';
	}
	return `${amount} ${u}`.trim();
}

export function todayStr() {
	const d = new Date();
	return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
}
