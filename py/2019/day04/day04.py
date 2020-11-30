from collections import Counter
from typing import List

PUZZLE_INPUT = "240920-789857"


def solution1(data: List[int]) -> int:
    count = 0
    for i in range(data[0], data[1] + 1):
        number_as_list = list(str(i))
        has_adjacent = False
        never_decrease = True
        for j in range(5):
            if number_as_list[j] == number_as_list[j + 1]:
                has_adjacent = True
            elif number_as_list[j] > number_as_list[j + 1]:
                never_decrease = False
                break
        if has_adjacent and never_decrease:
            count += 1
    return count


def solution2(data: List[int]) -> int:
    count = 0
    for i in range(data[0], data[1] + 1):
        number_as_list = list(str(i))
        never_decrease = True
        for j in range(5):
            if number_as_list[j] > number_as_list[j + 1]:
                never_decrease = False
                break
        if (
            any(value == 2 for value in Counter(number_as_list).values())
            and never_decrease
        ):
            count += 1
    return count


if __name__ == "__main__":
    data = list(map(int, PUZZLE_INPUT.split("-")))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
