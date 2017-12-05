def solution(instructions):
    steps = 0
    instruction = 0
    while 0 <= instruction and instruction < len(instructions):
        step_val = instructions[instruction]
        instructions[instruction] += 1
        instruction += step_val
        steps += 1
    return steps

def solution2(instructions):
    steps = 0
    instruction = 0
    while 0 <= instruction and instruction < len(instructions):
        step_val = instructions[instruction]
        instructions[instruction] += 1 if step_val < 3 else -1
        instruction += step_val
        steps += 1
    return steps

if __name__ == '__main__':
    with open('aoc_day_05_input.txt') as f:
        s = [int(x) for x in f.readlines()]
    print(solution(list(s)))
    print(solution2(list(s)))
