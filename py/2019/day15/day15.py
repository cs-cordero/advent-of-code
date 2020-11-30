from __future__ import annotations

from collections import defaultdict, deque
from enum import Enum
from typing import Dict, Generator, List, Tuple


def solution1(data: List[int]) -> int:
    graph = create_graph(data)
    queue = deque([((0, 0), 0)])
    visited = {(0, 0)}
    while queue:
        (row, col), steps = queue.popleft()
        for drow, dcol in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            next_loc = row + drow, col + dcol
            if next_loc in visited:
                continue
            visited.add(next_loc)

            value = graph.get(next_loc, "#")
            if value == "#":
                continue
            elif value == "T":
                return steps + 1
            queue.append((next_loc, steps + 1))
    return -1


def solution2(data: List[int]) -> int:
    graph = create_graph(data)
    for start, value in graph.items():
        if value == "T":
            break

    queue = deque([start])
    visited = {start}
    steps = 0
    while queue:
        next_queue = deque()
        while queue:
            row, col = queue.popleft()
            for drow, dcol in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                next_loc = row + drow, col + dcol
                if next_loc in visited:
                    continue
                visited.add(next_loc)

                if graph.get(next_loc, "#") == "#":
                    continue
                graph[next_loc] = "O"
                next_queue.append(next_loc)
        queue = next_queue.copy()
        if queue:
            steps += 1
    return steps


def create_graph(data: List[int]) -> Dict[Tuple[int, int], str]:
    computer = intcode_computer(data, 1)
    next(computer)

    graph = defaultdict(str)
    graph[(0, 0)] = "S"

    direction = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    reverse_map = {1: 2, 2: 1, 3: 4, 4: 3}

    def dfs(row: int, col: int, return_direction: int) -> None:
        for i, (drow, dcol) in enumerate(direction, 1):
            next_loc = (row + drow, col + dcol)
            if i == return_direction or next_loc in graph:
                continue
            result = computer.send(i)
            if result == 0:
                graph[next_loc] = "#"
                continue
            elif result == 1:
                graph[next_loc] = " "
            elif result == 2:
                graph[next_loc] = "T"

            dfs(row + drow, col + dcol, reverse_map[i])
        computer.send(return_direction)

    dfs(0, 0, 1)
    return graph


class ParameterMode(Enum):
    POSITION = 0
    IMMEDIATE = 1
    RELATIVE = 2


def intcode_computer(data: List[int], setting: int) -> Generator[int, int, None]:
    registers = defaultdict(int)
    for i, value in enumerate(data):
        registers[i] = value

    next_input = yield
    setting_has_been_set = False

    i = 0
    relative_offset = 0
    while registers[i] != 99:
        opcode, *modes = parse_instruction(registers[i])
        if opcode in (1, 2):
            j = get_value(registers, registers[i + 1], modes[0], relative_offset)
            k = get_value(registers, registers[i + 2], modes[1], relative_offset)
            m = get_write_location(registers[i + 3], modes[2], relative_offset)
            registers[m] = j + k if opcode == 1 else j * k
            i += 4
        elif opcode == 3:
            m = get_write_location(registers[i + 1], modes[0], relative_offset)
            registers[m] = setting if not setting_has_been_set else next_input
            setting_has_been_set = True
            i += 2
        elif opcode == 4:
            next_input = yield get_value(
                registers, registers[i + 1], modes[0], relative_offset
            )
            i += 2
        elif opcode == 5:
            j = get_value(registers, registers[i + 1], modes[0], relative_offset)
            k = get_value(registers, registers[i + 2], modes[1], relative_offset)
            i = k if j != 0 else i + 3
        elif opcode == 6:
            j = get_value(registers, registers[i + 1], modes[0], relative_offset)
            k = get_value(registers, registers[i + 2], modes[1], relative_offset)
            i = k if j == 0 else i + 3
        elif opcode == 7:
            j = get_value(registers, registers[i + 1], modes[0], relative_offset)
            k = get_value(registers, registers[i + 2], modes[1], relative_offset)
            m = get_write_location(registers[i + 3], modes[2], relative_offset)
            registers[m] = 1 if j < k else 0
            i += 4
        elif opcode == 8:
            j = get_value(registers, registers[i + 1], modes[0], relative_offset)
            k = get_value(registers, registers[i + 2], modes[1], relative_offset)
            m = get_write_location(registers[i + 3], modes[2], relative_offset)
            registers[m] = 1 if j == k else 0
            i += 4
        elif opcode == 9:
            j = get_value(registers, registers[i + 1], modes[0], relative_offset)
            relative_offset += j
            i += 2
        else:
            assert False, "Received invalid opcode"


def parse_instruction(instruction: int) -> Tuple[int, bool, bool, bool]:
    opcode = instruction % 100
    instruction //= 100
    param1 = instruction % 10
    instruction //= 10
    param2 = instruction % 10
    instruction //= 10
    param3 = instruction % 10
    return opcode, ParameterMode(param1), ParameterMode(param2), ParameterMode(param3)


def get_value(
    registers: Dict[int, int], pos: int, mode: ParameterMode, offset: int
) -> int:
    if mode == ParameterMode.POSITION:
        return registers[pos]
    elif mode == ParameterMode.IMMEDIATE:
        return pos
    elif mode == ParameterMode.RELATIVE:
        return registers[offset + pos]
    assert False


def get_write_location(pos: int, mode: ParameterMode, offset: int) -> int:
    return pos + offset if mode == ParameterMode.RELATIVE else pos


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().split(",")))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
