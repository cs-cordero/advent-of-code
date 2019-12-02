from typing import List


def solution1(data: List[int]) -> int:
    return sum((mass // 3) - 2 for mass in data)


def solution2(data: List[int]) -> int:
    dp = {0: 0, -1: 0, -2: 0}
    total = 0
    for mass in data:
        if mass in dp:
            total += dp[mass]
            continue
        current_mass = mass
        while current_mass not in dp:
            current_mass //= 3
            current_mass -= 2
            total += max(current_mass, 0)
        total += dp[current_mass]
    return total


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readlines()))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
