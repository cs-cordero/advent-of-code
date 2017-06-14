#!/bin/python3

import re

def security_through_obscurity(room_codes):
    sector_id_sum = 0
    for line in room_codes:
        m = re.search('([a-z\-]*)(\d+)\[(\w*)\]', line)
        room_code, sector_id, check_sum = m.groups()
        shift = int(sector_id) % 26
        room_shifted = []

        # decipher all of the characters
        for char in room_code:
            if char == '-':
                room_shifted.append(' ')
                continue
            room_shifted.append(chr((((ord(char) + shift) - 97) % 26) + 97))
        room_shifted = ''.join(char for char in room_shifted)

        # filter for where North Pole Items are stored
        if re.search('storage', room_shifted):
            print('{}: {}'.format(sector_id, room_shifted))

if __name__ == '__main__':
    sample = ['qzmt-zixmtkozy-ivhz-343[wjvzd]']
    print('Sample:')
    security_through_obscurity(sample)
    print('\n')

    advent_input = []
    print('Challenge:')
    with open('aoc_day_04_input.txt', 'r') as f:
        for line in f:
            advent_input.append(line)
    security_through_obscurity(advent_input)