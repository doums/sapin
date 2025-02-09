<!-- This Source Code Form is subject to the terms of the Mozilla Public
   - License, v. 2.0. If a copy of the MPL was not distributed with this
   - file, You can obtain one at https://mozilla.org/MPL/2.0/. -->

<script lang="ts">
  import { onDestroy } from 'svelte';
  import { isCrosshair, isDot, type ShapeSize } from './types';
  import { drawCrosshair, drawDot, drawTwix } from './draw';
  import type { Shape } from './types';
  import { cfg } from './store';

  let shape: Shape;
  let color: string;
  let alpha: number;
  let debug: boolean;
  let canvas: HTMLCanvasElement | undefined = $state();
  let size: ShapeSize;

  const updateCanvas = () => {
    if (!canvas) {
      console.error('canvas not set');
      return;
    }
    canvas.width = size.width;
    canvas.height = size.height;
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      console.error('could not get 2d context');
      return;
    }
    ctx.clearRect(0.0, 0.0, size.width, size.height);
    ctx.fillStyle = color;
    ctx.globalAlpha = alpha;
    if (isCrosshair(shape)) {
      drawCrosshair(ctx, shape, debug);
    } else if (isDot(shape)) {
      drawDot(ctx, shape, debug);
    } else {
      drawTwix(ctx, shape, debug);
    }
  };

  const usubs = cfg.subscribe((c) => {
    console.log('___update overlay', c);
    if (!c) {
      return;
    }
    shape = c.shape;
    size = c.size;
    color = c.color;
    alpha = c.alpha;
    debug = c.debug;
    updateCanvas();
  });

  onDestroy(usubs);
</script>

<canvas
  bind:this={canvas}
  width={32}
  height={32}
></canvas>
