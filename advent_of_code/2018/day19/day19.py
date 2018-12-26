from math import sqrt, floor


TEST_DATA = """
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
"""


def read_file(filepath, use_test_data = False):
    if not use_test_data:
        with open(filepath) as f:
            for line in f.readlines():
                yield line.strip()
    else:
        for line in TEST_DATA.strip().split('\n'):
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


OPERATIONS = {
    'addr': addr, 'addi': addi,
    'mulr': mulr, 'muli': muli,
    'banr': banr, 'bani': bani,
    'borr': borr, 'bori': bori,
    'setr': setr, 'seti': seti,
    'gtir': gtir, 'gtri': gtri,
    'gtrr': gtrr, 'eqir': eqir,
    'eqri': eqri, 'eqrr': eqrr,
}


def solution(use_test_data = False):
    registers = [0 for _ in range(6)]
    ip_register, *instructions = read_file('input.txt', use_test_data)
    ip_register = int(ip_register.split(' ')[1])
    instructions = [
        (OPERATIONS[instruction.split(' ')[0]], [int(x) for x in instruction.split(' ')[1:]])
        for instruction in instructions
    ]

    ip = registers[ip_register]
    while ip < len(instructions):
        DEBUG_ip = f'ip={str(ip).ljust(3)}'
        DEBUG_start = f'[{", ".join(str(x).rjust(6) for x in registers)}]'
        DEBUG = f'{DEBUG_ip} {DEBUG_start}'
        registers[ip_register] = ip
        operation, data = instructions[ip]
        operation(registers, *data)
        ip = registers[ip_register]
        DEBUG += f' {operation.__name__} {" ".join(str(x) for x in data)} {registers}'
        print(DEBUG)
        ip += 1


def solution2():
    register5 = 10551264
    factors = []
    for divisor in range(1, floor(sqrt(register5))):
        quotient = register5 / divisor
        if quotient == floor(quotient):
            factors.extend([int(quotient), divisor])
    return sum(factors)


# print(solution(False))
print(solution2())
