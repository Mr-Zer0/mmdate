# Third-party notices

The Gregorian <-> Myanmar calendar conversion algorithm in this crate is a
Rust re-expression of the calculation logic published by **Yan Naing Aye** in
the **mmcal** project ("mc1500 / Modern Myanmar Calendrical Calculations"):

- Project: <https://github.com/yan9a/mmcal>
- Reference implementation used: `javascript/ceMmDateTime.js`
- Algorithm write-up: [Algorithm, Program and Calculation of Myanmar
  Calendar](http://cool-emerald.blogspot.com/2013/06/algorithm-program-and-calculation-of.html)
  (cool-emerald, 2013)

This crate does not copy mmcal's source text; every ported routine has been
re-expressed as idiomatic Rust (see the `// Ported from yan9a/mmcal (MIT, ©
2018 Yan Naing Aye).` attribution comment at the top of each source file that
is a direct port). The underlying empirical constants, era tables, and
exception years are reproduced verbatim, as required for calendrical
correctness, and are not an original creative contribution of this crate.

mmcal is distributed under the MIT License. Its copyright notice and
permission notice are reproduced below in full, as required by that license.

## mmcal license text

```
MIT License

Copyright (c) 2018 Yan Naing Aye

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## This crate's own license

This crate (`myanmar-date`) is itself dual-licensed under `MIT OR Apache-2.0`
— see `LICENSE-MIT` and `LICENSE-APACHE` in the repository root. That license
applies to this crate's own source code; it does not replace or relicense
mmcal, whose MIT notice above must be preserved by anyone redistributing the
calculation logic it originated.
