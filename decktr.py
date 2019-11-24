#! /usr/bin/python3

import argparse
import scryfallapi
import re


def parseArguments():
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)

    group.add_argument('-t', '--translate',
            help='Languages to translate, in the format \'to:from\'.')
    group.add_argument('-l', '--list', action='store_true',
            help='List supported languages.')

    return parser.parse_args()


def validateTranslateString(tr_str):
    langs = None

    if ':' in tr_str:
        langs = tr_str.split(':')
        if len(langs) > 2:
            print('More than 2 languages given; only the first 2 will be used.')
            del langs[2:]

    return langs


def readInput():
    line_list = list()

    read_line = str(input())
    while len(read_line) == 0:
        read_line = str(input())

    # Match a deck listing line
    m = re.match(r'(\d+) ([\w,\' ]+)\((\w{3,4})\) (\d+)', read_line)
    if m:
        for pos in m.groups():
            line_list.append(pos.strip())
    else:
        line_list.append(read_line)

    return line_list


def executeTranslation(langs):
    while True:
        try:
            line_list = readInput()
            card_set_code = None
            card_coll_n = None

            if len(line_list) == 1:
                # Handle this input as a single card
                card_name = line_list[0]
                card_list, total_cards = scryfallapi.queryDatabase(card_name)
                if total_cards == 1:
                    card_set_code = card_list[0]['set']
                    card_coll_n = card_list[0]['collector_number']
                else:
                    print('Ambiguous input!')
            elif len(line_list) == 4:
                # Handle this input as a card in a decklist
                card_set_code = line_list[2]
                card_coll_n = line_list[3]
            else:
                print('Could not understand input: ' + str(line_list))

            if card_set_code and card_coll_n:
                translate_res = scryfallapi.findCard(card_set_code,
                        card_coll_n, langs[1])

                if 'printed_name' in translate_res:
                    t_name = translate_res['printed_name']
                elif 'card_faces' in translate_res:
                    t_name = translate_res['card_faces'][0]['printed_name']
                else:
                    t_name = '???'

                if len(line_list) == 4:
                    print(line_list[0] + ' ' + t_name + ' (' + card_set_code
                            + ') ' + card_coll_n)
                else:
                    print(t_name)
        except Exception as e:
            print(str(e))
            break


if __name__ == "__main__":
    args = parseArguments()
    if args.list:
        print('Listing supported languages:')
    else:
        langs = validateTranslateString(args.translate)
        if langs is not None:
            executeTranslation(langs)
        else:
            print('Invalid \'translate\' string.')
