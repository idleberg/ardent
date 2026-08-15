// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightThemeExquisitus from 'starlight-theme-exquisitus';
import sitemap from '@astrojs/sitemap'

// https://astro.build/config
export default defineConfig({
	site: 'https://idleberg.github.io',
	base: '/ardent',
	integrations: [
		sitemap(),
		starlight({
			title: 'Ardent',
			logo: {
				src: './public/favicon.svg',
			},
			plugins: [starlightThemeExquisitus()],
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/idleberg/ardent' }],
			sidebar: [
				{ label: 'Getting started', slug: 'getting-started' },
				{ label: 'CLI Usage', slug: 'cli-usage' },
				{ label: 'Integrations', slug: 'integrations' },
				{ label: 'Playground', slug: 'playground' },
			],
		}),
	],
});
