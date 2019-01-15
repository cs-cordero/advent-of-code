from collections import defaultdict
from typing import List
import sys


if 'test' not in sys.argv:
    PUZZLE_INPUT = sorted([11, 30, 47, 31, 32, 36, 3, 1, 5, 3, 32, 36, 15, 11, 46,
                           26, 28, 1, 19, 3])
    TARGET = 150
else:
    PUZZLE_INPUT = sorted([20, 15, 10, 5, 5])
    TARGET = 25


def helper(current_sum: int, index: int, current: List[int],
           target: int) -> List[List[str]]:
    answers = []
    for i in range(index, len(PUZZLE_INPUT)):
        next_value = PUZZLE_INPUT[i]
        next_collection = current + [next_value]
        next_sum = current_sum + next_value
        if next_sum == target:
            answers.append(next_collection)
            continue
        if next_sum > target:
            break
        answers.extend(helper(next_sum, i+1, next_collection, target))
    return answers


def solution(target: int = 150) -> int:
    combinations = helper(0, 0, [], target)
    part1 = len(combinations)

    count_dict = defaultdict(int)
    for combination in combinations:
        count_dict[str(len(combination))] += 1
    part2 = min(count_dict.items(), key=lambda x: int(x[0]))[1]

    return part1, part2


answer = solution(target=TARGET)
print(answer)
