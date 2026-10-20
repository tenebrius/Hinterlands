// Headless check of the web build: load the page, wait, optionally click Play,
// and save a screenshot. Usage: node tools/screenshot.mjs <url> <out.png> [playSeconds]
import { createRequire } from 'module';
const require = createRequire(import.meta.url);
let chromium;
try { ({ chromium } = require('playwright')); } catch { ({ chromium } = require(process.env.PLAYWRIGHT_MODULE || '/opt/node-tools/node_modules/playwright')); }

const [url, out, play = '0'] = process.argv.slice(2);
const browser = await chromium.launch({ args: ['--use-gl=swiftshader', '--enable-webgl', '--ignore-gpu-blocklist'] });
const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });
const errors = [];
page.on('console', m => { if (m.type() === 'error') errors.push(m.text()); if (m.text().includes('selftest')) console.log(m.text()); });
page.on('pageerror', e => errors.push(String(e)));
await page.goto(url);
await page.waitForTimeout(4000);
if (Number(play) > 0) {
  // The Play button sits near the top of the left panel.
  await page.keyboard.press('Space');
  await page.waitForTimeout(Number(play) * 1000);
}
await page.screenshot({ path: out });
const loading = await page.$eval('#loading', e => e.textContent).catch(() => null);
console.log(JSON.stringify({ loadingDivStillPresent: loading, errors }, null, 1));
await browser.close();
