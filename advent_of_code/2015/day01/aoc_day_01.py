#!/bin/python3


def aoc(instructions):
    floor_map = { '(': 1, ')': -1 }

    current_floor = 0
    visited_basement = False
    for i, instruction in enumerate(instructions):
        current_floor += floor_map.get(instruction, 0)
        if not visited_basement and current_floor < 0:
            visited_basement = True
            visit = i + 1
    print('Part 1:  Finished on floor #{}.'.format(current_floor))
    print('Part 2:  Visited Basement on instruction #{}.'.format(visit))


if __name__ == '__main__':
    with open('aoc_day_01_input.txt') as f:
        instructions = ''.join(f.read().splitlines())
    aoc(instructions)
