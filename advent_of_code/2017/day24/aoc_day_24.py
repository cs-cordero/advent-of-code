from collections import defaultdict

def solution():
    starting_components = d[0]
    best_scores = []
    for component in starting_components:
        my_score = sum(get_ports(component))
        n_a, n_b = get_ports(component)
        nxt_port = n_a if n_b == 0 else n_b
        best_scores.append(recurse(component, set(), nxt_port, 0))
    print('fuck', max(best_scores))

def recurse(component, seen, next_port, level):
    seen.add(component)
    c_a, c_b = get_ports(component)
    next_components = d[next_port] - seen
    my_score = sum(get_ports(component))

    scores = []
    for next_component in next_components:
        n_a, n_b = get_ports(next_component)
        nxt_port = n_a if n_b in (c_a, c_b) else n_b
        score, reclevel = recurse(next_component, seen.copy(), nxt_port, level+1)
        scores.append((score, reclevel))

    scores = sorted(scores, key=lambda x: (x[1], x[0]), reverse=True)
    print(component, level, scores)
    return my_score + (scores[0][0] if scores else 0), scores[0][1] if scores else level

def get_ports(component):
    return map(int, component.split('/'))

if __name__ == '__main__':
    d = defaultdict(set)
    #with open('aoc_day_24_sample.txt') as f:
    with open('aoc_day_24_input.txt') as f:
        sample = f.readlines()
    # sample = [
    #     '0/1',
    #     '1/2',
    #     '1/3',
    #     '1/4',
    #     '5/0',
    #     '2/5',
    #     '3/6',
    #     '4/500'
    # ]
    for component in sample:
        a, b = map(int, component.split('/'))
        d[a].add(component)
        d[b].add(component)
    solution()
