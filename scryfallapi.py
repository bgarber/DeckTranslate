import requests
import json
import time


def __execute_scryfall_req(req):
    scryfall_url = 'https://api.scryfall.com/'

    # Scryfall kindly asks us to put a 50ms delay between requests.
    time.sleep(0.05)

    #print(scryfall_url + req)

    req = requests.get(scryfall_url + req)
    data = json.loads(req.text)

    if data['object'] == 'error' and data['status'] == 404:
        raise Exception('Card not found.')

    return data


def queryDatabase(query):
    data = __execute_scryfall_req('cards/search?q=' + query)
    query_result = data['data']

    return query_result, data['total_cards']


def findCard(set_code, collector_n, lang):
    t_card = __execute_scryfall_req('cards/' + set_code.lower() + '/'
            + collector_n + '/' + lang)

    return t_card
