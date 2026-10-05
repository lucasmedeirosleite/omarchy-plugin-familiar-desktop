// Keep final releases exact; numbered RCs may use the final package version.
const fs = require('node:fs');
function verifyTag(tag, version, prerelease) {
  if (!/^\d+\.\d+\.\d+$/.test(version)) return false;
  const base = `v${version}`;
  return prerelease
    ? tag.startsWith(`${base}-rc.`) && /^[1-9]\d*$/.test(tag.slice(`${base}-rc.`.length))
    : tag === base;
}
if (require.main === module) {
  const version = JSON.parse(fs.readFileSync('manifest.json')).version;
  if (!verifyTag(process.env.TAG || '', version, process.env.PRERELEASE === 'true')) {
    console.error(`Release tag ${process.env.TAG} does not match ${version} and its prerelease status.`);
    process.exit(1);
  }
}
module.exports = {verifyTag};
