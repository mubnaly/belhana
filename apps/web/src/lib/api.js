// Belhana API client. The base URL is the backend server; during `vite dev`
// requests go through the Vite proxy, in production to the configured origin.

// Inside the Tauri WebView there is no Vite proxy; talk to the local API
// server directly. The host page may override this with __BELHANA_API_BASE__.
const TAURI_BASE =
	typeof location !== 'undefined' && location.protocol === 'tauri:'
		? 'http://127.0.0.1:8080'
		: '';

const BASE =
	(typeof window !== 'undefined' && window.__BELHANA_API_BASE__) ||
	import.meta.env?.BELHANA_API_BASE ||
	TAURI_BASE ||
	'';

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
		res = await fetch(`${BASE}${path}`, {
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
		const res = await fetch(`${BASE}/api/auth/refresh`, {
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

export function todayStr() {
	const d = new Date();
	return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
}
