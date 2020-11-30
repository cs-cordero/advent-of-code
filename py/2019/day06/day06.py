from collections import defaultdict
from typing import List


def solution1(data: List[int]) -> int:
    graph = defaultdict(set)
    for center, orbital in data:
        graph[center].add(orbital)

    count = 0
    visited = {"COM"}

    def dfs_helper(node: str, depth: int) -> None:
        nonlocal count
        count += depth
        for orbital in graph[node]:
            if orbital in visited:
                continue
            visited.add(orbital)
            dfs_helper(orbital, depth + 1)

    dfs_helper("COM", 0)
    return count


def solution2(data: List[int]) -> int:
    graph = defaultdict(str)
    for parent, orbital in data:
        graph[orbital] = parent

    def find_depth(node: str) -> int:
        count = 0
        while node != "COM":
            node = graph[node]
            count += 1
        return count

    pointer_san = graph["SAN"]
    pointer_you = graph["YOU"]
    depth_san = find_depth(pointer_san)
    depth_you = find_depth(pointer_you)

    distance = 0

    while depth_san > depth_you:
        pointer_san = graph[pointer_san]
        depth_san -= 1
        distance += 1

    while depth_you > depth_san:
        pointer_you = graph[pointer_you]
        depth_you -= 1
        distance += 1

    while pointer_san != pointer_you:
        pointer_san = graph[pointer_san]
        pointer_you = graph[pointer_you]
        distance += 2

    return distance


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = [line.strip().split(")") for line in f.readlines()]
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
