// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

import { type Crosshair, type Dot } from './types';

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

export function drawDot(
  ctx: CanvasRenderingContext2D,
  dot: Dot,
  _debug: boolean,
) {
  const { radius } = dot;
  ctx.arc(radius, radius, radius, 0.0, 2.0 * Math.PI);
  ctx.fill();
}
