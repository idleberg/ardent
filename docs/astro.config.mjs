// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightThemeExquisitus from 'starlight-theme-exquisitus';
import sitemap from '@astrojs/sitemap'
import indexnow from "astro-indexnow";

const site = 'https://idleberg.github.io';
const base = '/ardent';

// https://astro.build/config
export default defineConfig({
	site,
	base,
	integrations: [
		indexnow({
      key: 'b51721840dd84206b240f86949e12359',
    }),
		sitemap(),
		starlight({
			title: 'Ardent',
			logo: {
				src: './public/favicon.svg',
			},
			head: [
				{
					tag: 'meta',
					attrs: { property: 'og:image', content: new URL(`${base}/social.jpg`, site).href },
				},
				{
					tag: 'meta',
					attrs: { name: 'twitter:card', content: 'summary_large_image' },
				},
			],
			plugins: [starlightThemeExquisitus()],
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/idleberg/ardent' }],
			sidebar: [
				{ label: 'Getting started', slug: 'getting-started' },
				{ label: 'CLI Usage', slug: 'cli-usage' },
				{ label: 'Integrations', slug: 'integrations' },
				// { label: 'Formatting', slug: 'formatting' },
				{ label: 'Playground', slug: 'playground' },
			],
		}),
	],
});
