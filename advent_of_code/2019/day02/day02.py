from typing import List

from intcode import IntcodeComputer


def solution1(data: List[int], noun=12, verb=2) -> int:
    computer = IntcodeComputer(data)
    computer.registers[1] = noun
    computer.registers[2] = verb
    while not computer.halted:
        computer.run()
    return computer.registers[0]


def solution2(data: List[int]) -> int:
    target = 19690720
    for noun in range(100):
        for verb in range(100):
            result = solution1(data, noun=noun, verb=verb)
            if result == target:
                return 100 * noun + verb


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().split(",")))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
