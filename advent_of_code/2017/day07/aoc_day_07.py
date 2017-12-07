import re
from collections import deque

class Node:
    def __init__(self, name):
        self.name = name
        self.data = None
        self.parent = None
        self.cum_data = None
        self.children = []

def solution(tree_root):
    print(f'Part 1: {tree_root.name}')
    print(f'Part 2: {find_imbalance(tree_root)}')

def create_tree(nodes):
    """ Takes the raw data and generates a Tree. Returns root. """

    def find_root(graph):
        # arbitrary start point
        root_node = graph.get(next(iter(all_nodes.keys())))

        while root_node.parent is not None:
            root_node = root_node.parent
        return root_node

    def calculate_cumulative_weight(node):
        node.cum_data = node.data
        if not node.children:
            return node.cum_data
        node.cum_data += sum(calculate_cumulative_weight(child)
                             for child in node.children)
        return node.cum_data

    all_nodes = {}
    for node in nodes:
        split_node = node.split(' -> ')
        parent = re.sub('[()]', '', split_node[0]).split(' ')
        parent_node = all_nodes.setdefault(parent[0], Node(parent[0]))
        parent_node.data = parent_node.data or int(parent[1])

        children = split_node[1].split(', ') if len(split_node) > 1 else []
        for child in children:
            child_node = all_nodes.setdefault(child, Node(child))
            child_node.parent = parent_node
            parent_node.children.append(child_node)

    root = find_root(all_nodes)
    calculate_cumulative_weight(root)
    return root


def find_imbalance(root):
    """ Beginning at root, traverse to the lowest imbalanced node

    Returns the value that the lowest imbalanced node should be to make a
    balanced tree (as defined by the problem)
    """
    queue = deque()
    queue.append(root)
    while queue:
        node = queue.popleft()
        num_children = len(node.children)
        if num_children == 0:
            continue
        elif num_children == 1:
            queue.append(node.children[0])
        elif num_children == 2:
            if not (node.children[0].cum_data == node.children[1].cum_data):
                queue.extend(node.children)
        else:
            first_child = node.children[0]
            second_child = node.children[1]

            if first_child.cum_data != second_child.cum_data:
                if first_child.cum_data == node.children[2].cum_data:
                    queue.append(second_child)
                    continue
                else:
                    queue.append(first_child)
                    continue

            for i in range(2, len(node.children)):
                if first_child.cum_data != node.children[i].cum_data:
                    queue.append(node.children[i])
                    break
    # node is now the imbalanced node
    siblings = [sibling for sibling in node.parent.children
                if not sibling is node]
    return node.data - (node.cum_data - siblings[0].cum_data)


if __name__ == '__main__':
    with open('aoc_day_07_input.txt') as f:
        s = (line.strip() for line in f.readlines())
    tree = create_tree(s)
    solution(tree)
