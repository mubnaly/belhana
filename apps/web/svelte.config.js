import adapter from '@sveltejs/adapter-static';

// GitHub Pages serves project sites from /<repo>/. Set BELHANA_BASE=/belhana
// in the Pages workflow; local dev and desktop builds keep the empty base.
const base = process.env.BELHANA_BASE || '';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		paths: { base },
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html',
			precompress: false,
			strict: false
		})
	}
};

export default config;
