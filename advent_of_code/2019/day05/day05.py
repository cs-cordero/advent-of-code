from typing import List

from intcode import IntcodeComputer


def solution1(data: List[int]) -> int:
    computer = IntcodeComputer(data)
    computer.send(1)
    while not computer.halted:
        computer.run()
    result = 0
    while result == 0:
        result = computer.read()
    return result


def solution2(data: List[int]) -> int:
    computer = IntcodeComputer(data)
    computer.send(5)
    while not computer.halted:
        computer.run()
    return computer.read()


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().split(",")))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
