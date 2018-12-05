from datetime import datetime, timedelta
from collections import defaultdict


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.strip()


def react_polymer(polymer):
    i = 0
    ascii_diff = abs(ord('a') - ord('A'))
    while i < len(polymer)-1:
        current_unit, next_unit = polymer[i], polymer[i+1]
        if abs(ord(current_unit) - ord(next_unit)) == ascii_diff:
            polymer = polymer[:i] + polymer[i+2:]
            i = max(i-1, 0)
            continue
        i += 1
    return polymer


def solution():
    with open('input.txt') as f:
        polymer = f.readline().strip()
    return len(react_polymer(polymer))


def solution2():
    with open('input.txt') as f:
        polymer = f.readline().strip()

    mapping = {}
    for x in range(ord('A'), ord('Z')+1):
        upper = chr(x)
        lower = chr(x + ord('a') - ord('A'))
        _polymer = ''.join(unit for unit in polymer if unit not in (upper, lower))
        mapping[chr(x)] = len(react_polymer(_polymer))
    return sorted(mapping.items(), key=lambda x: x[1])[0]


print(solution())
print(solution2())
