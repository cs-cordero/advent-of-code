import os
from collections import defaultdict
from enum import Enum
from typing import Dict, Generator, List, Tuple


def solution1(data: List[int]) -> int:
    computer = intcode_computer(data, None)
    next(computer)

    screen = {}
    while True:
        try:
            col = next(computer)
            row = next(computer)
            tile = next(computer)
        except StopIteration:
            break
        screen[(row, col)] = tile

    return sum(tile_id == 2 for tile_id in screen.values())


def solution2(data: List[int]) -> object:
    computer = intcode_computer(data, 0, 2)
    next(computer)

    screen = {}

    def print_screen():
        for row in range(20):
            s = ""
            for col in range(50):
                tile = screen.get((row, col))
                if tile in (None, 0):
                    s += " "  # empty
                elif tile == 1:
                    s += "#"  # wall
                elif tile == 2:
                    s += "X"  # block
                elif tile == 3:
                    s += "x"  # paddle
                elif tile == 4:
                    s += "o"  # ball
                else:
                    assert False
            print(s)

    score = 0
    direction = 0
    player_col = 18
    ball_col = 16
    while True:
        try:
            col = computer.send(direction)
            row = computer.send(direction)
            tile = computer.send(direction)
        except StopIteration:
            break
        if col == -1 and row == 0:
            # Special combination
            score = tile
            continue
        screen[(row, col)] = tile

        if tile == 4:
            ball_col = col
            os.system("clear")
            print_screen()
            print(score)

        if tile == 3:
            player_col = col

        if player_col < ball_col:
            direction = 1
        elif player_col > ball_col:
            direction = -1
        else:
            direction = 0
    return score


class ParameterMode(Enum):
    POSITION = 0
    IMMEDIATE = 1
    RELATIVE = 2


def intcode_computer(
    data: List[int], setting: int, seed: int = 1
) -> Generator[int, int, None]:
    registers = defaultdict(int)
    for i, value in enumerate(data):
        registers[i] = value
    registers[0] = seed

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
    result2 = solution2(data)
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {result2}")
