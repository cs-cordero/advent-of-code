from functools import reduce
from itertools import combinations
from typing import List, Set, Tuple


def solution1(data: List[int]) -> int:
    return get_quantum_entanglement(data, 2)


def solution2(data: List[int]) -> int:
    return get_quantum_entanglement(data, 3)


def get_quantum_entanglement(data: List[int], group_count: int) -> int:
    groups = []
    weights = set(data)
    for i in range(2, len(data)):
        weights_copy = weights.copy()
        for group in combinations(data, i):
            weights_copy -= set(group)
            if can_make_two_groups(tuple(weights_copy), sum(group), group_count):
                groups.append(group)
            weights_copy |= set(group)

        if not groups:
            continue

        # Answer lies in groups
        best_qe = float("inf")
        for group in groups:
            qe = reduce(lambda x, y: x * y, group)
            if qe < best_qe:
                best_qe = min(best_qe, qe)
        return best_qe


def can_make_two_groups(
    remaining_weights: Tuple[int], target: int, groups: int
) -> bool:
    if sum(remaining_weights) != groups * target:
        # Not a sufficient condition, but a good early escape hatch.
        return False

    memo = {}

    def backtracker(weights: Set[int], current: int = 0) -> bool:
        memo_key = current
        if memo_key in memo:
            return memo[memo_key]

        if current == target:
            memo[memo_key] = True
            return memo[memo_key]

        weight_copy = weights.copy()
        for weight in weights:
            weight_copy.remove(weight)
            if backtracker(weight_copy, current + weight):
                memo[memo_key] = True
                return memo[memo_key]
            weight_copy.add(weight)

        memo[memo_key] = False
        return memo[memo_key]

    return backtracker(set(remaining_weights))


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readlines()))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
