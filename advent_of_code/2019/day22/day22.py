from collections import defaultdict, deque
from dataclasses import dataclass
from heapq import heappop, heappush
from typing import Dict, List, Tuple


def solution1(data: List[str]) -> str:
    # https://codeforces.com/blog/entry/72593
    a = 1
    b = 0
    m = 10007
    for instruction in data:
        if instruction == "deal into new stack":
            a *= -1
            b *= -1
            b -= 1
        elif "cut" in instruction:
            amount = int(instruction.split(" ")[1])
            b -= amount
        elif "deal with increment" in instruction:
            amount = int(instruction.split(" ")[-1])
            a *= amount
            b *= amount

        a %= m
        b %= m

    return (a * 2019 + b) % 10007


def solution2(data: List[str]) -> str:
    # https://codeforces.com/blog/entry/72593
    a = 1
    b = 0
    m = 119315717514047
    k = 101741582076661
    for instruction in data:
        if instruction == "deal into new stack":
            a *= -1
            b *= -1
            b -= 1
        elif "cut" in instruction:
            amount = int(instruction.split(" ")[1])
            b -= amount
        elif "deal with increment" in instruction:
            amount = int(instruction.split(" ")[-1])
            a *= amount
            b *= amount

        a %= m
        b %= m

    # Transform into F_k(x), since it forms a geometric series
    b = b * (1 - pow(a, k, m)) * pow(1 - a, -1, m)
    a = pow(a, k, m)
    return ((2020 - b) * pow(a, -1, m)) % m


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = [line.strip() for line in f.readlines()]
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
