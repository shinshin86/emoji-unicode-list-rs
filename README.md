# emoji-unicode-list-rs
This program is used to get a list of unicode of emoji.

The data is obtained from this site.  
[UnicodeのEmojiの一覧](https://ja.wikipedia.org/wiki/Unicode%E3%81%AEEmoji%E3%81%AE%E4%B8%80%E8%A6%A7)

## Usage

```sh
cargo run > unicode.json
```

We get data with this kind of structure.
* `unicode` : emoji's unicode.
* `name` : emoji's name.

```json
[{"unicode":"U+0023","name":"NUMBER SIGN"},{"unicode":"U+002A","name":"ASTERISK"},{"unicode":"U+0030","name":"DIGIT ZERO"}...]
```