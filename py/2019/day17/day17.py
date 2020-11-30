from __future__ import annotations

from collections import defaultdict, deque
from itertools import chain
from typing import Dict, List, Tuple

from intcode import IntcodeComputer


def solution1(data: List[int]) -> int:
    data = data[:]
    graph = create_graph(data)

    # Find starting point
    for starting_point, representation in graph.items():
        if representation in ("<", ">", "^", "v"):
            break

    intersections = 0
    visited = {starting_point}
    queue = deque([starting_point])
    direction = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    while queue:
        row, col = queue.popleft()
        if all(
            graph.get((row + drow, col + dcol), ".") == "#" for drow, dcol in direction
        ):
            intersections += row * col

        for drow, dcol in direction:
            next_point = row + drow, col + dcol
            if next_point in visited:
                continue
            visited.add(next_point)
            if graph.get(next_point) == "#":
                queue.append(next_point)
    return intersections


def solution2(data: List[int]) -> int:
    orig_data = data[:]
    graph = create_graph(data)

    for current_location, representation in graph.items():
        if representation in ("<", ">", "^", "v"):
            break

    current_row, current_col = current_location
    delta_row, delta_col = -1, 0

    left_map = {
        (-1, 0): (0, -1),
        (0, -1): (1, 0),
        (1, 0): (0, 1),
        (0, 1): (-1, 0),
    }
    right_map = {
        (-1, 0): (0, 1),
        (0, 1): (1, 0),
        (1, 0): (0, -1),
        (0, -1): (-1, 0),
    }

    path = []
    while True:
        steps_forward = 0
        while True:
            next_location = current_row + delta_row, current_col + delta_col
            if not graph.get(next_location) == "#":
                break
            steps_forward += 1
            current_row, current_col = next_location

        if steps_forward:
            last_turn = path.pop()
            path.append(f"{last_turn}{steps_forward}")

        left_row, left_col = left_map[(delta_row, delta_col)]
        right_row, right_col = right_map[(delta_row, delta_col)]

        left_location = graph.get((current_row + left_row, current_col + left_col))
        right_location = graph.get((current_row + right_row, current_col + right_col))

        if left_location == "#" and right_location == "#":
            assert False, "Should not happen..."
        elif left_location == "#":
            path.append("L")
            delta_row, delta_col = left_row, left_col
        elif right_location == "#":
            path.append("R")
            delta_row, delta_col = right_row, right_col
        else:
            break

    routine, a, b, c = find_groups(path)
    data = orig_data[:]
    data[0] = 2

    computer = IntcodeComputer(data)
    for char in chain(routine, a, b, c):
        computer.send(ord(char))
    computer.send(ord("n"))
    computer.send(ord("\n"))
    computer.run_until_blocked()
    result = None
    while computer.has_output():
        result = computer.read()
    return result


def find_groups(path: List[str]) -> List[str]:
    # Group finding done manually by inspecting the path
    groups = {
        "A": ["L6", "R8", "R10", "L6", "L6"],
        "B": ["L4", "R8", "L6", "L10"],
        "C": ["L4", "L4", "L10"],
    }
    ordering = []
    while path:
        for name, group in groups.items():
            if path[: len(group)] == group:
                path = path[len(group) :]
                ordering.append(name)

    def deconstruct(p):
        return ",".join(f"{a},{''.join(b)}" for a, *b in p) + "\n"

    return [
        ",".join(ordering) + "\n",
        deconstruct(groups["A"]),
        deconstruct(groups["B"]),
        deconstruct(groups["C"]),
    ]


def create_graph(data: List[int]) -> Dict[Tuple[int, int], str]:
    computer = IntcodeComputer(data)
    computer.run_until_blocked()

    graph = defaultdict(str)

    row = 0
    col = 0
    while computer.stdout:
        value = computer.read()
        if value == 10:  # Newline
            col = 0
            row += 1
            continue

        graph[(row, col)] = chr(value)
        col += 1

    return graph


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().split(",")))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
