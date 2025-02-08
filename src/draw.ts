// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

import { type Crosshair, type Dot, type Twix } from './types';

export function drawCrosshair(
  ctx: CanvasRenderingContext2D,
  crosshair: Crosshair,
  debug: boolean,
) {
  const { size, thickness, gap } = crosshair;
  const middle = (size - thickness) / 2;
  if (debug) {
    ctx.strokeRect(0.0, 0.0, size, size);
  }
  ctx.fillRect(0.0, middle, size, thickness);
  ctx.fillRect(middle, 0.0, thickness, size);
  if (gap) {
    console.log!('drawing gap');
    ctx.save();
    let drift = (size - gap) / 2.0;
    ctx.translate(drift, drift);
    ctx.clearRect(0.0, 0.0, gap, gap);
    if (debug) {
      ctx.strokeRect(0.0, 0.0, gap, gap);
    }
    ctx.restore();
  }
}

export function drawTwix(
  ctx: CanvasRenderingContext2D,
  twix: Twix,
  debug: boolean,
) {
  const { height, thickness, gap } = twix;
  const width = thickness * 2 + gap;
  if (debug) {
    ctx.strokeRect(0.0, 0.0, width, height);
  }
  ctx.fillRect(0.0, 0.0, width, height);
  if (gap > 0) {
    const middle = width / 2 - gap / 2;
    ctx.clearRect(middle, 0.0, gap, height);
  }
}

export function drawDot(
  ctx: CanvasRenderingContext2D,
  dot: Dot,
  _debug: boolean,
) {
  const { radius } = dot;
  ctx.arc(radius, radius, radius, 0.0, 2.0 * Math.PI);
  ctx.fill();
}
