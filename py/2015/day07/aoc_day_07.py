from enum import Enum
from functools import partial
from typing import Callable, Optional, Union

SignalFunc = Callable[..., Optional[int]]


class Node:
    def __init__(
        self, spec: Optional[str] = None, signal_func: Optional[SignalFunc] = None
    ) -> None:
        self.value: Optional[int] = None
        self.spec = spec
        self.signal = signal_func

    def receive_signal(self) -> bool:
        old_value = self.value
        new_value = self.signal()
        if new_value and new_value < 0:
            new_value += 1 << 16
        self.value = new_value if new_value is not None else old_value
        return old_value != new_value and new_value is not None

    def __repr__(self) -> str:
        if self.spec is None:
            return f"Node: Unknown Spec with Value {self.value}"
        return f"Node: {self.spec} ({self.value})"


def op_set(value: Union[Node, int]) -> int:
    return value.value if isinstance(value, Node) else value


def op_and(arg1: Union[Node, int], arg2: Union[Node, int]) -> Optional[int]:
    a = arg1.value if isinstance(arg1, Node) else arg1
    b = arg2.value if isinstance(arg2, Node) else arg2
    return a & b if a is not None and b is not None else None


def op_lshift(node: Node, lshift_val: int) -> Optional[int]:
    return node.value << lshift_val if node.value is not None else None


def op_rshift(node: Node, rshift_val: int) -> Optional[int]:
    return node.value >> rshift_val if node.value is not None else None


def op_not(node: Node) -> Optional[int]:
    return ~node.value if node.value is not None else None


def op_or(node1: Node, node2: Node) -> Optional[int]:
    return (
        node1.value | node2.value
        if node1.value is not None and node2.value is not None
        else None
    )


class Operation(Enum):
    value: SignalFunc

    SET = op_set
    AND = op_and
    LSHIFT = op_lshift
    RSHIFT = op_rshift
    NOT = op_not
    OR = op_or


def solution(part2: Optional[int] = None):
    with open("aoc_day_07_input.txt") as f:
        lines = f.readlines()

    nodes = {}

    def get_value_or_node(value: str, line: str) -> Union[int, Node]:
        try:
            return int(value)
        except ValueError:
            return nodes.setdefault(value, Node(line.strip()))

    for line in lines:
        source, target = line.strip().split(" -> ")
        target_node = nodes.setdefault(target, Node())
        target_node.spec = line.strip()

        if "AND" in source:
            arg_name1, arg_name2 = source.split(" AND ")
            arg1 = get_value_or_node(arg_name1, line)
            arg2 = get_value_or_node(arg_name2, line)
            target_node.signal = partial(Operation.AND, arg1, arg2)
        elif "LSHIFT" in source:
            node_name, value = source.split(" LSHIFT ")
            node = nodes.setdefault(node_name, Node())
            value = int(value)
            target_node.signal = partial(Operation.LSHIFT, node, value)
        elif "RSHIFT" in source:
            node_name, value = source.split(" RSHIFT ")
            node = nodes.setdefault(node_name, Node())
            value = int(value)
            target_node.signal = partial(Operation.RSHIFT, node, value)
        elif "NOT" in source:
            _, node_name = source.split("NOT ")
            node = nodes.setdefault(node_name, Node())
            target_node.signal = partial(Operation.NOT, node)
        elif "OR" in source:
            node_name1, node_name2 = source.split(" OR ")
            node1 = nodes.setdefault(node_name1, Node())
            node2 = nodes.setdefault(node_name2, Node())
            target_node.signal = partial(Operation.OR, node1, node2)
        else:
            arg = get_value_or_node(source, line)
            target_node.signal = partial(Operation.SET, arg)

    if part2:
        nodes["b"].signal = partial(Operation.SET, part2)
        nodes["b"].spec = f"{part2} -> b"
    while sum(node.receive_signal() for name, node in nodes.items()):
        pass

    return nodes["a"].value


part1 = solution()
print(f"Part 1: {part1}")
print(f"Part 2: {solution(part1)}")
