#!/bin/python3

import collections

ELF_COUNT = 3014603

class Elf(object):
    def __init__(self, i):
        self.i = i
        self.next = None


def solve_partone():
    # Generate linked list of Elves
    head = Elf(1)
    current_elf = head
    for i in range(2, ELF_COUNT + 1):
        new_elf = Elf(i)
        current_elf.next = new_elf
        current_elf = current_elf.next
    new_elf.next = head

    while head.next != head:
        head.next = head.next.next
        head = head.next
    return head.i


def solve_parttwo():
    left = collections.deque()
    right = collections.deque()
    for i in range(1, ELF_COUNT+1):
        if i < (ELF_COUNT // 2) + 1:
            left.append(i)
        else:
            right.appendleft(i)

    while left and right:
        if len(left) > len(right):
            left.pop()
        else:
            right.pop()

        # rotate
        right.appendleft(left.popleft())
        left.append(right.pop())
    return left[0] or right[0]


def main():
    print('Solve Part 1: %d' % solve_partone())
    print('Solve Part 2: %d' % solve_parttwo())

if __name__ == '__main__':
    main()