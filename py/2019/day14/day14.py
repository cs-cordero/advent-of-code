from __future__ import annotations

import math
from copy import deepcopy
from dataclasses import dataclass, field
from typing import Dict, List


@dataclass
class Vertex:
    per_production_amount: int = 0
    edges: Dict[str, int] = field(default_factory=dict)
    reversed_edges: Dict[str, int] = field(default_factory=dict)
    required: int = 0

    def add_edge(self, vertex: str, cost: int) -> None:
        self.edges[vertex] = cost

    def add_reversed_edge(self, vertex: str, cost: int) -> None:
        self.reversed_edges[vertex] = cost


def generate_graph(data: List[str]) -> Dict[str, Vertex]:
    graph = {}
    for line in data:
        components, output = line.strip().split(" => ")
        produced_amount, output_name = output.split(" ")
        vertex = graph.setdefault(output_name, Vertex())
        vertex.per_production_amount = int(produced_amount)

        for component in components.split(", "):
            required_amount, component_name = component.split(" ")
            component = graph.setdefault(component_name, Vertex())

            vertex.add_edge(component_name, int(required_amount))
            component.add_reversed_edge(output_name, int(required_amount))

    graph["FUEL"].required = 1
    graph["ORE"].per_production_amount = 1
    return graph


def get_reversed_top_sort(graph: Dict[str, Vertex], start: str) -> List[str]:
    topological_order = []
    visited = set()

    def top_sort_dfs(vertex: Vertex, name: str) -> None:
        if not vertex.reversed_edges:
            topological_order.append(name)
            return

        for neighbor in vertex.reversed_edges:
            if neighbor in visited:
                continue
            visited.add(neighbor)
            top_sort_dfs(graph[neighbor], neighbor)
        topological_order.append(name)

    top_sort_dfs(graph[start], start)
    return topological_order


def solution1(data: Dict[str, Vertex]) -> int:
    graph = deepcopy(data)
    topological_order = get_reversed_top_sort(data, "ORE")
    for vertex_name in topological_order:
        vertex = graph[vertex_name]
        required_iterations = max(
            1, math.ceil(vertex.required / vertex.per_production_amount)
        )
        for component_name, cost in vertex.edges.items():
            graph[component_name].required += required_iterations * cost
    return graph["ORE"].required


def solution2(data: Dict[str, Vertex]) -> int:
    graph = deepcopy(data)

    best = 0
    ore_limit = 1000000000000
    right = 1000000000000
    left = 1
    while left <= right:
        mid = (left + right) // 2
        graph["FUEL"].required = mid
        required_ore = solution1(graph)
        if required_ore < ore_limit:
            best = max(best, mid)
            left = mid + 1
        elif required_ore > ore_limit:
            right = mid - 1
        else:
            return mid
    return best


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = generate_graph(f.readlines())
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
