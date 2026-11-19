// Backend-free demo client used when the static site is hosted without an API
// (GitHub Pages, a plain nginx/Apache share, etc.). The snapshot in
// demo-data.json comes from a freshly seeded local API; see
// scripts/capture_demo_fixture.py to regenerate it after seed changes.

import fixture from './demo-data.json';

const STATE_KEY = 'belhana.demoState';
const clone = (value) => JSON.parse(JSON.stringify(value));
const uid = () =>
	globalThis.crypto?.randomUUID?.() ?? `demo-${Date.now()}-${Math.random().toString(16).slice(2)}`;

let state = null;

function current() {
	if (state) return state;
	try {
		const saved = globalThis.localStorage?.getItem(STATE_KEY);
		if (saved) {
			const parsed = JSON.parse(saved);
			if (parsed?.data && parsed?.recipes) {
				state = parsed;
				return state;
			}
		}
	} catch {
		/* storage may be unavailable or full */
	}
	state = clone(fixture);
	return state;
}

function persist() {
	try {
		globalThis.localStorage?.setItem(STATE_KEY, JSON.stringify(state));
	} catch {
		/* keep working in memory if the quota is exceeded */
	}
}

function productName(id) {
	const products = current().data['/api/catalog/products?limit=200'] ?? [];
	return products.find((p) => p.id === id)?.name_ar ?? '';
}

function scaleCost(cost, factor) {
	if (!cost) return cost;
	const out = clone(cost);
	out.servings = Math.round((cost.servings ?? 1) * factor);
	for (const key of ['known_total_minor', 'total_minor', 'per_serving_minor']) {
		if (out[key] != null) out[key] = Math.round(out[key] * factor);
	}
	for (const line of out.lines ?? []) {
		if (line.quantity != null) line.quantity = line.quantity * factor;
		if (line.price_minor != null) line.price_minor = Math.round(line.price_minor * factor);
	}
	return out;
}

function matchDynamic(st, path, query) {
	const seg = path.split('/').filter(Boolean);

	if (seg[1] === 'recipes' && seg[2] && seg[3] === 'cost') {
		const cost = st.cost[`${seg[2]}|${fixture.recipes[seg[2]]?.default_servings ?? 4}`];
		const servings = Number(query.get('servings')) || fixture.recipes[seg[2]]?.default_servings || 4;
		const base = fixture.recipes[seg[2]]?.default_servings || 4;
		return scaleCost(cost ?? Object.values(st.cost)[0], servings / base);
	}
	if (seg[1] === 'recipes' && seg[2]) return st.recipes[seg[2]] ?? null;

	if (seg[1] === 'meal-plans' && seg[2]) {
		const plans = st.data['/api/meal-plans'] ?? [];
		return plans.find((p) => p.id === seg[2]) ?? plans[0] ?? null;
	}

	if (seg[1] === 'imports' && seg[2] === 'batches' && seg[3]) {
		const prefix = `/api/imports/batches/${seg[3]}${seg[4] ? `/${seg[4]}` : ''}`;
		const hit = Object.keys(st.data).find((key) => key.startsWith(prefix));
		return hit ? st.data[hit] : null;
	}

	if (seg[1] === 'family' && seg[2] === 'members' && seg[4]) return [];

	if (seg[1] === 'prices' && seg[2]) {
		const all = st.data['/api/prices?limit=100'] ?? [];
		return all.find((p) => p.id === seg[2]) ?? null;
	}

	return undefined;
}

function doGet(path, query) {
	const st = current();
	if (Object.hasOwn(st.data, path)) return clone(st.data[path]);

	// Query variations (different limit/filter) reuse the widest captured list.
	const bare = path.split('?')[0];
	for (const key of Object.keys(st.data)) {
		if (key.split('?')[0] !== bare) continue;
		let value = st.data[key];
		const status = query.get('review_status');
		if (bare === '/api/prices' && status && Array.isArray(value)) {
			value = value.filter((p) => p.review_status === status);
		}
		const search = query.get('search');
		if (bare === '/api/catalog/products' && search && Array.isArray(value)) {
			const q = search.toLowerCase();
			value = value.filter(
				(p) =>
					p.name_ar?.toLowerCase().includes(q) ||
					p.name_en?.toLowerCase().includes(q) ||
					p.aliases?.some((a) => a.toLowerCase().includes(q))
			);
			value = value.slice(0, Number(query.get('limit')) || 200);
		}
		const category = query.get('category');
		if (bare === '/api/meal-tips' && category && Array.isArray(value)) {
			value = value.filter((tip) => tip.category === category);
		}
		return clone(value);
	}

	const dynamic = matchDynamic(st, bare, query);
	return dynamic === undefined ? [] : clone(dynamic);
}

function updatePrice(id, patch) {
	const st = current();
	for (const key of Object.keys(st.data)) {
		if (!key.startsWith('/api/prices?')) continue;
		st.data[key] = (st.data[key] ?? []).map((p) => (p.id === id ? { ...p, ...patch } : p));
	}
	persist();
}

function doMutation(method, path, body) {
	const st = current();
	const seg = path.split('/').filter(Boolean);
	const payload = body ?? {};

	if (path === '/api/auth/login' || path === '/api/auth/register') return clone(st.auth);
	if (path === '/api/auth/refresh') return { access_token: st.auth.access_token };
	if (path === '/api/auth/logout') return { ok: true };

	if (path === '/api/household' && method === 'PUT') {
		st.data['/api/household'] = { ...(st.data['/api/household'] ?? {}), ...payload };
		st.auth.household = { ...(st.auth.household ?? {}), ...payload };
		persist();
		return clone(st.data['/api/household']);
	}

	if (path === '/api/prices' && method === 'POST') {
		const record = {
			id: uid(),
			product_id: payload.product_id,
			product_name_ar: productName(payload.product_id),
			amount_minor: payload.amount_minor,
			unit: payload.unit ?? null,
			package_size: payload.package_size ?? null,
			package_unit: payload.package_unit ?? null,
			observation_date: payload.observation_date ?? null,
			merchant_id: payload.merchant_id ?? null,
			notes: payload.notes ?? null,
			source_type: payload.source_type ?? 'manual',
			review_status: 'confirmed',
			reviewed_at: new Date().toISOString(),
			previous_amount_minor: null,
			raw_product_label: productName(payload.product_id)
		};
		for (const key of Object.keys(st.data)) {
			if (key.startsWith('/api/prices?')) st.data[key] = [record, ...(st.data[key] ?? [])];
		}
		persist();
		return clone(record);
	}

	if (seg[1] === 'prices' && seg[2]) {
		if (method === 'PUT') {
			updatePrice(seg[2], { ...payload, review_status: 'confirmed' });
			return { ok: true };
		}
		if (seg[3] === 'review') {
			const action = payload.action;
			updatePrice(seg[2], {
				review_status: action === 'verify' ? 'verified' : action === 'reject' ? 'rejected' : 'confirmed',
				reviewed_at: new Date().toISOString()
			});
			return { ok: true };
		}
	}

	if (path === '/api/meal-plans' && method === 'POST') {
		const plans = st.data['/api/meal-plans'] ?? [];
		const id = plans[0]?.id;
		const detail = id ? doGet(`/api/meal-plans/${id}`, new URLSearchParams()) : null;
		return { plan: detail ?? { slots: [] }, plan_id: id ?? uid() };
	}
	if (seg[1] === 'planned-meals' && seg[3] === 'prepared') {
		for (const plan of Object.values(st.data)) {
			if (!Array.isArray(plan)) continue;
			for (const slot of plan.slots ?? []) if (slot.id === seg[2]) slot.prepared = true;
		}
		persist();
		return { ok: true };
	}

	if (path === '/api/shopping-lists' && method === 'POST') {
		return clone(st.data['/api/shopping-lists/active'] ?? { items: [] });
	}
	if (seg[1] === 'shopping-lists' && seg[3] === 'items' && seg[4]) {
		const list = st.data['/api/shopping-lists/active'];
		const item = list?.items?.find((i) => i.id === seg[4]);
		if (item) {
			item.purchased = Boolean(payload.purchased ?? !item.purchased);
			persist();
		}
		return clone(list ?? { items: [] });
	}

	if (path === '/api/recipes' && method === 'POST') {
		const recipe = { id: uid(), default_servings: 4, ...payload };
		st.data['/api/recipes?limit=200'] = [recipe, ...(st.data['/api/recipes?limit=200'] ?? [])];
		st.recipes[recipe.id] = recipe;
		persist();
		return clone(recipe);
	}
	if (seg[1] === 'recipes' && seg[2] && method === 'DELETE') {
		delete st.recipes[seg[2]];
		st.data['/api/recipes?limit=200'] = (st.data['/api/recipes?limit=200'] ?? []).filter(
			(r) => r.id !== seg[2]
		);
		persist();
		return { ok: true };
	}

	// Everything else in the demo is best-effort: acknowledge the change so the
	// UI stays usable, without pretending data was saved to a server.
	return { ok: true, id: uid(), ...clone(payload) };
}

export async function demoRequest(method, path, body) {
	const [bare, search = ''] = String(path).split('?');
	const query = new URLSearchParams(search);
	await Promise.resolve();
	return method.toUpperCase() === 'GET' ? doGet(String(path), query) : doMutation(method.toUpperCase(), bare, body);
}

export function resetDemo() {
	try {
		globalThis.localStorage?.removeItem(STATE_KEY);
	} catch {
		/* ignore */
	}
	state = null;
}
