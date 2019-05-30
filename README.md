# stevia

[![Build Status](https://travis-ci.org/Pomettini/stevia.svg?branch=master)](https://travis-ci.org/Pomettini/stevia)
[![Build status](https://ci.appveyor.com/api/projects/status/19lf9hwujgk7mlc0?svg=true)](https://ci.appveyor.com/project/Pomettini/stevia)
[![Coverage Status](https://coveralls.io/repos/github/Pomettini/stevia/badge.svg?branch=master)](https://coveralls.io/github/Pomettini/stevia?branch=master)

A lightweight file format for simple visual novels that is easy to parse

It takes a subset of [Ink](https://github.com/inkle/ink) scripting language as the input

## Usage

Build the binary with:

```bash
cargo build --bin stevia
```

Run with:

```bash
./stevia file.ink
```

## Examples

Stevia will transform this:

```text
Hello there

I'm a VN written in the Ink format

Do you like it?

+ [Yes, I like it!] -> like
+ [No, I do not like it] -> hate

=== like

Thank you!

-> END

=== hate

Oh, I see

-> END
```

Into this:

```
P;Hello there|P;I'm a VN written in the Ink format|P;Do you like it?|Q;Yes, I like it!;00120;No, I do not like it;00136|P;Thank you!|E;|P;Oh, I see|E;
```

Still work in progress!