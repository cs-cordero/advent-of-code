from __future__ import annotations

from typing import List

from intcode import IntcodeComputer


def solution1(data: List[int]) -> int:
    computer = IntcodeComputer(data)
    instructions = [
        "NOT C T\n",
        "NOT A J\n",
        "OR T J\n",
        "AND D J\n",
    ]
    for instruction in instructions:
        computer.send_long(instruction)
    computer.send_long("WALK\n")
    computer.run_until_blocked()
    while computer.has_output():
        value = computer.read()
        try:
            print(chr(value), end="")
        except ValueError:
            return value


def solution2(data: List[int]) -> int:
    computer = IntcodeComputer(data)
    instructions = [
        "NOT C J\n",
        "AND D J\n",
        "NOT H T\n",
        "NOT T T\n",
        "OR E T\n",
        "AND T J\n",
        "NOT A T\n",
        "OR T J\n",
        "NOT B T\n",
        "NOT T T\n",
        "OR E T\n",
        "NOT T T\n",
        "OR T J\n",
    ]
    for instruction in instructions:
        computer.send_long(instruction)
    computer.send_long("RUN\n")
    computer.run_until_blocked()
    while computer.has_output():
        value = computer.read()
        try:
            print(chr(value), end="")
        except ValueError:
            return value


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().split(",")))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
