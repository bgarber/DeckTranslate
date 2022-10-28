# DeckTranslate

## 1. Disclaimer

DeckTranslate is unofficial Fan Content permitted under the Fan Content Policy.
Not approved/endorsed by Wizards. Portions of the materials used are property
of Wizards of the Coast. ©Wizards of the Coast LLC. Card names, characters, and
lore are all trademarks owned by Wizards of the Coast. Refer to <https://company.wizards.com/fancontentpolicy>
for a Wizards Fan Content Policy guide.

This software makes use of the Scryfall database API. Scryfall does not endorse
and/or approve this software by any means. Refer to <https://scryfall.com/docs/api>
site for a Scryfall API guide.

DeckTranslate is made by a fan, for fans, and just for fun. It is _free software_,
under GPLv3 regulation.

## 2. About

With the launch of the Magic: the Gathering Arena came the possibility to import
and export decks to/from the Internet. By default, the Arena will use your system's
language, which is not bad at all. But if you wish to share that incredible deck
you created on sites like TappedOut.net, you'll need to translate your deck listing
to English. Translating card by card your deck listing is a pain. This tool will
do that for you!

## 3. Installation & Usage

For now, you must clone this repository. You may run it using the Cargo command:
```bash
cargo run -- -h
```

Or build and run it:
```bash
cargo build && target/debug/deck-translate -h
```

Or you can install it:
```bash
cargo install --path .
```

Usage example:

```
$ cargo build --release
   Compiling deck-translate v0.1.0 (/home/bgarber/Projects/DeckTranslate)
    Finished release [optimized] target(s) in 2.93s
$ target/release/deck-translate -l pt tests/decksample.txt
4 Rato Ladrão (GRN) 64
2 Fuga Sombria de Davriel (WAR) 84
2 Davriel, Mago das Sombras Ladino (WAR) 83
4 Desfigurar (M20) 95
3 Coagir (M20) 97
2 Cavaleiro da Legião de Ébano (M20) 105
3 Ginete Homicida (ELD) 97
4 Empesteiro (GRN) 82
3 Despojar Alma (ELD) 103
1 Prole do Pandemônio (RNA) 85
23 Pântano (M20) 269
2 Assombração da Torre Alta (RNA) 273
4 Rumores Pravos (GRN) 89
3 Espreitador Pantaneiro de Yarok (M20) 123
```

