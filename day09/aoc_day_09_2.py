#!/bin/python3

import re


def main(s):
    counter = 0
    while s:
        # print(s, counter)
        m = re.match(r'^(\((\d+)x(\d+)\))', s) # gets groups
        if not m:
            next_dec = re.search(r'\(', s)
            if not next_dec:
                counter += len(s)
                return counter
            move_to = next_dec.start()
            s = s[move_to:]
            counter += move_to
            continue

        start = len(m.group(1))
        end = start + int(m.group(2))
        unpack = s[start:end]
        if not re.match(r'^(\((\d+)x(\d+)\))', unpack):
            # print(unpack, counter)
            counter += len(unpack) * int(m.group(3))
            s = s[end:]
            continue
        s = unpack * int(m.group(3)) + s[end:]
    return counter


def load():
    with open("aoc_day_09_input.txt", "r") as f:
        ins = ''.join(re.sub(r'\s*', r'', line) for line in f)
    return main(ins)


print('Sample 1: %d' % main('(3x3)XYZ'))
print('Sample 2: %d' % main('X(8x2)(3x3)ABCY'))
print('Sample 3: %d' % main('(27x12)(20x12)(13x14)(7x10)(1x12)A'))
print('Sample 4: %d' % main('(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN'))
print('Challenge: %d' % load())