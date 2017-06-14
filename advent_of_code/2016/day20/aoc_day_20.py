#!/bin/python3

import collections


def main():
    with open('aoc_day_20_input.txt') as f:
        instructions = collections.deque(sorted(
                       [(int(x.split('-')[0]), int(x.split('-')[1]))
                       for x in f.readlines()]))

    part_one = 0
    while instructions:
        instruction = instructions.popleft()
        if instruction[0] <= part_one:
            part_one = max(part_one, instruction[1] + 1) 
        else:
            instructions.appendleft(instruction)
            break
    print(part_one)

    part_two = 0
    while instructions:
        instruction = instructions.popleft()
        part_two += max(instruction[0] - part_one, 0)
        part_one = max(instruction[1] + 1, part_one)
    print(part_two)


if __name__ == '__main__':
    main()