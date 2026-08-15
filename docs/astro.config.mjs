// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightThemeExquisitus from 'starlight-theme-exquisitus';

// https://astro.build/config
export default defineConfig({
	site: 'https://idleberg.github.io',
	base: '/ardent',
	integrations: [
		starlight({
			title: 'Ardent',
			plugins: [starlightThemeExquisitus()],
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/idleberg/ardent' }],
			sidebar: [
				{ label: 'Getting started', slug: 'getting-started' },
				{ label: 'Command Line', slug: 'cli-usage' },
				{ label: 'Integrations', slug: 'integrations' },
				{ label: 'Playground', slug: 'playground' },
			],
		}),
	],
});
