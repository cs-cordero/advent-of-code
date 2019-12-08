from collections import defaultdict
from heapq import heapify, heappop, heappush


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.strip()


def find_nodes_with_no_dependencies(graph):
    all_nodes = set(graph.keys())
    nodes_with_dependencies = set().union(*graph.values())
    return all_nodes - nodes_with_dependencies


def solution():
    graph = defaultdict(set)
    backwards_graph = defaultdict(set)
    visited_nodes = set()
    order = ""

    for line in read_file("input.txt"):
        _a, _b = line[5:].split("must be finished before step ")
        source, target = ord(_a[0]), ord(_b[0])
        graph[source].add(target)
        backwards_graph[target].add(source)

    queue = list(find_nodes_with_no_dependencies(graph))
    heapify(queue)
    while queue:
        next_node = heappop(queue)
        visited_nodes.add(next_node)
        order += chr(next_node)

        for node in graph[next_node]:
            prerequisites = backwards_graph[node]
            if visited_nodes.issuperset(prerequisites):
                heappush(queue, node)
    return order


def solution2():
    graph = defaultdict(set)
    backwards_graph = defaultdict(set)
    visited_nodes = set()
    workers = {}
    time = 0

    for line in read_file("input.txt"):
        _a, _b = line[5:].split("must be finished before step ")
        source, target = ord(_a[0]), ord(_b[0])
        graph[source].add(target)
        backwards_graph[target].add(source)

    queue = list(find_nodes_with_no_dependencies(graph))
    heapify(queue)

    all_nodes = set(graph.keys()) | set(backwards_graph.keys())
    while all_nodes != visited_nodes:
        # queue another worker if we can
        while queue and len(workers) < 5:
            next_node = heappop(queue)
            workers[time + 60 + next_node - ord("A")] = next_node

        # worker completes work
        if time in workers:
            visited_nodes.add(workers.pop(time))

        # add to the queue
        for node in all_nodes - visited_nodes - set(queue) - set(workers.values()):
            prerequisites = backwards_graph[node]
            if visited_nodes.issuperset(prerequisites):
                heappush(queue, node)

        time += 1
    return time


print(solution())
print(solution2())
