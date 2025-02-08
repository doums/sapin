// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

export type Crosshair = {
  size: number;
  thickness: number;
  gap: number | null;
};

export type Dot = {
  radius: number;
};

export type Twix = {
  height: number;
  thickness: number;
  gap: number;
};

export type Shape = Crosshair | Dot | Twix;
export type ShapeSize = {
  width: number;
  height: number;
};

export type Position = 'center' | { x: number; y: number };

export type AppConfig = {
  shape: Shape;
  size: ShapeSize;
  color: string;
  alpha: number;
  debug: boolean;
  position: Position;
};

export type PayloadEvent = {
  config: AppConfig;
};

export function isCrosshair(shape: Shape): shape is Crosshair {
  return 'size' in shape && 'thickness' in shape;
}

export function isDot(shape: Shape): shape is Dot {
  return 'radius' in shape;
}

export function isTwix(shape: Shape): shape is Twix {
  return 'height' in shape && 'thickness' in shape && 'gap' in shape;
}
