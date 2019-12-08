from collections import defaultdict, deque
from copy import deepcopy


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.strip()


def addr(registers, A, B, C):
    registers[C] = registers[A] + registers[B]
    return registers


def addi(registers, A, B, C):
    registers[C] = registers[A] + B
    return registers


def mulr(registers, A, B, C):
    registers[C] = registers[A] * registers[B]
    return registers


def muli(registers, A, B, C):
    registers[C] = registers[A] * B
    return registers


def banr(registers, A, B, C):
    registers[C] = registers[A] & registers[B]
    return registers


def bani(registers, A, B, C):
    registers[C] = registers[A] & B
    return registers


def borr(registers, A, B, C):
    registers[C] = registers[A] | registers[B]
    return registers


def bori(registers, A, B, C):
    registers[C] = registers[A] | B
    return registers


def setr(registers, A, B, C):
    registers[C] = registers[A]
    return registers


def seti(registers, A, B, C):
    registers[C] = A
    return registers


def gtir(registers, A, B, C):
    registers[C] = 1 if A > registers[B] else 0
    return registers


def gtri(registers, A, B, C):
    registers[C] = 1 if registers[A] > B else 0
    return registers


def gtrr(registers, A, B, C):
    registers[C] = 1 if registers[A] > registers[B] else 0
    return registers


def eqir(registers, A, B, C):
    registers[C] = 1 if A == registers[B] else 0
    return registers


def eqri(registers, A, B, C):
    registers[C] = 1 if registers[A] == B else 0
    return registers


def eqrr(registers, A, B, C):
    registers[C] = 1 if registers[A] == registers[B] else 0
    return registers


def solution():
    operations = {
        "addr": addr,
        "addi": addi,
        "mulr": mulr,
        "muli": muli,
        "banr": banr,
        "bani": bani,
        "borr": borr,
        "bori": bori,
        "setr": setr,
        "seti": seti,
        "gtir": gtir,
        "gtri": gtri,
        "gtrr": gtrr,
        "eqir": eqir,
        "eqri": eqri,
        "eqrr": eqrr,
    }

    with open("input.txt") as f:
        lines = deque(f.readlines())

    count = 0
    opcode_matches = defaultdict(set)
    while lines:
        before = list(map(int, lines.popleft().strip()[9:-1].split(", ")))
        operation = list(map(int, lines.popleft().strip().split(" ")))
        after = list(map(int, lines.popleft().strip()[9:-1].split(", ")))
        try:
            lines.popleft()  # empty line
        except IndexError:
            pass

        opcode, a, b, c = operation
        for operation in operations.values():
            valid_operations = 0
            if operation(deepcopy(before), a, b, c) == after:
                valid_operations += 1
                opcode_matches[opcode].add(operation.__name__)

        count += 1 if valid_operations >= 3 else 0

    seen_keys = set()
    while True:
        had_changes = False
        for key, val in opcode_matches.items():
            if len(val) == 1 and key not in seen_keys:
                seen_keys.add(key)
                break
        else:
            if not had_changes:
                break

        for target_key, target_val in opcode_matches.items():
            if key == target_key:
                continue
            target_val -= val

    final_opcodes = {
        key: operations[list(val)[0]] for key, val in opcode_matches.items()
    }

    registers = after
    for line in read_file("input2.txt"):
        opcode, a, b, c = map(int, line.strip().split(" "))
        operation = final_opcodes[opcode]
        operation(registers, a, b, c)

    return count, registers


print(solution())
