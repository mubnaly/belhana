// GitHub Pages serves 404.html for URLs that are not real files. This app is
// a client-routed SPA, so the same bootstrap document must answer both the
// root URL and any deep link (/recipes, /plan, /more/prices, ...).
import { copyFile, access } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const from = path.join(root, 'build', 'index.html');
const to = path.join(root, 'build', '404.html');

await access(from);
await copyFile(from, to);
console.log('Wrote build/404.html for GitHub Pages deep links');
