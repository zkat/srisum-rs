# `kdl` Release Changelog

<a name="4.0.0"></a>

## 4.0.0 (2022-06-11)

### Features

* **modernify:** update srisum to get with the times ([256b3f9e](https://github.com/kdl-org/kdl-rs/commit/256b3f9eb88b21f44396bd7ff6b6cc15d28d109a))

<a name="3.0.0"></a>

## 3.0.0 (2019-11-06)

#### Breaking Changes

- **errors:** rewrite API to use anyhow::Result instead of panicking ([793bd9e7](https://github.com/zkat/srisum-rs/commit/793bd9e75f089f6c4a75fa4f3b5e108fa17a8487))
- **license:** upgrade to Parity 7.0 release ([850f9266](https://github.com/zkat/srisum-rs/commit/850f926686a7869eecf456bc921d725f6db96640))

#### Bug Fixes

- **compute:** properly support non-UTF8 filenames for output. Note that `check` still can't handle reading funky filenames from a checksum file/stream ([d3759096](https://github.com/zkat/srisum-rs/commit/d375909685ae5d100dc5f832b74e09cdf16c3512))

#### Features

- **errors:** rewrite API to use anyhow::Result instead of panicking ([793bd9e7](https://github.com/zkat/srisum-rs/commit/793bd9e75f089f6c4a75fa4f3b5e108fa17a8487))
- **license:** upgrade to Parity 7.0 release ([850f9266](https://github.com/zkat/srisum-rs/commit/850f926686a7869eecf456bc921d725f6db96640))

<a name="2.0.0"></a>

## 2.0.0 (2019-10-21)

#### Breaking Changes

- **license:** switch license to Parity 7.0.0-pre.3 + Apache-2.0 ([db678d74](https://github.com/zkat/srisum-rs/commit/db678d740ed61fc99082762d532c787dc1243110))

#### Bug Fixes

- **compute:** use path.display() for cross-platform printing ([823c1cd5](https://github.com/zkat/srisum-rs/commit/823c1cd5235f99294ec3d4df8bbb2b6eda486def))
- **windows:** encode_wide does u16, not u8 ([a6ab4097](https://github.com/zkat/srisum-rs/commit/a6ab40978a7a7a4fa1eae3524f9bb0974193e5fb))

#### Features

- **license:** switch license to Parity 7.0.0-pre.3 + Apache-2.0 ([db678d74](https://github.com/zkat/srisum-rs/commit/db678d740ed61fc99082762d532c787dc1243110))
