#!/bin/python3

import re
import copy


def increment(curr_pos, max_pos, inc):
    for i in range(len(curr_pos)):
        curr_pos[i] = (curr_pos[i] + inc) % max_pos[i]
    return curr_pos


def day15(curr_pos, max_pos):
    steps = 0
    while not all(i == 0 for i in curr_pos):
        # get incrementor
        incrementor = 1
        for i in range(len(curr_pos)):
            if curr_pos[i] == 0:
                incrementor *= max_pos[i]

        # increment everything
        increment(curr_pos, max_pos, incrementor)

        # increment step counter
        steps += incrementor

    return steps - 1


def load(fn):
    max_positions = []
    curr_positions = []
    with open('aoc_day_15_{}.txt'.format(fn)) as f:
        m = re.compile(r'^.*#(\d+)[\D]*(\d+).*positions;.*position (\d+)')
        for line in f:
            r = m.search(line).groups()
            max_positions.append(int(r[1]))
            curr_positions.append((int(r[2]) + (int(r[0])-1)) % int(r[1]))
    return curr_positions, max_positions

curr_pos, max_pos = load('input')
# curr_pos, max_pos = load('sample')
print('Part 1: {}'.format(day15(copy.deepcopy(curr_pos), max_pos)))

curr_pos.append((0 + 6) % 11)
max_pos.append(11)
print('Part 2: {}'.format(day15(curr_pos, max_pos)))
