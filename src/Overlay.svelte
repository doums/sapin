<!-- This Source Code Form is subject to the terms of the Mozilla Public
   - License, v. 2.0. If a copy of the MPL was not distributed with this
   - file, You can obtain one at https://mozilla.org/MPL/2.0/. -->

<script lang="ts">
  import { onDestroy } from 'svelte';
  import { blur } from 'svelte/transition';
  import { isCrosshair } from './types';
  import { drawCrosshair, drawDot } from './draw';
  import type { Shape } from './types';
  import { cfg } from './store';

  let shape: Shape;
  let color: string;
  let alpha: number;
  let debug: boolean;
  let canvas: HTMLCanvasElement;
  let size: number;

  const shapeSize = (shape: Shape) => {
    if (isCrosshair(shape)) {
      return shape.size;
    } else {
      return shape.radius * 2;
    }
  };

  const updateCanvas = () => {
    canvas.width = size;
    canvas.height = size;
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      console.error('could not get 2d context');
      return;
    }
    ctx.clearRect(0.0, 0.0, size, size);
    ctx.fillStyle = color;
    ctx.globalAlpha = alpha;
    if (isCrosshair(shape)) {
      drawCrosshair(ctx, shape, debug);
    } else {
      drawDot(ctx, shape, debug);
    }
  };

  const usubs = cfg.subscribe((c) => {
    console.log('___update overlay', c);
    if (!c) {
      return;
    }
    shape = c.shape;
    color = c.color;
    alpha = c.alpha;
    debug = c.debug;
    size = shapeSize(c.shape);
    updateCanvas();
  });

  onDestroy(usubs);
</script>

<canvas
  transition:blur={{ amount: 10 }}
  bind:this={canvas}
  width={32}
  height={32}
/>
