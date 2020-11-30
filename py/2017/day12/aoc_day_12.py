from collections import deque


class Node:
    def __init__(self, name, children):
        self.name = name
        self.children = children
        self.visited = False

    def __repr__(self):
        return f"{self.name}: {self.children}"


def solution(nodes):
    village = build_graph(nodes)

    part_one = tag_group(village.get("0"), village)
    part_two = 1
    for _, node in village.items():
        if node.visited:
            continue
        part_two += 1
        tag_group(node, village)
    return part_one, part_two


def build_graph(nodes):
    village = {}
    for node in nodes:
        name, children = node.split(" <-> ")
        program_node = village.get(name, Node(name, children.split(", ")))
        village[name] = program_node

        for child in program_node.children:
            child_node = village.get(child, Node(child, [name]))
            if name not in child_node.children:
                child_node.children.append(name)
    return village


def tag_group(starting_node, village):
    queue = deque([starting_node])
    starting_node.visited = True
    group_size = 1
    while queue:
        current = queue.popleft()
        for child in map(village.get, current.children):
            if child.visited:
                continue
            child.visited = True
            queue.append(child)
            group_size += 1
    return group_size


if __name__ == "__main__":
    with open("aoc_day_12_input.txt", "r") as f:
        s = [x.strip() for x in f.readlines()]
    answer = solution(s)
    print("Part One: ", answer[0])
    print("Part Two: ", answer[1])
