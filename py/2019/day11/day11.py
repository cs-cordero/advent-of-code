from collections import defaultdict
from enum import Enum
from typing import Dict, Generator, List, Tuple


def solution1(data: List[int]) -> int:
    graph = get_graph(data, 0)
    return len(graph) - 1


def solution2(data: List[int]) -> object:
    graph = get_graph(data, 1)
    min_row = float("inf")
    min_col = float("inf")
    max_row = float("-inf")
    max_col = float("-inf")

    for row, col in graph.keys():
        min_row = min(min_row, row)
        min_col = min(min_col, col)
        max_row = max(max_row, row)
        max_col = max(max_col, col)

    data = [
        "".join(
            "X" if graph[(row_i, col_i)] else " "
            for col_i in range(min_col, max_col + 1)
        )
        for row_i in range(min_row, max_row + 1)
    ]

    class ForPrettyPrinting:
        def __init__(self, data) -> None:
            self.data = data

        def __str__(self) -> str:
            return "\n" + "\n".join(data)

    return ForPrettyPrinting(data)


def get_graph(data: List[int], start_input: int) -> Dict[Tuple[int, int], int]:
    graph = defaultdict(bool)
    delta = [(-1, 0), (0, 1), (1, 0), (0, -1)]
    robot = (0, 0, 0)
    computer = intcode_computer(data, start_input)
    next(computer)
    while True:
        row, col, facing = robot
        try:
            value1 = computer.send(1 if graph[(row, col)] else 0)
            value2 = next(computer)
        except StopIteration:
            break

        graph[(row, col)] = bool(value1)

        if value2 == 0:
            facing -= 1
            facing %= 4
        else:
            facing += 1
            facing %= 4
        drow, dcol = delta[facing]
        row += drow
        col += dcol
        robot = (row, col, facing)
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
