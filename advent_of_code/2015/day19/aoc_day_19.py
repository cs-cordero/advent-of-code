from collections import deque
from typing import Tuple, List, Set
import sys


def get_data() -> Tuple[List[Tuple[str, str]], str]:
    if 'test' in sys.argv:
        replacements = [
            ('e', 'H'),
            ('e', 'O'),
            ('H', 'HO'),
            ('H', 'OH'),
            ('O', 'HH'),
        ]
        molecule = 'HOHOHO'
    else:
        replacements = []
        with open('aoc_day_19_input.txt') as f:
            while True:
                line = f.readline().strip()
                if not line:
                    break
                source, target = line.split(' => ')
                replacements.append((source, target))
            molecule = f.readline().strip()

    return replacements, molecule


def make_replacements(molecule: str, replacement: Tuple[str, str]) -> Set[str]:
    results = set()
    source, target = replacement
    prev_i = -1
    while True:
        try:
            i = molecule.index(source, prev_i+1)
        except ValueError:
            break
        result = f'{molecule[:i]}{target}{molecule[i+len(source):]}'
        results.add(result)
        prev_i = i
    return results


def solution():
    replacements, target_molecule = get_data()
    results = set()
    for replacement in replacements:
        results |= make_replacements(target_molecule, replacement)
    part1 = len(results)

    replacement_dict = {}
    for replacement in replacements:
        replacement_dict.setdefault(replacement[0], []).append(replacement)

    queue = deque()
    for starter in replacement_dict['e']:
        queue.append(('e', starter, 0))

    part2 = None
    seen = set()
    highest_count_seen = 0
    while queue:
        # print(len(queue))
        current, replacement, count = queue.popleft()
        next_results = make_replacements(current, replacement)
        for result in next_results:
            if result in seen:
                continue
            seen.add(result)

            if result == target_molecule:
                part2 = count+1
                break

            if len(result) >= len(target_molecule):
                continue

            for source, next_replacements in replacement_dict.items():
                if source not in result:
                    continue

                next_replacements = replacement_dict[source]
                for next_replacement in next_replacements:
                    a, b = next_replacement
                    if len(result) + len(b) - len(a) > len(target_molecule):
                        continue
                    if count+1 > highest_count_seen:
                        highest_count_seen = count+1
                        print(highest_count_seen)
                    queue.append((result, next_replacement, count+1))
        if part2:
            break
    return part1, part2


print(solution())
