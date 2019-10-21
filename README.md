# srisum(1) -- compute and check subresource integrity digests

## SYNOPSIS

`$ srisum [OPTION]... [FILE]...`

## EXAMPLES

### Computing SRI Digests

For a single file:

```
$ srisum styles.css > styles.css.sri
```

For multiple different files:

```
$ srisum styles.css index.js package.json bundle.js > app.sri
```

From `stdin`:

```
$ cat styles.css | srisum -a sha1
sha1-hmkHOZdrfLUVOqpAgryfC8XNGtE -
```

Specify algorithms to generate:

```
$ srisum styles.css index.js --algorithms sha512 sha256 sha1 > styles.css.sri
```

### Checking Integrity

Passing checksum file as an argument:

```
$ srisum -c styles.css.sri
styles.css: OK (sha512)
```

Passing multiple checksum files:

```
$ srisum -c styles.css.sri js-files.sri
styles.css: OK (sha512)
index.js: OK (sha512)
lib/util.js: OK (sha512)
```

Checksum file from `stdin`:

```
$ cat styles.css.sri | srisum -c
styles.css: OK (sha512)
```

Checksum `stdin` itself:

```
$ echo "hello" | srisum > stdin.sri
$ echo "hello" | srisum -c stdin.sri
-: OK (sha512)
```

## DESCRIPTION

Print or check Subresource Integrity digests.

Spec: https://w3c.github.io/webappsec/specs/subresourceintegrity/

`srisum`'s API is based on the `SHA[N]SUM(1)` family of unix utilities.

With no `FILE` or when `FILE` is `-`, read standard input.

`-a, --algorithms [ALGO]...` - hash algorithms to generate for the `FILE`s

`-c, --check` - read SRI sums from the `FILE`s and check them

`-d, --digest-only` - only output the digest for each `FILE`, without filenames

`--help` - display help and exit

`--version` - output version information and exit

## The following options are useful only when verifying integrity:

`--ignore-missing` - don't fail or report status for missing files

`--quiet` - don't print OK for each successfully verified file

`--status` - don't output anything, status code shows success

`-w, --warn` - warn about improperly formatted SRI lines

When checking, the input should be a former output of this program. The default mode is to print line with space-separated SRI digests, one more space, and a name for each FILE.

Strict mode, enabled with `--strict`, will entirely ignore digests (in input and output) that fail all of the following conditions:

- `algorithms` must be one or more of: `sha1`, `sha256`, `sha384`, `sha512`
- digest strings must be valid `RFC4648` `Base64` strings.

## AUTHOR

Written by [Kat Marchan](https://github.com/zkat)

## CONTRIBUTING

The srisum team enthusiastically welcomes contributions and project participation! There's a bunch of things you can do if you want to contribute! The [Contributor Guide](CONTRIBUTING.md) has all the information you need for everything from reporting bugs to contributing entire new features. Please don't hesitate to jump in if you'd like to, or even ask us questions if something isn't clear.

All participants and maintainers in this project are expected to follow [Code of Conduct](CODE_OF_CONDUCT.md), and just generally be excellent to each other.

Happy hacking!

## LICENSE

This project is licensed under [the Parity License](LICENSE-PARITY.md). Third-party contributions are licensed under [Apache-2.0](LICENSE-APACHE.md) and belong to their respective authors.

The Parity License is a copyleft license that, unlike the GPL family, allows you to license derivative and connected works under permissive licenses like MIT or Apache-2.0. It's free to use provided the work you do is freely available!

For proprietary use, please [contact me](mailto:kzm@zkat.tech?subject=srisum%20license), or just [sponsor me on GitHub](https://github.com/users/zkat/sponsorship) under the appropriate tier to [acquire a proprietary-use license](LICENSE-PATRON.md)! This funding model helps me make my work sustainable and compensates me for the work it took to write this crate!

## SEE ALSO

- `shasum(1)`
- `sha1sum(1)`
