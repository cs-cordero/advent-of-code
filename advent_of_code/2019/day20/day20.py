from collections import defaultdict, deque
from dataclasses import dataclass
from heapq import heappop, heappush
from typing import Dict, List, Tuple

Point = Tuple[int, int]


@dataclass(unsafe_hash=True)
class Portal:
    name: str
    recursion_delta: int


def solution1(data: List[str]) -> str:
    matrix = create_matrix(data)
    graph = create_graph(matrix)

    # dijkstra
    dist = defaultdict(lambda: float("inf"))
    dist["AA"] = 0
    heap = [(0, "AA")]
    while heap:
        steps, current = heappop(heap)
        if steps > dist[current]:
            continue

        for neighbor, required in graph[current].items():
            next_steps = steps + required
            if neighbor != "ZZ":
                next_steps += 1  # it costs 1 step to use the warp
            if next_steps < dist[neighbor]:
                dist[neighbor] = next_steps
                heappush(heap, (next_steps, neighbor))
    return dist["ZZ"]


def solution2(data: List[str]) -> str:
    matrix = create_matrix(data)
    graph = create_recursive_graph(matrix)
    portals = {(portal.name, portal.recursion_delta): portal for portal in graph.keys()}

    # bfs
    queue = deque([(0, 0, portals[("AA", -1)])])
    while queue:
        steps, level, portal = queue.popleft()

        for neighbor, required in graph[portal].items():
            next_steps = steps + required
            if neighbor.name == "ZZ" and level == 0:
                return next_steps

            next_level = level + neighbor.recursion_delta

            if (
                neighbor.name == "AA"
                or (neighbor.name == "ZZ" and level != 0)
                or next_level < 0
                or next_level > len(graph)
            ):
                continue

            if neighbor.name != "ZZ":
                next_steps += 1  # it costs 1 step to use the warp

            mirror = portals[(neighbor.name, neighbor.recursion_delta * -1)]
            queue.append((next_steps, next_level, mirror))


def create_graph(matrix: List[List[str]]) -> Dict[str, Dict[str, int]]:
    graph = {}
    for row_i, row in enumerate(matrix):
        for col_i, value in enumerate(row):
            if isinstance(value, Portal):
                if value.name not in graph:
                    graph[value.name] = {}
                graph[value.name].update(bfs(matrix, (row_i, col_i)))
    return graph


def create_recursive_graph(matrix: List[List[str]]) -> Dict[str, Dict[str, int]]:
    graph = {}
    for row_i, row in enumerate(matrix):
        for col_i, value in enumerate(row):
            if isinstance(value, Portal):
                graph[value] = bfs(matrix, (row_i, col_i), use_name=False)
    return graph


def bfs(matrix: List[List[str]], start: Point, use_name: bool = True) -> Dict[str, int]:
    visited = {start}
    queue = deque([(*start, 0)])
    edges = {}
    while queue:
        row, col, steps = queue.popleft()
        if isinstance(matrix[row][col], Portal) and (row, col) != start:
            if use_name:
                edges[matrix[row][col].name] = steps
            else:
                edges[matrix[row][col]] = steps
            continue

        for next_row, next_col in [
            (row - 1, col),
            (row, col - 1),
            (row + 1, col),
            (row, col + 1),
        ]:
            if (
                next_row < 0
                or next_col < 0
                or next_row >= len(matrix)
                or next_col >= len(matrix[next_row])
                or (next_row, next_col) in visited
                or matrix[next_row][next_col] == "#"
            ):
                continue
            visited.add((next_row, next_col))
            queue.append((next_row, next_col, steps + 1))
    return edges


def create_matrix(data: List[str]) -> List[List[str]]:
    max_row_length = len(max(data, key=lambda x: len(x)))

    graph = [
        [char if char in (".", "#") else "#" for char in row[2 : max_row_length - 2]]
        for row in data[2 : len(data) - 2]
    ]

    def inbounds(row: int, col: int) -> bool:
        return row >= 0 and col >= 0 and row < len(data) and col < len(data[row])

    for row_i, row in enumerate(data):
        for col_i, value in enumerate(row):
            if value == ".":
                portal_name = None
                if inbounds(row_i - 1, col_i) and data[row_i - 1][col_i].isalpha():
                    portal_name = f"{data[row_i-2][col_i]}{data[row_i-1][col_i]}"
                    if not inbounds(row_i - 3, col_i):  # topmost row
                        recursion_delta = -1
                    else:
                        recursion_delta = 1
                elif inbounds(row_i + 1, col_i) and data[row_i + 1][col_i].isalpha():
                    portal_name = f"{data[row_i+1][col_i]}{data[row_i+2][col_i]}"
                    if not inbounds(row_i + 3, col_i):  # bottommost row
                        recursion_delta = -1
                    else:
                        recursion_delta = 1
                elif inbounds(row_i, col_i - 1) and data[row_i][col_i - 1].isalpha():
                    portal_name = f"{data[row_i][col_i-2]}{data[row_i][col_i-1]}"
                    if not inbounds(row_i, col_i - 3):  # leftmost row
                        recursion_delta = -1
                    else:
                        recursion_delta = 1
                elif inbounds(row_i, col_i + 1) and data[row_i][col_i + 1].isalpha():
                    portal_name = f"{data[row_i][col_i+1]}{data[row_i][col_i+2]}"
                    if not inbounds(row_i, col_i + 3):  # rightmost row
                        recursion_delta = -1
                    else:
                        recursion_delta = 1

                if portal_name:
                    graph[row_i - 2][col_i - 2] = Portal(
                        name=portal_name, recursion_delta=recursion_delta
                    )
    return graph


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = [line.replace("\n", "") for line in f.readlines()]
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
