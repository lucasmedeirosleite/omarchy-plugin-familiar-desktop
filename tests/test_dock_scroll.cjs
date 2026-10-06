const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const scroll = {};
vm.createContext(scroll);
vm.runInContext(fs.readFileSync('DockScroll.js', 'utf8').replace(/^\.pragma library\s*/, ''), scroll);
let pending = 0;
for (let i = 0; i < 7; i++) {
  const result = scroll.step(pending, 0, -5, 0, -15);
  assert.equal(result.direction, 0);
  pending = result.pending;
}
assert.equal(scroll.step(pending, 0, -5, 0, -15).direction, 1);
assert.equal(scroll.step(0, 0, 0, 0, 120).direction, -1);
assert.equal(scroll.step(0, 50, 4, 0, 0).direction, 1);
assert.equal(scroll.step(30, -5, 0, 0, 0).pending, -5);
assert.equal(scroll.step(0, 0, 0, 0, -15).direction, 0);
console.log('smooth trackpad accumulation, axes and direction: passed');
