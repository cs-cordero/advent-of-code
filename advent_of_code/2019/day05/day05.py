from typing import Dict, List, Tuple


def solution1(data: List[int]) -> int:
    return intcode_computer(data, 1)


def solution2(data: List[int]) -> int:
    return intcode_computer(data, 5)


def intcode_computer(data: List[int], pegged_input: int) -> int:
    registers = {i: value for i, value in enumerate(data)}

    i = 0
    last_output = None
    while registers[i] != 99:
        opcode, *modes = parse_instruction(registers[i])
        if opcode in (1, 2):
            j = get_value(registers, registers[i + 1], modes[0])
            k = get_value(registers, registers[i + 2], modes[1])
            registers[registers[i + 3]] = j + k if opcode == 1 else j * k
            i += 4
        elif opcode == 3:
            registers[registers[i + 1]] = pegged_input
            i += 2
        elif opcode == 4:
            last_output = get_value(registers, registers[i + 1], modes[0])
            i += 2
        elif opcode == 5:
            j = get_value(registers, registers[i + 1], modes[0])
            k = get_value(registers, registers[i + 2], modes[1])
            i = k if j != 0 else i + 3
        elif opcode == 6:
            j = get_value(registers, registers[i + 1], modes[0])
            k = get_value(registers, registers[i + 2], modes[1])
            i = k if j == 0 else i + 3
        elif opcode == 7:
            j = get_value(registers, registers[i + 1], modes[0])
            k = get_value(registers, registers[i + 2], modes[1])
            registers[registers[i + 3]] = 1 if j < k else 0
            i += 4
        elif opcode == 8:
            j = get_value(registers, registers[i + 1], modes[0])
            k = get_value(registers, registers[i + 2], modes[1])
            registers[registers[i + 3]] = 1 if j == k else 0
            i += 4
        else:
            assert False, "Received invalid opcode"

    return last_output


def parse_instruction(instruction: int) -> Tuple[int, bool, bool, bool]:
    opcode = instruction % 100
    instruction //= 100
    param1 = instruction % 10
    instruction //= 10
    param2 = instruction % 10
    instruction //= 10
    param3 = instruction % 10
    return opcode, bool(param1), bool(param2), bool(param3)


def get_value(registers: Dict[int, int], pos: int, immediate_mode: bool = False) -> int:
    return registers[pos] if not immediate_mode else pos


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().split(",")))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")