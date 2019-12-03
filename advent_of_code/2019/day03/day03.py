from typing import Dict, List, Tuple


def get_visited_points(movements: List[Tuple[str, int]]) -> Dict[Tuple[int, int], int]:
    visited = {}
    direction_map = {"U": (-1, 0), "D": (1, 0), "R": (0, 1), "L": (0, -1)}
    x, y = (0, 0)
    steps = 0
    for direction, movement in movements:
        dx, dy = direction_map[direction]
        for _ in range(movement):
            steps += 1
            x += dx
            y += dy
            visited.setdefault((x, y), steps)
    return visited


def solution1(data: List[List[Tuple[str, int]]]) -> int:
    wire1_visited = get_visited_points(data[0])
    wire2_visited = get_visited_points(data[1])
    shortest_distance = float("inf")
    for intersection in set(wire1_visited) & set(wire2_visited):
        shortest_distance = min(
            abs(intersection[0]) + abs(intersection[1]), shortest_distance
        )
    return shortest_distance


def solution2(data: List[List[Tuple[str, int]]]) -> int:
    wire1_visited = get_visited_points(data[0])
    wire2_visited = get_visited_points(data[1])
    shortest_steps = float("inf")
    for intersection in set(wire1_visited) & set(wire2_visited):
        shortest_steps = min(
            shortest_steps, wire1_visited[intersection] + wire2_visited[intersection]
        )
    return shortest_steps


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = [
            [(move[0], int(move[1:])) for move in line.split(",")]
            for line in f.readlines()
        ]
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
