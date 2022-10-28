#! /usr/bin/python3

import re
import argparse
import scryfallapi

def parseArguments():
    parser = argparse.ArgumentParser(description='Prints stats for a deck listing and recommendation of lands')
    parser.add_argument('deckfile', help='File with the deck listing')

    return parser.parse_args()

def parseLine(line):
    match = re.match(r'^(\w{3,4}) (\d+) (\w{2})$', line)
    if match:
        return match.group(1), match.group(2), match.group(3)
    else:
        raise Exception('invalid line: ' + line)


if __name__ == "__main__":
    args = parseArguments()

    deck = []
    with open(args.deckfile, 'r') as f:
        for line in f:
            print('Parsing: ' + line.strip())
            exp_code, number, lang = parseLine(line)

            try:
                card = scryfallapi.findCard(exp_code, number, lang)
                deck.append(card)
            except Exception as e:
                print('Could not find a card for: ' + line + ': ' + str(e))

    for card in deck:
        if 'printed_name' in card:
            print(card['printed_name'])
        else:
            print(card['name'])
