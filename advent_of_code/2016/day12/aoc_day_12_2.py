#!/bin/python3


def day12(lines):
    registers = {"a": 0, "b": 0, "c": 1, "d": 0}
    i = 0
    k = 0
    while i < len(lines):
        if k > 0:
            i += 1
            k -= 1
            continue
        line = lines[i].strip("\n").split(" ")
        print(
            "[%d, %d, %d, %d], %s"
            % (registers["a"], registers["b"], registers["c"], registers["d"], line)
        )
        try:
            x = registers[line[1]]
        except KeyError:
            x = int(line[1])

        if line[0] == "cpy":
            registers[line[2]] = registers.get(line[1], x)
        elif line[0] == "inc":
            registers[line[1]] += 1
        elif line[0] == "dec":
            registers[line[1]] -= 1
        elif line[0] == "jnz" and x != 0:
            jump = int(line[2])
            if jump < 0:
                i = max(i + jump, 0)
                continue
            else:
                k += jump
                continue

        i += 1
    return registers["a"]


with open("aoc_day_12_input.txt") as f:
    r = f.readlines()

print(day12(r))
