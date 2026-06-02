/*
 * Copyright (C) 2026 Leif Barton
 * Licensed under the Open Software License 3.0
 */

// @ts-check
import { defineConfig, fontProviders } from 'astro/config';

import svelte from '@astrojs/svelte';

// https://astro.build/config
export default defineConfig({
  integrations: [svelte()],
  vite: {
    envDir: "../"
  },
  fonts: [
    {
      provider: fontProviders.fontsource(),
      name: 'Source Serif 4',
      cssVariable: '--font-ss4'
    }
  ]
});