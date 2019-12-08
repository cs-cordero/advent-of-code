import enum
import heapq
from collections import defaultdict
from typing import NamedTuple

with open("input.txt") as f:
    DEPTH = int(f.readline().strip().split(" ")[1])
    TARGET = tuple(map(int, f.readline().strip().split(" ")[1].split(",")))


class RegionType(enum.Enum):
    ROCKY = 0
    WET = 1
    NARROW = 2


class Equipment(enum.Enum):
    GEAR = 0
    TORCH = 1
    NEITHER = 2

    def __lt__(self, other):
        return self.value < other.value


EQUIPMENT_TO_REGION_TYPE_MAP = {
    Equipment.GEAR: {RegionType.ROCKY, RegionType.WET},
    Equipment.TORCH: {RegionType.ROCKY, RegionType.NARROW},
    Equipment.NEITHER: {RegionType.WET, RegionType.NARROW},
}

REGION_TYPE_TO_EQUIPMENT_MAP = defaultdict(set)
for equipment, region_types in EQUIPMENT_TO_REGION_TYPE_MAP.items():
    for region_type in region_types:
        REGION_TYPE_TO_EQUIPMENT_MAP[region_type].add(equipment)


class Node(NamedTuple):
    x: int
    y: int
    equipment: Equipment


class Cave:
    def __init__(self, depth, target):
        self._storage = {}
        self.depth = depth
        self.target = target

    def __getitem__(self, key):
        assert isinstance(key, tuple)
        assert len(key) == 2
        assert all(isinstance(val, int) for val in key)
        try:
            erosion_level = self._storage[key]
        except KeyError:
            x, y = key
            if (x, y) in ((0, 0), self.target):
                geologic_index = 0
            elif y == 0:
                geologic_index = x * 16807
            elif x == 0:
                geologic_index = y * 48271
            else:
                self[(x - 1, y)]
                self[(x, y - 1)]
                geologic_index = self._storage[(x - 1, y)] * self._storage[(x, y - 1)]

            erosion_level = (geologic_index + self.depth) % 20183
            self._storage[key] = erosion_level
        return RegionType(erosion_level % 3)

    def __setitem__(self, key, value):
        assert isinstance(key, tuple)
        assert len(key) == 2
        assert all(isinstance(val, int) for val in key)
        assert isinstance(value, int)
        self._storage[key] = value


def adjacent_points(point):
    assert isinstance(point, tuple)
    assert len(point) == 2
    assert all(isinstance(val, int) for val in point)
    x1, y1 = point
    for delta in [(0, -1), (-1, 0), (1, 0), (0, 1)]:
        x2, y2 = delta
        adjacent_point = (x1 + x2, y1 + y2)
        if not all(value >= 0 for value in adjacent_point):
            continue

        if adjacent_point[0] > DEPTH or adjacent_point[1] > DEPTH:
            continue

        yield adjacent_point


def solution():
    cave = Cave(DEPTH, TARGET)
    part1 = sum(
        cave[(x, y)].value for x in range(TARGET[0] + 1) for y in range(TARGET[1] + 1)
    )

    final_target = (*TARGET, Equipment.TORCH)
    time_distances = {(0, 0, Equipment.TORCH): 0, final_target: 1 << 31}

    queue = [(0, Node(0, 0, Equipment.TORCH))]
    while queue:
        minutes, node = heapq.heappop(queue)
        *point, equipment = node

        if node == final_target:
            return part1, minutes

        if minutes > time_distances.setdefault(node, 1 << 31):
            continue

        time_distances[node] = minutes

        # Add to Queue any valid Adjacent Node
        for adjacent_point in adjacent_points(point):
            next_node = Node(*adjacent_point, equipment)
            next_minutes = minutes + 1

            # Can reach the adjacent point with my current equipment
            if cave[adjacent_point] in EQUIPMENT_TO_REGION_TYPE_MAP[equipment]:
                adjacent_point_time = time_distances.setdefault(next_node, 1 << 31)
                if next_minutes < adjacent_point_time:
                    time_distances[next_node] = next_minutes
                    heapq.heappush(queue, (next_minutes, next_node))

        # Add to Queue current node with second set of equipment
        next_equipment = list(REGION_TYPE_TO_EQUIPMENT_MAP[cave[point]] - {equipment})[
            0
        ]
        next_node = Node(*point, next_equipment)
        next_minutes = minutes + 7
        if next_minutes < time_distances.setdefault(next_node, 1 << 31):
            time_distances[next_node] = next_minutes
            heapq.heappush(queue, (next_minutes, next_node))

    assert False, "Could not reach the target!"


part1, part2 = solution()
print(f"Part1: {part1}")
print(f"Part2: {part2}")
