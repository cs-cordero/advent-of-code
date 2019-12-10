from heapq import heappush, heappop
from math import atan, degrees
from collections import defaultdict
from typing import Dict, Generator, List, Tuple


def solution1(data: List[int]) -> int:
    best = 0
    asteroid_vantage_points = get_asteroid_vantage_points(data)
    for _, viewable in asteroid_vantage_points.items():
        best = max(best, len(viewable))
    return best


def solution2(data: List[int]) -> object:
    best_vantage_point = None
    observable = 0
    asteroid_vantage_points = get_asteroid_vantage_points(data)
    for coordinate, viewable in asteroid_vantage_points.items():
        if len(viewable) > observable:
            observable = len(viewable)
            best_vantage_point = coordinate

    targets_by_angle = asteroid_vantage_points[best_vantage_point]
    degrees = list(sorted(targets_by_angle.keys()))

    degree_i = 0
    destroyed = 0
    while True:
        degree = degrees[degree_i]
        targets = targets_by_angle[degree]
        if targets:
            _, asteroid_shot = heappop(targets_by_angle[degree])
            destroyed += 1
            if destroyed == 200:
                row, col = asteroid_shot
                return col * 100 + row
        degree_i += 1
        degree_i %= len(degrees)


def get_asteroid_vantage_points(
    data: List[int]
) -> Dict[Tuple[int, int], Dict[float, list]]:
    asteroids = []
    for row_i, row in enumerate(data):
        for col_i, value in enumerate(row):
            if value == "#":
                asteroids.append((row_i, col_i))

    asteroid_vantage_points = defaultdict(lambda: defaultdict(list))
    for base_row, base_col in asteroids:
        for cand_row, cand_col in asteroids:
            if (base_row, base_col) == (cand_row, cand_col):
                continue

            opposite = cand_row - base_row
            adjacent = cand_col - base_col
            if opposite != 0 and adjacent != 0:
                local_angle = abs(round(degrees(atan(opposite / adjacent)), 16))
                if opposite < 0 and adjacent > 0:
                    # Quadrant 1
                    angle = 90 - local_angle
                elif opposite > 0 and adjacent > 0:
                    # Quadrant 2
                    angle = 90 + local_angle
                elif opposite > 0 and adjacent < 0:
                    # Quadrant 3
                    angle = 270.0 - local_angle
                else:
                    # Quadrant 4
                    angle = 270.0 + local_angle
            elif not opposite:
                angle = 90.0 if cand_col > base_col else 270.0
            elif not adjacent:
                angle = 180.0 if cand_row > base_row else 0

            distance = abs(cand_col - base_row) + abs(cand_row - base_row)
            heappush(
                asteroid_vantage_points[(base_row, base_col)][angle],
                (distance, (cand_row, cand_col)),
            )
    return asteroid_vantage_points


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = [line.strip() for line in f.readlines()]
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
