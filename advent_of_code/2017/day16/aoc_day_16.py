def solution(moves):
    moves = list(moves)
    programs = [chr(s) for s in range(ord('a'), ord('a')+16)]
    hashmap = {program: position for position, program in enumerate(programs)}
    movemap = {
        's': spin,
        'x': exchange,
        'p': partner
    }

    seen = []
    s = ''.join(programs)
    while s not in seen:
        seen.append(s)
        for move, instructions in moves:
            movemap.get(move)(instructions, programs, hashmap)
        s = ''.join(programs)

    first = seen[1]
    billionth = seen[1000000000 % len(seen)]

    return first, billionth

def spin(x, programs, hashmap):
    spin_size = int(x[0]) % len(programs)
    new_programs = programs[-spin_size:] + programs[:-spin_size]

    for i, program in enumerate(programs[-spin_size:]):
        hashmap[program] = i
    for program in programs[:-spin_size]:
        hashmap[program] += spin_size

    for i in range(len(programs)):
        programs[i] = new_programs[i]

def exchange(positions, programs, hashmap):
    a, b = map(int, positions)
    pA, pB = programs[a], programs[b]

    programs[a], programs[b] = programs[b], programs[a]
    hashmap[pA], hashmap[pB] = hashmap[pB], hashmap[pA]

def partner(prog_names, programs, hashmap):
    pA, pB = prog_names
    a, b = hashmap[pA], hashmap[pB]

    programs[a], programs[b] = programs[b], programs[a]
    hashmap[pA], hashmap[pB] = hashmap[pB], hashmap[pA]

if __name__ == '__main__':
    with open('aoc_day_16_input.txt', 'r') as f:
        s = ((inst[0], inst[1:].split('/'))
             for inst in f.readlines()[0].split(','))
    answer = solution(s)
    print(f'Part One: {answer[0]}')
    print(f'Part Two: {answer[1]}')
