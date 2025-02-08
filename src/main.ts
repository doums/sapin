// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

import './app.css';
import App from './App.svelte';
import { mount } from 'svelte';
import { init as logInit } from './log';

logInit();

const app = mount(App, {
  target: document.getElementById('app')!,
});

export default app;
