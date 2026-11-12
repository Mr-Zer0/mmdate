#!/usr/bin/env node
// Generates ground-truth golden vectors from the upstream reference
// implementation (yan9a/mmcal, MIT © 2018 Yan Naing Aye) for this crate's
// test suite to check its Rust port against.
//
// NOT shipped in the crate (see Cargo.toml `exclude`); this is provenance
// tooling only, run manually when fixtures need to be regenerated.
//
// Usage: node util/generate_fixtures.js [path-to-ceMmDateTime.js]

'use strict';
const fs = require('fs');
const path = require('path');
const vm = require('vm');

const srcPath =
  process.argv[2] ||
  '/Users/yannaing/experiment/mmcal/javascript/ceMmDateTime.js';

function loadUpstream(file) {
  let src = fs.readFileSync(file, 'utf8');

  // The file's top-level IIFE loads chronicle JSON over `fetch` using
  // browser globals (`document.currentScript`) that don't exist in Node and
  // aren't needed for date conversion. Strip it; the `ceDateTime` and
  // `ceMmDateTime` classes underneath are plain, dependency-free JS.
  const iifeStart = src.indexOf('(() => {');
  const iifeMarker = "globalThis.ceMmChronicleDataReady = Promise.all(";
  const markerIdx = src.indexOf(iifeMarker);
  if (iifeStart === -1 || markerIdx === -1) {
    throw new Error('Could not locate the browser-only IIFE to strip; upstream file format may have changed.');
  }
  const closeIdx = src.indexOf('})();', markerIdx);
  if (closeIdx === -1) {
    throw new Error('Could not find the end of the browser-only IIFE.');
  }
  const iifeEnd = closeIdx + '})();'.length;
  src = src.slice(0, iifeStart) + src.slice(iifeEnd);

  src += '\nmodule.exports = { ceDateTime, ceMmDateTime };\n';

  const sandbox = { module: { exports: {} }, exports: {}, console };
  vm.createContext(sandbox);
  vm.runInContext(src, sandbox, { filename: file });
  return sandbox.module.exports;
}

const { ceDateTime, ceMmDateTime } = loadUpstream(srcPath);

const SG = 2361222;
const CT_GREGORIAN = 1;

// Thingyan astronomical constants, ported directly from the calculation
// embedded in `cal_holiday` (`ceMmDateTime.js:1352-1366`) — there is no
// standalone static method for it upstream, so this is inlined here exactly
// as upstream computes it (not re-derived).
const THINGYAN_SY = 1577917828.0 / 4320000.0;
const THINGYAN_MO = 1954168.050623;
const THINGYAN_BGNTG = 1100;
const THINGYAN_SE3 = 1312;

function thingyanFor(my, mmt) {
  const myEff = my + mmt;
  if (myEff < THINGYAN_BGNTG) return null;
  const ja = THINGYAN_SY * myEff + THINGYAN_MO; // atat time
  const jk = my >= THINGYAN_SE3 ? ja - 2.169918982 : ja - 2.1675; // akya time
  const akn = Math.round(jk);
  const atn = Math.round(ja);
  return { akn, atn, newyear: atn + 1, ja, jk };
}

function rowFor(y, m, d) {
  const jd = ceDateTime.w2j(y, m, d, 12, 0, 0, CT_GREGORIAN, SG);
  const jdn = Math.round(jd);
  const wd = (jdn + 2) % 7;
  const mdt = ceMmDateTime.j2m(jdn);
  const mp = ceMmDateTime.cal_mp(mdt.md, mdt.mm, mdt.myt);
  const mf = ceMmDateTime.cal_mf(mdt.md);
  const sy = ceMmDateTime.my2sy(mdt.my, mdt.mm, mdt.md, 0);

  const yearName = mdt.my % 12;
  const nagahle = ceMmDateTime.cal_nagahle(mdt.mm);
  const mahabote = ceMmDateTime.cal_mahabote(mdt.my, wd);
  const nakhat = ceMmDateTime.cal_nakhat(mdt.my);
  const sabbath = ceMmDateTime.cal_sabbath(mdt.md, mdt.mm, mdt.myt);
  const yatyaza = ceMmDateTime.cal_yatyaza(mdt.mm, wd);
  const pyathada = ceMmDateTime.cal_pyathada(mdt.mm, wd);
  const thamanyo = ceMmDateTime.cal_thamanyo(mdt.mm, wd);
  const amyeittasote = ceMmDateTime.cal_amyeittasote(mdt.md, wd);
  const warameittugyi = ceMmDateTime.cal_warameittugyi(mdt.md, wd);
  const warameittunge = ceMmDateTime.cal_warameittunge(mdt.md, wd);
  const yatpote = ceMmDateTime.cal_yatpote(mdt.md, wd);
  const thamaphyu = ceMmDateTime.cal_thamaphyu(mdt.md, wd);
  const nagapor = ceMmDateTime.cal_nagapor(mdt.md, wd);
  const yatyotema = ceMmDateTime.cal_yatyotema(mdt.mm, mdt.md);
  const mahayatkyan = ceMmDateTime.cal_mahayatkyan(mdt.mm, mdt.md);
  const shanyat = ceMmDateTime.cal_shanyat(mdt.mm, mdt.md);

  const mmt = Math.floor(mdt.mm / 13);
  const thingyan = thingyanFor(mdt.my, mmt);
  const thingyanValid = thingyan ? 1 : 0;
  const akyaJdn = thingyan ? thingyan.akn : '';
  const atatJdn = thingyan ? thingyan.atn : '';
  const newYearJdn = thingyan ? thingyan.newyear : '';
  const akyaTime = thingyan ? thingyan.jk : '';
  const atatTime = thingyan ? thingyan.ja : '';

  return [
    y, m, d, jdn, mdt.my, mdt.mm, mdt.md, mdt.myt, mp, mf, wd, sy,
    yearName, nagahle, mahabote, nakhat, sabbath, yatyaza, pyathada,
    thamanyo, amyeittasote, warameittugyi, warameittunge, yatpote,
    thamaphyu, nagapor, yatyotema, mahayatkyan, shanyat,
    thingyanValid, akyaJdn, atatJdn, newYearJdn, akyaTime, atatTime,
  ].join(',');
}

function* daysInGregorianRange(y1, y2) {
  for (let y = y1; y <= y2; y++) {
    const leap = (y % 4 === 0 && y % 100 !== 0) || y % 400 === 0;
    const lens = [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (let m = 1; m <= 12; m++) {
      for (let d = 1; d <= lens[m - 1]; d++) {
        yield [y, m, d];
      }
    }
  }
}

const rows = new Map(); // keyed by jdn to de-duplicate overlapping windows

function addRange(y1, y2) {
  for (const [y, m, d] of daysInGregorianRange(y1, y2)) {
    const line = rowFor(y, m, d);
    const jdn = line.split(',')[3];
    rows.set(jdn, line);
  }
}

// 1. Dense recent range.
addRange(2015, 2035);

// 2. Every era boundary (ME 798, 1100, 1217, 1312), exhaustive daily for a
// window around the approximate corresponding Gregorian year
// (Myanmar Era year 0 ~= 638 CE).
for (const my of [798, 1100, 1217, 1312]) {
  const centerYear = my + 638;
  addRange(centerYear - 3, centerYear + 3);
}

// 3. Every watat-exception (wte) and full-moon-exception (fme) year from
// GetMyConst, exhaustive daily for a window around its approximate
// corresponding Gregorian year.
const wteYears = [1344, 1345, 1263, 1264, 1201, 1202];
const fmeYears = [
  1377, 1234, 1261, 1120, 1126, 1150, 1172, 1207, 813, 849, 851, 854, 927, 933,
  936, 938, 949, 952, 963, 968, 1039, 205, 246, 471, 572, 651, 653, 656, 672,
  729, 767,
];
for (const my of [...new Set([...wteYears, ...fmeYears])]) {
  const centerYear = my + 638;
  addRange(centerYear - 2, centerYear + 2);
}

const sorted = [...rows.values()].sort((a, b) => Number(a.split(',')[3]) - Number(b.split(',')[3]));

const header = [
  'gregorian_year', 'gregorian_month', 'gregorian_day', 'jdn', 'myanmar_year',
  'myanmar_month', 'myanmar_day', 'year_type', 'moon_phase', 'fortnight_day',
  'weekday', 'sasana_year',
  'year_name', 'nagahle', 'mahabote', 'nakhat', 'sabbath', 'yatyaza', 'pyathada',
  'thamanyo', 'amyeittasote', 'warameittugyi', 'warameittunge', 'yatpote',
  'thamaphyu', 'nagapor', 'yatyotema', 'mahayatkyan', 'shanyat',
  'thingyan_valid', 'akya_jdn', 'atat_jdn', 'new_year_jdn', 'akya_time', 'atat_time',
].join(',');
const outPath = path.join(__dirname, '..', 'tests', 'fixtures', 'golden_vectors.csv');
fs.writeFileSync(outPath, header + '\n' + sorted.join('\n') + '\n');

process.stderr.write(`Wrote ${sorted.length} rows to ${outPath}\n`);
