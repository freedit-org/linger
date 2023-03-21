# linger

[![CI](https://github.com/freedit-org/linger/actions/workflows/ci.yml/badge.svg)](https://github.com/freedit-org/linger/actions/workflows/ci.yml)
[![release](https://github.com/freedit-org/linger/actions/workflows/release.yml/badge.svg)](https://github.com/freedit-org/linger/releases)

A simple, cross-platform, text-based utility for working with dictionaries written in Rust. Based on the api service <https://dictionaryapi.dev/>.

## Usage

* Intervative mode: `./linger -i`
* Direct mode: `./linger <word>`

tips: use `./linger <word> | less` for long page.

```
> ./linger -i
Input the word: ('e'/'exit' to quit)

rust
== rust ==

/ɹʌst/
https://api.dictionaryapi.dev/media/pronunciations/en/rust-us.mp3

【noun】
0. The deteriorated state of iron or steel as a result of moisture and oxidation.
1. A similar substance based on another metal (usually with qualification, such as "copper rust").
2. A reddish-brown color.
3. A disease of plants caused by a reddish-brown fungus.
4. Damage caused to stamps and album pages by a fungal infection.


== rust ==

/ɹʌst/
https://api.dictionaryapi.dev/media/pronunciations/en/rust-us.mp3

【verb】
0. To oxidize, especially of iron or steel.
1. To cause to oxidize.
2. To be affected with the parasitic fungus called rust.
3. To (cause to) degenerate in idleness; to make or become dull or impaired by inaction.
synonyms: ["corrode", "oxidise", "oxidize"]


Input the word: ('e'/'exit' to quit)
```