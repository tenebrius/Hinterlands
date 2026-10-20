// Headless check of the web build. Loads the page, performs actions, saves a
// screenshot and prints console errors and self-test lines.
// Usage: node tools/screenshot.mjs <url> <out.png> [action ...]
//   actions: click:X,Y   wait:MS   key:NAME   shot:FILE
import { createRequire } from 'module';
const require = createRequire(import.meta.url);
let chromium;
try { ({ chromium } = require('playwright')); } catch { ({ chromium } = require('/opt/node-tools/node_modules/playwright')); }

const [url, out, ...actions] = process.argv.slice(2);
const browser = await chromium.launch({ args: ['--use-gl=swiftshader', '--enable-webgl', '--ignore-gpu-blocklist'] });
const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });
const errors = [];
page.on('console', m => {
  if (m.type() === 'error') errors.push(m.text());
  if (m.text().includes('selftest')) console.log(m.text());
});
page.on('pageerror', e => errors.push(String(e)));
await page.goto(url);
await page.waitForTimeout(3000);
for (const a of actions) {
  const [kind, arg] = a.split(/:(.*)/);
  if (kind === 'click') { const [x, y] = arg.split(',').map(Number); await page.mouse.click(x, y); await page.waitForTimeout(300); }
  else if (kind === 'move') { const [x, y] = arg.split(',').map(Number); await page.mouse.move(x, y); await page.waitForTimeout(300); }
  else if (kind === 'wait') await page.waitForTimeout(Number(arg));
  else if (kind === 'key') await page.keyboard.press(arg);
  else if (kind === 'shot') await page.screenshot({ path: arg });
}
await page.screenshot({ path: out });
console.log(JSON.stringify({ errors }));
await browser.close();
