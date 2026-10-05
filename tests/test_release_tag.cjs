const assert = require('node:assert/strict');
const {verifyTag} = require('../scripts/verify-release-tag.cjs');
assert.equal(verifyTag('v0.1.0', '0.1.0', false), true);
assert.equal(verifyTag('v0.1.0-rc.4', '0.1.0', true), true);
for (const [tag, prerelease] of [
  ['v0.1.0-rc.4', false], ['v0.1.0', true], ['v0.0.6', false],
  ['v0.2.0-rc.1', true], ['v0.1.0-rc.0', true], ['v0.1.0-rc.01', true],
  ['v0.1.0-rc.', true], ['v0.1.0-rc.1-extra', true], ['', false],
]) assert.equal(verifyTag(tag, '0.1.0', prerelease), false, tag);
console.log('Release identity: final/RC match; wrong versions, flags and malformed tags rejected.');
