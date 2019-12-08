import math
from collections import defaultdict


def solution(instructions):
    registers = defaultdict(int)
    instruction_pointer = 0
    iterations = 0
    fn_map = {
        "set": set_register,
        "sub": subtract_register,
        "mul": multiply_register,
        "jnz": jump,
    }
    mul_counter = 0
    while instruction_pointer < len(instructions):
        iterations += 1
        current = instructions[instruction_pointer]
        fn = fn_map[current[0]]
        if current[0] == "jnz":
            instruction_pointer += fn(registers, current)
            continue
        else:
            mul_counter += 1 if current[0] == "mul" else 0
            fn(registers, current)
            instruction_pointer += 1
    return mul_counter


def solution2():
    nums = [i for i in range(122701)]
    nums[0] = False
    nums[1] = False
    for i in range(2, int(math.sqrt(122700))):
        if nums[i] is False:
            continue
        for j in range(i + i, 122701, i):
            nums[j] = False
    return [x for x in nums if x]


def set_register(registers, instruction):
    action, x, y = instruction
    registers[x] = registers[y] if y.isalpha() else int(y)


def subtract_register(registers, instruction):
    action, x, y = instruction
    registers[x] -= registers[y] if y.isalpha() else int(y)


def multiply_register(registers, instruction):
    action, x, y = instruction
    registers[x] *= registers[y] if y.isalpha() else int(y)


def jump(registers, instruction):
    action, x, y = instruction
    x = registers[x] if x.isalpha() else int(x)
    return int(y) if x != 0 else 1


if __name__ == "__main__":
    with open("aoc_day_23_input.txt", "r") as f:
        s = [line.strip().split(" ") for line in f.readlines()]
    print(solution(s))
    primes = solution2()
    count = sum(
        1 for prime in primes if prime >= 105700 and prime <= 122700 and prime % 17 == 0
    )
    print(count)
    import pdb

    pdb.set_trace()
