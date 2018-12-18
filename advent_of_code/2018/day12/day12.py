from collections import deque
from copy import deepcopy


def spread_plants(spread_patterns, state):
    pattern = deque(state[:4])
    new_state = deepcopy(state)
    for next_pot in range(4, len(state)):
        pattern.append(state[next_pot])

        spread_result = spread_patterns.get(''.join(pattern))
        changing_pot = next_pot - 2
        new_state[changing_pot] = spread_result or '.'

        pattern.popleft()
    return new_state


def solution(n, verbose=False):
    spread_patterns = {}
    with open('input.txt') as f:
        initial_state = f.readline()[15:].strip()
        f.readline()
        for line in f.readlines():
            spread_pattern, result = line.strip().split(' => ')
            spread_patterns[spread_pattern] = result

    state_padding_size = 1000
    state = list('.' * state_padding_size + initial_state + '.' * state_padding_size)

    # if verbose:
    #     print(f'0: {"".join(state)}')

    foo = []
    for i in range(1, n+1):
        state = spread_plants(spread_patterns, state)
        # if verbose:
        #     print(f'{i}: {"".join(state)}')
        if verbose:
            foo.append(sum(i-state_padding_size for i in range(len(state)) if state[i] == '#'))

    bar = [foo[i] - foo[i-1] for i in range(1, len(foo))]
    import pdb; pdb.set_trace()
    return sum(i-state_padding_size for i in range(len(state)) if state[i] == '#')


# print(solution(20))
print(solution(2000, verbose = True))
