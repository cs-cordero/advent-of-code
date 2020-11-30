from collections import defaultdict
from dataclasses import dataclass
from typing import Dict, Iterator, NamedTuple, Set


@dataclass(frozen=True, order=True)
class Point:
    row: int
    col: int

    def __iter__(self) -> Iterator[int]:
        return iter([self.row, self.col])

    def is_valid(self) -> bool:
        return 0 <= self.row and self.row < 5 and 0 <= self.col and self.col < 5


class DimensionalPoint(NamedTuple):
    dimension: int
    point: Point

    def __iter__(self) -> Iterator[int]:
        return iter([self.dimension, self.point])


def solution1(data: str) -> str:
    seen = set()
    while data not in seen:
        seen.add(data)
        data = play(data)
    return str(get_biodiversity(data))


def play(spaces: str) -> str:
    result = []
    for i, value in enumerate(spaces):
        adj_bug_count = sum(spaces[j] == "#" for j in get_nearby_indexes(i))
        if value == "#":
            result.append("#" if adj_bug_count == 1 else ".")
        else:
            result.append("#" if adj_bug_count in (1, 2) else ".")
    return "".join(result)


def get_nearby_indexes(base_index: int) -> Iterator[int]:
    base_point = index_to_coordinate(base_index)
    for dr, dc in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
        adj_point = Point(base_point.row + dr, base_point.col + dc)
        if not adj_point.is_valid():
            continue
        yield coordinate_to_index(adj_point)


def get_biodiversity(spaces: str) -> int:
    result = 0
    current = 1
    for space in spaces:
        if space == "#":
            result += current
        current *= 2
    return result


def solution2(data: str) -> str:
    points = parse_to_dimensional_points(data)
    for _ in range(200):
        points = play_recursive(points)
    return len(points)


def play_recursive(bugs: Set[DimensionalPoint]) -> Set[DimensionalPoint]:
    adjacents: Dict[DimensionalPoint, int] = defaultdict(int)
    for bug in bugs:
        for adj_space in get_nearby_indexes_recursive(bug):
            adjacents[adj_space] += 1

    result: Set[DimensionalPoint] = set()
    for point in bugs | set(adjacents):
        if (point in bugs and adjacents[point] == 1) or (
            point not in bugs and adjacents[point] in (1, 2)
        ):
            result.add(point)
    return result


def get_nearby_indexes_recursive(
    point: DimensionalPoint,
) -> Iterator[DimensionalPoint]:
    dimension, base_point = point
    for dr, dc in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
        adj_point = Point(base_point.row + dr, base_point.col + dc)
        if adj_point.is_valid() and adj_point != Point(2, 2):
            yield DimensionalPoint(dimension, adj_point)
        elif adj_point.row == -1:
            yield DimensionalPoint(dimension - 1, Point(1, 2))
        elif adj_point.row == 5:
            yield DimensionalPoint(dimension - 1, Point(3, 2))
        elif adj_point.col == -1:
            yield DimensionalPoint(dimension - 1, Point(2, 1))
        elif adj_point.col == 5:
            yield DimensionalPoint(dimension - 1, Point(2, 3))
        else:
            assert adj_point == Point(2, 2)
            if base_point == Point(1, 2):
                points = [Point(0, i) for i in range(5)]
            elif base_point == Point(2, 3):
                points = [Point(i, 4) for i in range(5)]
            elif base_point == Point(3, 2):
                points = [Point(4, i) for i in range(5)]
            elif base_point == Point(2, 1):
                points = [Point(i, 0) for i in range(5)]

            for point in points:
                yield DimensionalPoint(dimension + 1, point)


def index_to_coordinate(index: int) -> Point:
    row = index // 5
    col = index % 5
    return Point(row, col)


def coordinate_to_index(point: Point) -> int:
    return point.row * 5 + point.col


def parse_to_dimensional_points(data: str) -> Set[DimensionalPoint]:
    return {
        DimensionalPoint(0, index_to_coordinate(i))
        for i, val in enumerate(data)
        if val == "#"
    }


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = "".join([line.strip() for line in f.readlines()])
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
