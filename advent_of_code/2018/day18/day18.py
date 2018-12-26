from collections.abc import MutableMapping
from copy import deepcopy
from functools import wraps
from typing import NamedTuple
import enum
import os


TEST_DATA = """
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
"""


def read_file(filepath, use_test_data = False):
    if not use_test_data:
        with open(filepath) as f:
            for line in f.readlines():
                yield line.strip()
    else:
        for line in TEST_DATA.strip().split('\n'):
            yield line.strip()


class PointType(enum.Enum):
    GROUND = '.'
    TREES = '|'
    LUMBERYARD = '#'

    def __repr__(self):
        return self.value

    def __str__(self):
        return self.value


class Grid(MutableMapping):
    class Point(NamedTuple):
        x: int
        y: int

        def __add__(self, other):
            x1, y1 = self
            x2, y2 = other
            return Grid.Point(x1 + x2, y1 + y2)

        def __repr__(self):
            return f'({self.x}, {self.y})'

        def get_adjacent_points(self, *, minx = None, maxx = None,
                                miny = None, maxy = None):
            def generator():
                for delta in ((0, -1), (-1, 0), (1, 0), (0, 1),
                              (-1, -1), (1, -1), (-1, 1), (1, 1)):
                    yield self + delta

            return [
                point for point in generator()
                if (minx is None or point.x >= minx)
                and (maxx is None or point.x <= maxx)
                and (miny is None or point.y >= miny)
                and (maxy is None or point.y <= maxy)
            ]

    class Limit(NamedTuple):
        minx: int
        maxx: int
        miny: int
        maxy: int

    def __init__(self, *args, **kwargs):
        assert not args and not kwargs, "Grid does not accept init arguments!"
        self._storage = {}

    def _impose_key_restrictions(fn):
        @wraps(fn)
        def wrapper(self, *args, **kwargs):
            try:
                key = kwargs.get('key', args[0])
            except KeyError:
                raise Exception("Expected a tuple key!")

            assert (isinstance(key, Grid.Point) or (
                isinstance(key, tuple)
                and len(key) == 2
                and all(isinstance(value, int) for value in key)
            ))

            return fn(self, *args, **kwargs)
        return wrapper

    @_impose_key_restrictions
    def __getitem__(self, key):
        return self._storage[key]

    @_impose_key_restrictions
    def __setitem__(self, key, value):
        if not isinstance(key, Grid.Point):
            key = Grid.Point(*key)
        self._storage[key] = value

    @_impose_key_restrictions
    def __delitem__(self, key):
        del self._storage[key]

    def __iter__(self):
        return iter(self._storage)

    def __len__(self):
        return len(self._storage)

    def __repr__(self):
        return str(self._storage)

    @property
    def limits(self):
        assert len(self) > 0, "No points to create a limit!"
        all_x, all_y = zip(*self)
        return Grid.Limit(
            minx=min(all_x),
            maxx=max(all_x),
            miny=min(all_y),
            maxy=max(all_y),
        )

    def copy(self):
        return deepcopy(self)

    def show(self):
        limits = self.limits
        for row in range(limits.miny, limits.maxy+1):
            _row = ''
            for col in range(limits.minx, limits.maxx+1):
                _row += str(self[(col, row)])
            print(_row)



def solution(use_test_data = False):
    grid = Grid()
    input_mapping = {
        '.': PointType.GROUND,
        '|': PointType.TREES,
        '#': PointType.LUMBERYARD,
    }
    for row, line in enumerate(read_file('input.txt', use_test_data)):
        for col, value in enumerate(line):
            grid[(col, row)] = input_mapping[value]

    grid_limits = grid.limits._asdict()
    patterns = []
    cycle_found = None
    for minute in range(1000000000):
        next_grid = grid.copy()
        for point, point_type in next_grid.items():
            old_adjacent_point_types = [
                grid[adjacent_point]
                for adjacent_point in point.get_adjacent_points(**grid_limits)
            ]
            if point_type is PointType.GROUND:
                if old_adjacent_point_types.count(PointType.TREES) >= 3:
                    next_grid[point] = PointType.TREES
            elif point_type is PointType.TREES:
                if old_adjacent_point_types.count(PointType.LUMBERYARD) >= 3:
                    next_grid[point] = PointType.LUMBERYARD
            elif point_type is PointType.LUMBERYARD:
                if not (old_adjacent_point_types.count(PointType.LUMBERYARD) >= 1
                        and old_adjacent_point_types.count(PointType.TREES) >= 1):
                    next_grid[point] = PointType.GROUND
        grid = next_grid

        # PART TWO
        #
        # Finding if a cycle exists
        # stringified = ''.join(str(x[1]) for x in sorted(grid.items()))
        # if not cycle_found:
        #     print(f'Minute: {minute}, Score: {len(stringified) - stringified.count(".")}')
        #     if stringified in patterns:
        #         cycle_found = (stringified, minute)
        #         import pdb; pdb.set_trace()
        #     patterns.append(stringified)
        # else:
        #     if stringified == cycle_found[0]:
        #         print(f'Cycle start: {cycle_found[1]}, Cycle end: {minute}')
        #         import pdb; pdb.set_trace()

        # Checking if the Cycle is correct
        # begin = 478
        # end = 506
        # if minute >= begin and (minute - begin) % (end - begin) == 0:
        #     stringified = ''.join(str(x[1]) for x in sorted(grid.items()))
        #     print(f'Minute: {minute}, Score: {stringified.count("|") * stringified.count("#")}')

        # Finding the answer
        # if minute == 495:
        #     stringified = ''.join(str(x[1]) for x in sorted(grid.items()))
        #     trees = sum(zone is PointType.TREES for zone in grid.values())
        #     lumberyards = sum(zone is PointType.LUMBERYARD for zone in grid.values())
        #     resource_value = trees * lumberyards
        #     print(f'Minute: {minute}, Score: {stringified.count("|") - stringified.count("#")}, Alternate Score Calculation: {resource_value}')
        #     return

    trees = sum(zone is PointType.TREES for zone in grid.values())
    lumberyards = sum(zone is PointType.LUMBERYARD for zone in grid.values())
    resource_value = trees * lumberyards
    return resource_value

resource_value = solution(False)
print(resource_value)
