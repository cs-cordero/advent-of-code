from typing import List


def solution1(data: List[int], noun=12, verb=2) -> int:
    pos = {i: value for i, value in enumerate(data)}
    pos[1] = noun
    pos[2] = verb

    i = 0
    while pos[i] != 99:
        j, k = pos[pos[i + 1]], pos[pos[i + 2]]
        pos[pos[i + 3]] = j + k if pos[i] == 1 else j * k
        i += 4
    return pos[0]


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
