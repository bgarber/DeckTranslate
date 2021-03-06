#! /usr/bin/python3

import re
import argparse
import scryfallapi


def parseArguments():
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)

    parser.add_argument('-f', '--file',
            help='Use file as input data.', required=False)

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


def readInput(input_fd=None):
    line_list = list()

    if input_fd:
        read_line = input_fd.readline()
    else:
        read_line = str(input())
        while len(read_line) == 0:
            read_line = str(input())

    m = re.match(r"^//.*$", read_line)
    if m is not None:
        print('Ignoring line: ' + read_line)
        return None

    # Match a deck listing line
    m = re.match(r"(\d+) ([\w,\-\' /]+) \((\w{3,4})\) (\d+)", read_line)
    if m:
        for pos in m.groups():
            line_list.append(pos.strip())
    else:
        line_list.append(read_line)

    return line_list


def executeTranslation(langs, in_file=None):
    in_file_fd = None
    if in_file:
        in_file_fd = open(in_file, 'r')

    while True:
        try:
            line_list = readInput(in_file_fd)
            if line_list is None:
                continue

            card_name = ''
            card_set_code = None
            card_coll_n = None

            if len(line_list) == 1 and len(line_list[0]) == 0:
                break

            if len(line_list) == 1:
                # Handle this input as a single card
                card_name = line_list[0]
                try:
                    card_list, total_cards = scryfallapi.queryDatabase(card_name)
                    if total_cards == 1:
                        card_set_code = card_list[0]['set']
                        card_coll_n = card_list[0]['collector_number']
                    else:
                        print('Ambiguous input!')
                except Exception as e:
                    print(card_name + ': ' + str(e))
            elif len(line_list) == 4:
                # Handle this input as a card in a decklist
                card_name = line_list[1]
                card_set_code = line_list[2]
                card_coll_n = line_list[3]
            else:
                print('Could not understand input: ' + str(line_list))

            if card_set_code and card_coll_n:
                try:
                    translate_res = scryfallapi.findCard(card_set_code,
                            card_coll_n, langs[1])

                    if 'printed_name' in translate_res:
                        t_name = translate_res['printed_name']
                    else:
                        t_name = translate_res['name']

                    if len(line_list) == 4:
                        print(line_list[0] + ' ' + t_name + ' (' + card_set_code
                                + ') ' + card_coll_n)
                    else:
                        print(t_name)

                except Exception as e:
                    print(card_name + ': ' + str(e))

        except Exception as e:
            break

    if in_file_fd:
        in_file_fd.close()


if __name__ == "__main__":
    args = parseArguments()
    if args.list:
        print('Listing supported languages:')
    else:
        langs = validateTranslateString(args.translate)
        if langs is not None:
            executeTranslation(langs, args.file)
        else:
            print('Invalid \'translate\' string.')
