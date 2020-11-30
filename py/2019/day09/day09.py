from collections import defaultdict
from enum import Enum
from typing import Dict, Generator, List, Tuple

from intcode import IntcodeComputer


def solution1(data: List[int]) -> int:
    computer = IntcodeComputer(data)
    computer.send(1)
    computer.run_until_blocked()
    return computer.read()


def solution2(data: List[int]) -> object:
    computer = IntcodeComputer(data)
    computer.send(2)
    computer.run_until_blocked()
    return computer.read()


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
