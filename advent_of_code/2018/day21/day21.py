import os

def solution(zero_register, find_cycle = False, current=None, total=None):
    if current and total:
        os.system('clear')
        pct_complete = str(round(current / total * 100, 1)).rjust(5)
        total = str(total)
        current = str(current).rjust(len(total))
        print(f'{pct_complete}%:  {current} out of {total}.  Working on {zero_register} now.')
    else:
        print(f'Working on {zero_register} now.')
    r0 = zero_register
    r1 = 0
    r2 = 0
    r3 = 0
    r4 = 65536
    r5 = 3935295
    seen = {}
    steps = 0
    while True:
        r2 = r4 & 255
        steps += 1
        r5 = ((((r5 + r2) & 16777215) * 65899) & 16777215)
        steps += 4
        if r4 < 256:
            steps += 2
            if r5 == r0:
                steps += 1
                return steps
            else:
                steps += 1
                if find_cycle:
                    if r5 in seen:
                        return [x for x in seen.keys()]
                    else:
                        seen[r5] = True
                r4 = r5 | 65536  # instruction 06
                r5 = 3935295     # instruction 07
                steps += 2
                continue
        elif r4 >= 256:
            steps += (r5 // 256) * 7
            r4 //= 256

all_valid_zero_registers = solution(0, True)
instructions_run = {
    zero_register: solution(zero_register, False,
                            i, len(all_valid_zero_registers))
    for i, zero_register in enumerate(all_valid_zero_registers)
}
sorted_instruction_counts = sorted(instructions_run.items(), key=lambda x: x[1])
print(f'Part 1: {sorted_instruction_counts[0][0]}')  # 16457176
print(f'Part 2: {sorted_instruction_counts[-1][0]}') # 13625951
