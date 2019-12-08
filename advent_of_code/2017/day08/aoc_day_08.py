from collections import defaultdict


def solution(instructions):
    highest_register_ever = None
    registers = defaultdict(int)
    for instruction in instructions:
        action, conditional = instruction.strip().split(" if ")
        cond_register, cond_criteria = conditional.split(" ", maxsplit=1)
        eval_str = " ".join([f'registers["{cond_register}"]', cond_criteria])
        if eval(eval_str):
            register, atype, avalue = action.split(" ")
            registers[register] = registers[register] + int(avalue) * (
                1 if atype == "inc" else -1
            )
            if not highest_register_ever or highest_register_ever < registers[register]:
                highest_register_ever = registers[register]

    highest_register = None
    for k, v in registers.items():
        if not highest_register or highest_register < v:
            highest_register = v

    return highest_register, highest_register_ever


if __name__ == "__main__":
    with open("aoc_day_08_input.txt") as f:
        s = [line.strip() for line in f.readlines()]
    answer = solution(s)
    print(f"Part 1: {answer[0]}")
    print(f"Part 2: {answer[1]}")
