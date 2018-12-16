from collections import deque
from copy import deepcopy


def helper(nodes):
    children_count = nodes.popleft()
    metadata_count = nodes.popleft()

    sum_of_metadata = 0
    for child in range(children_count):
        sum_of_metadata += helper(nodes)

    for metadata in range(metadata_count):
        sum_of_metadata += nodes.popleft()

    return sum_of_metadata


def helper2(nodes):
    children_count = nodes.popleft()
    metadata_count = nodes.popleft()

    value = 0
    children = [helper2(nodes) for child in range(children_count)]
    if not children:
        sum_of_metadata = 0
        for metadata in range(metadata_count):
            sum_of_metadata += nodes.popleft()
        return sum_of_metadata

    metadata_points = [nodes.popleft() for metadata in range(metadata_count)]
    def get_child_value(index):
        try:
            return children[index-1]
        except IndexError:
            return 0
    return sum(map(get_child_value, metadata_points))


def solution():
    with open('input.txt') as f:
        nodes = deque(map(int, f.readline().strip().split(' ')))
    return (helper(deepcopy(nodes)), helper2(deepcopy(nodes)))

print(solution())
