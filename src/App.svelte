<!-- This Source Code Form is subject to the terms of the Mozilla Public
   - License, v. 2.0. If a copy of the MPL was not distributed with this
   - file, You can obtain one at https://mozilla.org/MPL/2.0/. -->

<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import Overlay from './Overlay.svelte';
  import { cfg } from './store';
  import type { AppConfig, PayloadEvent } from './types';

  const cfgEvent = 'config-reloaded';
  let unlisten: UnlistenFn;

  onMount(async () => {
    // init the config
    let config = await invoke<AppConfig>('config');
    cfg.set(config);

    unlisten = await listen<PayloadEvent>(cfgEvent, (event) => {
      console.log(`event ${cfgEvent}`, event.payload);
      cfg.set(event.payload.config);
    });
  });

  onDestroy(() => {
    unlisten();
  });
</script>

<main class="h-full select-none flex justify-center items-center">
  <Overlay />
</main>
