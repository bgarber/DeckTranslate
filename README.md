DeckTranslate
================================================================================

## 1. Disclaimer

DeckTranslate is unofficial Fan Content permitted under the Fan Content Policy.
Not approved/endorsed by Wizards. Portions of the materials used are property
of Wizards of the Coast. ©Wizards of the Coast LLC.

Card names and characters are all trademarks owned by Wizards of the Coast.

This software is distributed freely, and with good intentions. It was created
by a fan, for fans, and just for fun.

This software makes use of the Scryfall database API. Scryfall does not endorse
and/or approve this software by any means.

For a guide on the Scryfall API, please check https://scryfall.com/docs/api.
For a guide on the Wizards Fan Content Policy, please check https://company.wizards.com/fancontentpolicy

## 2. About

With the launch of the Magic: the Gathering Arena came the possibilty to import
and export decks to/from the Internet. By default, the Arena will use your
system's language, which is not bad at all. But, if you wish to export and share
that awesome deck you created on sites like TappedOut.net, that site won't
understand your deck, because the site understands English, but your deck is on
portuguese.

Translating card by card yor deck listing is a pain. So, this tool helps you
translating your entire deck.

## 3. Usage

Example:

    decktr -t pt:en

The software will enter a loop, where you can type the card and it will return
the card name in the selected language. For example:

    decktr -t pt:en
    Rato Ladrão
    Burglar Rat

Use standard input/output to read/write to/from files:

    decktr -t en:pt < decksample.txt
