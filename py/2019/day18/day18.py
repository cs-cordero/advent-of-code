from collections import deque
from dataclasses import dataclass, field
from typing import Dict, Iterable, List, Set, Tuple

Point = Tuple[int, int]


@dataclass
class Key:
    row: int
    col: int
    required: Set[str] = field(default_factory=set)


def solution1(data: List[List[str]]) -> str:
    start = get_start_location(data)
    keys = get_key_requirements(data, start)
    distances = {
        key_name: get_distances((key.row, key.col), data)
        for key_name, key in keys.items()
    }
    distances["@"] = get_distances(start, data)
    all_keys = frozenset(keys)

    memo = {}

    def dfs(current: str, remaining: Set[str]) -> int:
        memo_key = (current, frozenset(remaining))
        if not remaining:
            memo[memo_key] = 0
        elif memo_key not in memo:
            collected = all_keys - remaining

            min_steps = float("inf")
            for key_name, key in keys.items():
                if key_name in collected or key.required - collected:
                    continue

                min_steps = min(
                    min_steps,
                    distances[current][key_name]
                    + dfs(key_name, remaining - {key_name}),
                )
            memo[memo_key] = min_steps
        return memo[memo_key]

    return dfs("@", all_keys)


def solution2(data: List[List[str]]) -> str:
    start = get_start_location(data)
    keys = get_key_requirements(data, start)
    all_keys = frozenset(keys)

    original_row, original_col = start
    data[original_row - 1][original_col - 1] = "@"
    data[original_row - 1][original_col + 1] = "@"
    data[original_row + 1][original_col - 1] = "@"
    data[original_row + 1][original_col + 1] = "@"
    data[original_row][original_col - 1] = "#"
    data[original_row][original_col] = "#"
    data[original_row][original_col + 1] = "#"
    data[original_row - 1][original_col] = "#"
    data[original_row + 1][original_col] = "#"

    distances = {
        key_name: get_distances((key.row, key.col), data)
        for key_name, key in keys.items()
    }
    distances["@0"] = get_distances((original_row - 1, original_col - 1), data)
    distances["@1"] = get_distances((original_row - 1, original_col + 1), data)
    distances["@2"] = get_distances((original_row + 1, original_col - 1), data)
    distances["@3"] = get_distances((original_row + 1, original_col + 1), data)

    memo = {}

    def dfs(current: Tuple[str], remaining: Set[str]) -> int:
        memo_key = (*current, frozenset(remaining))
        if not remaining:
            memo[memo_key] = 0
        elif memo_key not in memo:
            collected = all_keys - remaining

            min_steps = float("inf")
            for key_name, key in keys.items():
                if key_name in collected or key.required - collected:
                    continue

                key_location = (key.row, key.col)
                robot_i = get_robot_number(key_location, start)
                robot = current[robot_i]
                next_current = (*current[:robot_i], key_name, *current[robot_i + 1 :])

                min_steps = min(
                    min_steps,
                    distances[robot][key_name]
                    + dfs(next_current, remaining - {key_name}),
                )
            memo[memo_key] = min_steps
        return memo[memo_key]

    return dfs(("@0", "@1", "@2", "@3"), all_keys)


def get_robot_number(point: Point, center: Point) -> int:
    row, col = point
    crow, ccol = center
    if row < crow and col < ccol:
        return 0
    elif row < crow and col > ccol:
        return 1
    elif row > crow and col < ccol:
        return 2
    elif row > crow and col > ccol:
        return 3
    else:
        assert False, "Invariant"


def get_start_location(graph: List[List[str]]) -> Point:
    for row_i, row in enumerate(graph):
        for col_i, value in enumerate(row):
            if value == "@":
                return (row_i, col_i)


def get_key_requirements(graph: List[List[str]], start: Point) -> Dict[str, Key]:
    visited = {start}

    keys = {}

    queue = deque([(*start, set())])
    while queue:
        row, col, _required = queue.popleft()
        required = _required.copy()

        value = graph[row][col]
        if value.isalpha():
            if value.isupper():
                required.add(value.lower())
            else:
                keys[value] = Key(row=row, col=col, required=required.copy())

        for neighbor in get_neighbors((row, col), graph):
            if neighbor in visited:
                continue
            visited.add(neighbor)
            queue.append((*neighbor, required))
    return keys


def get_distances(point: Point, graph: List[List[str]]) -> Dict[str, int]:
    queue = deque([(*point, 0)])
    visited = {point}
    distances = {}
    while queue:
        row, col, steps = queue.popleft()
        value = graph[row][col]
        if value.isalpha() and value.islower():
            distances[value] = steps
        for neighbor in get_neighbors((row, col), graph):
            if neighbor in visited:
                continue
            visited.add(neighbor)
            queue.append((*neighbor, steps + 1))
    return distances


def get_neighbors(point: Point, graph: List[List[str]]) -> Iterable[Point]:
    row, col = point
    row_lim, col_lim = len(graph), len(graph[0])

    for drow, dcol in [(-1, 0), (0, -1), (1, 0), (0, 1)]:
        next_row, next_col = row + drow, col + dcol
        if next_row < 0 or next_row >= row_lim or next_col < 0 or next_col >= col_lim:
            continue
        if graph[next_row][next_col] == "#":
            continue
        yield (next_row, next_col)


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = [[char for char in line.strip()] for line in f.readlines()]
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
