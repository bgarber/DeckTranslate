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
Portuguese.

Translating card by card your deck listing is a pain. So, this tool aims to
help you translating your entire deck.

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
    4 Rato Ladrão (GRN) 64
    2 Fuga Sombria de Davriel (WAR) 84
    2 Davriel, Mago das Sombras Ladino (WAR) 83
    4 Desfigurar (M20) 95
    3 Coagir (M20) 97
    2 Cavaleiro da Legião de Ébano (M20) 105
    3 Ginete Homicida (ELD) 97
    4 Dossel Esmagador (GRN) 126
    3 Despojar Alma (ELD) 103
    1 Prole do Pandemônio (RNA) 85
    23 Pântano (M20) 269
    2 Assombração da Torre Alta (RNA) 273
    4 Rumores Pravos (GRN) 89
    3 Espreitador Pantaneiro de Yarok (M20) 123

On Windows, you will need to use the '-f' parameter (Windows doesn't support
standard input redirection):

    > python decktr.py -t en:pt -f decksample.txt
    4 Rato Ladrão (GRN) 64
    2 Fuga Sombria de Davriel (WAR) 84
    2 Davriel, Mago das Sombras Ladino (WAR) 83
    4 Desfigurar (M20) 95
    3 Coagir (M20) 97
    2 Cavaleiro da Legião de Ébano (M20) 105
    3 Ginete Homicida (ELD) 97
    4 Dossel Esmagador (GRN) 126
    3 Despojar Alma (ELD) 103
    1 Prole do Pandemônio (RNA) 85
    23 Pântano (M20) 269
    2 Assombração da Torre Alta (RNA) 273
    4 Rumores Pravos (GRN) 89
    3 Espreitador Pantaneiro de Yarok (M20) 123
