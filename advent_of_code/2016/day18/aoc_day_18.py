#!/bin/python3


def check_trap(prev_row, i):
    if i - 1 < 0:
        left = '.'
    else:
        left = prev_row[i-1]
    if i + 1 > row_len - 1:
        right = '.'
    else:
        right = prev_row[i+1]
    if (left == '.' and right == '^') \
        or (left == '^' and right == '.'):
        return '^'
    else:
        return '.'


def main():
    rows = []
    with open('aoc_day_18_input.txt') as f:
        rows.append([x for x in f.read()])
    counter = rows[0].count('.')

    row_len = len(rows[0])
    row = 1
    while row < 400000:
        next_row = []
        for i in range(row_len):
            next_square = check_trap(rows[row-1], i)
            counter += 1 if next_square == '.' else 0
            next_row.append(next_square)
        rows.append(next_row)
        row += 1

    return counter


if __name__ == '__name__':
    print(main())