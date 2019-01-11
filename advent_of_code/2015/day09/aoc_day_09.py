from collections import deque
from typing import Dict, Optional, NamedTuple, List


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.rstrip()


class Node:
    def __init__(self) -> None:
        self.children: Dict['Node', int] = {}


class QueueNode(NamedTuple):
    path: List[Node]
    distance: int


def solution(part2: bool = False):
    locations = {}
    for line in read_file('aoc_day_09_input.txt'):
        path, distance = line.split(' = ')
        source, target = path.split(' to ')
        source_node = locations.setdefault(source, Node())
        target_node = locations.setdefault(target, Node())
        source_node.children[target_node] = int(distance)
        target_node.children[source_node] = int(distance)

    location_count = len(locations)
    shortest_or_longest_distance_traveled = (1 << 31) * (-1 if part2 else 1)
    for city, node in locations.items():
        queue = deque([QueueNode([node], 0)])
        while queue:
            path, distance_so_far = queue.popleft()
            current_city = path[-1]
            if len(path) == location_count:
                if (shortest_or_longest_distance_traveled <= distance_so_far
                        and not part2):
                    continue
                elif (shortest_or_longest_distance_traveled >= distance_so_far
                      and part2):
                    continue
                shortest_or_longest_distance_traveled = distance_so_far

            for next_city, distance_to_travel in current_city.children.items():
                if next_city in path:
                    continue
                next_distance = distance_so_far + distance_to_travel
                next_path = path + [next_city]
                queue.append(QueueNode(next_path, next_distance))
    return shortest_or_longest_distance_traveled


print(solution(part2=False))
print(solution(part2=True))
