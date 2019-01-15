from copy import deepcopy
from typing import NamedTuple, Iterable, Dict, Tuple
import os
import sys


TEST_DATA = [
    '.#.#.#',
    '...##.',
    '#....#',
    '..#...',
    '#.#..#',
    '####..',
]


def read_file(filepath):
    if 'test' not in sys.argv:
        with open(filepath) as f:
            for line in f.readlines():
                yield line.rstrip()
    else:
        for data in TEST_DATA:
            yield data


class Point2D(NamedTuple):
    x: int
    y: int

    def __add__(self, other: 'Point2D') -> 'Point2D':
        x1, y1 = self
        x2, y2 = other
        return Point2D(x1+x2, y1+y2)

    @property
    def neighboring_points(self) -> Iterable['Point2D']:
        deltas = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0), (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ]
        for delta in deltas:
            yield self + Point2D(*delta)


class Grid:
    def __init__(self, grid: Dict[Point2D, bool]) -> None:
        self.grid = grid
        xs, ys = zip(*grid.keys())
        self.minx = min(xs)
        self.maxx = max(xs)
        self.miny = min(ys)
        self.maxy = max(ys)

    @staticmethod
    def populate_from_input() -> 'Grid':
        grid = {}
        for x, row in enumerate(read_file('aoc_day_18_input.txt')):
            for y, col in enumerate(row):
                grid[Point2D(x, y)] = col == '#'
        return Grid(grid)

    def __repr__(self) -> str:
        stringified = [
            ''.join('#' if self.grid[Point2D(row, col)] else '.'
                    for col in range(self.maxx+1))
            for row in range(self.maxy+1)
        ]
        return '\n'.join(stringified)

    def turn_corners_on(self) -> None:
        self.grid[Point2D(self.minx, self.miny)] = True
        self.grid[Point2D(self.minx, self.maxy)] = True
        self.grid[Point2D(self.maxx, self.miny)] = True
        self.grid[Point2D(self.maxx, self.maxy)] = True


    def play(self, part2: bool = False) -> None:
        old_grid = deepcopy(self.grid)

        for row in range(self.maxy+1):
            for col in range(self.maxx+1):
                on_neighbors = 0
                current_point = Point2D(row, col)
                for neighbor in current_point.neighboring_points:
                    if old_grid.get(neighbor, False):
                        on_neighbors += 1
                if self.grid[current_point]:
                    self.grid[current_point] = on_neighbors in (2, 3)
                else:
                    self.grid[current_point] = on_neighbors == 3

        if part2:
            self.turn_corners_on()

        if 'verbose' in sys.argv:
            os.system('clear')
            print(self)


def solution() -> Tuple[int, int]:
    grid = Grid.populate_from_input()
    for i in range(100):
        grid.play()
    part1 = sum(light for light in grid.grid.values())

    grid2 = Grid.populate_from_input()
    grid2.turn_corners_on()
    for i in range(100):
        grid2.play(part2=True)
    part2 = sum(light for light in grid2.grid.values())
    return part1, part2


print(solution())
