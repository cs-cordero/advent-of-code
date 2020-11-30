from collections import deque
from collections.abc import MutableMapping
from copy import deepcopy
from functools import wraps
from typing import NamedTuple


class Grid(MutableMapping):
    class Point(NamedTuple):
        x: int
        y: int

        def __add__(self, other):
            x1, y1 = self
            x2, y2 = other
            return Grid.Point(x1 + x2, y1 + y2)

        def __repr__(self):
            return f"({self.x}, {self.y})"

        def get_adjacent_points(self, *, minx=None, maxx=None, miny=None, maxy=None):
            def generator():
                for delta in (
                    (0, -1),
                    (-1, 0),
                    (1, 0),
                    (0, 1),
                    (-1, -1),
                    (1, -1),
                    (-1, 1),
                    (1, 1),
                ):
                    yield self + delta

            return [
                point
                for point in generator()
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
                key = kwargs.get("key", args[0])
            except KeyError:
                raise Exception("Expected a tuple key!")

            assert isinstance(key, Grid.Point) or (
                isinstance(key, tuple)
                and len(key) == 2
                and all(isinstance(value, int) for value in key)
            )

            return fn(self, *args, **kwargs)

        return wrapper

    @_impose_key_restrictions
    def __getitem__(self, key):
        try:
            return self._storage[key]
        except KeyError:
            self._storage[key] = set()
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
            minx=min(all_x), maxx=max(all_x), miny=min(all_y), maxy=max(all_y)
        )

    def copy(self):
        return deepcopy(self)

    def show(self):
        limits = self.limits
        for row in range(limits.miny, limits.maxy + 1):
            _row = ""
            for col in range(limits.minx, limits.maxx + 1):
                _row += str(self[(col, row)])
            print(_row)


def solution():
    with open("input.txt") as f:
        DATA_INPUT = f.readline().strip()[1:-1]
    # DATA_INPUT = 'ENWWW(NEEE|SSE(EE|N))'

    grid = Grid()
    deltas = {"N": (0, -1), "E": (1, 0), "W": (-1, 0), "S": (0, 1)}

    i = 0
    stack = []
    current_point = Grid.Point(0, 0)
    while i < len(DATA_INPUT):
        delta_character = DATA_INPUT[i]
        if delta_character == "(":
            stack.append(current_point)
        elif delta_character == ")":
            current_point = stack.pop()
        elif delta_character == "|":
            current_point = stack[-1]
        else:
            delta = deltas[delta_character]
            next_point = current_point + delta
            grid[current_point].add(next_point)
            grid[next_point].add(current_point)
            current_point = next_point
        i += 1

    queue = deque([(0, 0)])
    distances = {}
    distances[queue[0]] = 0
    while queue:
        current_node = queue.popleft()
        current_distance = distances[current_node]
        next_nodes = grid[current_node]
        for next_node in next_nodes:
            if next_node not in distances:
                distances[next_node] = current_distance + 1
                queue.append(next_node)

    return distances


distances = solution()
print(f"Part 1:  {distances[max(distances, key=lambda x: distances[x])]}")
print(f"Part 2:  {sum(distances[x] >= 1000 for x in distances)}")
