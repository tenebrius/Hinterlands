// Median frame time of the web build in headless Chromium (SwiftShader, a
// software renderer: compare ratios, not absolute times).
// Usage: node tools/frametime.mjs <url> [frames]
// Waits 15 s for the page to settle, then times `frames` frames (default 8),
// dropping the first two, and prints the median in ms.
import { createRequire } from 'module';
const require = createRequire(import.meta.url);
let chromium;
try { ({ chromium } = require('playwright')); } catch { ({ chromium } = require('/opt/node-tools/node_modules/playwright')); }
const [url, n] = process.argv.slice(2);
const frames = Number(n || 8);
const browser = await chromium.launch({ args: ['--use-gl=swiftshader', '--enable-webgl', '--ignore-gpu-blocklist'] });
const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });
await page.goto(url);
await page.waitForTimeout(15000);
const ms = await page.evaluate((frames) => new Promise(res => {
  const t = []; let last = performance.now();
  const f = now => { t.push(now - last); last = now; if (t.length < frames) requestAnimationFrame(f); else res(t.slice(2).sort((a, b) => a - b)); };
  requestAnimationFrame(f);
}), frames);
console.log(`${Math.round(ms[Math.floor(ms.length / 2)])} ms median  ${url}`);
await browser.close();
