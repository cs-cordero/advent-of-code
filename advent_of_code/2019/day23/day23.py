from __future__ import annotations

from collections import deque
from typing import List

from intcode import IntcodeComputer


def solution1(data: List[int]) -> int:
    computers = [IntcodeComputer(data) for _ in range(50)]
    for i, computer in enumerate(computers):
        computer.send(i)
        computer.run_until_blocked()
        assert not computer.halted

    while True:
        # Scan for output first
        for i, computer in enumerate(computers):
            if computer.has_output():
                target = computer.read()
                x = computer.read()
                y = computer.read()
                if target == 255:
                    return y

                computers[target].send(x)
                computers[target].send(y)

        # Pass -1 to computers without input
        for i, computer in enumerate(computers):
            if not computer.has_input():
                computer.send(-1)

        # Run
        for i, computer in enumerate(computers):
            computer.run_until_blocked()


def solution2(data: List[int]) -> int:
    computers = [IntcodeComputer(data) for _ in range(50)]
    for i, computer in enumerate(computers):
        computer.send(i)
        computer.run_until_blocked()
        assert not computer.halted

    last_nat = None
    nat = None
    while True:
        # Scan for output first
        for i, computer in enumerate(computers):
            if computer.has_output():
                target = computer.read()
                x = computer.read()
                y = computer.read()
                if target == 255:
                    nat = (x, y)
                else:
                    computers[target].send(x)
                    computers[target].send(y)

        # Scan for idleness:
        network_is_idle = True
        for i, computer in enumerate(computers):
            if not computer.halted and not computer.needs_input:
                network_is_idle = False
                break
        if network_is_idle and nat:
            # print(f"Network is idle, sending {nat=}")
            x, y = nat
            if last_nat == y:
                return y
            last_nat = y
            computers[0].send(x)
            computers[0].send(y)

        # Pass -1 to computers without input
        for i, computer in enumerate(computers):
            if not computer.has_input():
                computer.send(-1)

        # Run
        for i, computer in enumerate(computers):
            computer.run_until_blocked()


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().split(",")))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
