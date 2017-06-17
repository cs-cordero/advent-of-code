#!/bin/python3

import re

def aoc(instructions):
    grid_fast = [int(''.join('0' for i in range(1000)), 2) for j in range(1000)]
    grid_slow = [[0 for i in range(1000)] for j in range(1000)]

    for instruction in instructions:
        action = instruction.groups()[0]
        x0, y0, x1, y1 = (int(x) for x in instruction.groups()[1:])
        bit_comparison = construct_bitstring(y0, y1)

        # Use fast bitwise logic for the first part of the challenge
        for x in range(x0, x1+1):
            target = grid_fast[x]
            compare = bit_comparison
            if action == 'turn off':
                compare = ~compare
                target &= compare
            elif action == 'turn on':
                target |= compare
            elif action == 'toggle':
                target ^= compare
            else:
                print('invalid action:  {}'.format(action))
            grid_fast[x] = target

        # Must use slow iteration for second part of challenge
        for x in range(x0, x1+1):
            for y in range(y0, y1+1):
                if action == 'turn off':
                    grid_slow[x][y] = max(grid_slow[x][y] - 1, 0)
                elif action == 'turn on':
                    grid_slow[x][y] += 1
                elif action == 'toggle':
                    grid_slow[x][y] += 2
                else:
                    print('invalid action:  {}'.format(action))

    lights_fast = sum(bin(x).count('1') for x in grid_fast)
    lights_slow = sum([sum(row) for row in grid_slow])

    print('Part 1:  {}.'.format(lights_fast))
    print('Part 2:  {}.'.format(lights_slow))

def construct_bitstring(begin, end):
    target = '1' * (end - begin + 1)
    tail = '0' * (1000 - end - 1)
    return int(target + tail, 2)

if __name__ == '__main__':
    parser = re.compile('(.*)\s(\d+),(\d+) through (\d+),(\d+)')
    with open('aoc_day_06_input.txt') as f:
        instructions = [parser.match(line) for line in f.readlines()]
    aoc(instructions)
