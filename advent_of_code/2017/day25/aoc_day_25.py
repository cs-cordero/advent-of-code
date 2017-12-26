from collections import defaultdict

def solution():
    d = defaultdict(int)
    pos = 0
    state = 'A'

    for i in range(12919244):
        if state == 'A':
          if d[pos] == 0:
            d[pos] = 1
            pos += 1
            state = 'B'
          elif d[pos] == 1:
            d[pos] = 0
            pos += -1
            state = 'C'

        elif state == 'B':
          if d[pos] == 0:
            d[pos] = 1
            pos += -1
            state = 'A'
          elif d[pos] == 1:
            d[pos] = 1
            pos += 1
            state = 'D'

        elif state == 'C':
          if d[pos] == 0:
            d[pos] = 1
            pos += 1
            state = 'A'
          elif d[pos] == 1:
            d[pos] = 0
            pos += -1
            state = 'E'

        elif state == 'D':
          if d[pos] == 0:
            d[pos] = 1
            pos += 1
            state = 'A'
          elif d[pos] == 1:
            d[pos] = 0
            pos += 1
            state = 'B'

        elif state == 'E':
          if d[pos] == 0:
            d[pos] = 1
            pos += -1
            state = 'F'
          elif d[pos] == 1:
            d[pos] = 1
            pos += -1
            state = 'C'

        elif state == 'F':
          if d[pos] == 0:
            d[pos] = 1
            pos += 1
            state = 'D'
          elif d[pos] == 1:
            d[pos] = 1
            pos += 1
            state = 'A'
    return sum(1 for k, v in d.items() if v == 1)

print(solution())
