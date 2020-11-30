from typing import Any, Dict, List, Tuple


def hlf(registers: Dict[str, int], register: str) -> int:
    registers[register] //= 2
    return 1


def tpl(registers: Dict[str, int], register: str) -> int:
    registers[register] *= 3
    return 1


def inc(registers: Dict[str, int], register: str) -> int:
    registers[register] += 1
    return 1


def jmp(registers: Dict[str, int], offset: int) -> int:
    return offset


def jie(registers: Dict[str, int], register: str, offset: int) -> int:
    return offset if registers[register] % 2 == 0 else 1


def jio(registers: Dict[str, int], register: str, offset: int) -> int:
    return offset if registers[register] == 1 else 1


OPERATION_MAP = {"hlf": hlf, "tpl": tpl, "inc": inc, "jmp": jmp, "jie": jie, "jio": jio}


def solution1(instructions: List[List[Tuple[Any, ...]]]) -> int:
    registers = {"a": 0, "b": 0}
    i = 0
    while i < len(instructions):
        operation, *args = instructions[i]
        i += OPERATION_MAP[operation](registers, *args)
    return registers["b"]


def solution2(instructions: List[List[Tuple[Any, ...]]]) -> int:
    registers = {"a": 1, "b": 0}
    i = 0
    while i < len(instructions):
        operation, *args = instructions[i]
        i += OPERATION_MAP[operation](registers, *args)
    return registers["b"]


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = [
            tuple(
                value if "+" not in value and "-" not in value else int(value)
                for value in line.strip().replace(",", "").split(" ")
            )
            for line in f.readlines()
        ]
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
