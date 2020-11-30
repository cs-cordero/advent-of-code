from __future__ import annotations

from typing import List

from intcode import IntcodeComputer


def solution1(data: List[int]) -> int:
    answer = 0
    for x in range(50):
        for y in range(50):
            answer += get_result(data, x, y)
    return answer


def solution2(data: List[int]) -> int:
    current_x, current_y = 0, 10

    # Find the first x in the 100th row
    while not get_result(data, current_x, current_y):
        current_x += 1

    while True:
        opposite_corner = (current_x + 99, current_y - 99)
        if get_result(data, *opposite_corner):
            result = (current_x * 10000) + (current_y - 99)
            return result

        current_y += 1
        while not get_result(data, current_x, current_y):
            current_x += 1


def get_result(data: List[int], x: int, y: int) -> int:
    computer = IntcodeComputer(data)
    computer.send(x)
    computer.send(y)
    computer.run_until_blocked()
    assert computer.has_output()
    result = computer.read()
    return result


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().split(",")))
    # print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
