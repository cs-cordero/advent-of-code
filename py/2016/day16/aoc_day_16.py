#!/bin/python3


DISK_SIZE = 272
INIT_STATE = "00111101111101000"


def fill_disk(state, max_size):
    if len(state) > max_size:
        return state[:max_size]

    temp_state = state
    state += "0"
    for i in range(1, len(temp_state) + 1):
        state += str(abs(int(temp_state[-i]) - 1))

    return fill_disk(state, max_size)


def acquire_check_sum(state):
    if len(state) % 2 != 0:
        return state

    checkdict = {"11": "1", "00": "1", "10": "0", "01": "0"}
    i = 0
    checksum = ""
    while i < len(state):
        checksum += checkdict[state[i : i + 2]]
        i += 2

    return acquire_check_sum(checksum)


def day16():
    disk = fill_disk(INIT_STATE, DISK_SIZE)
    checksum = acquire_check_sum(disk)
    print("Part 1: {}".format(checksum))

    disk = fill_disk(INIT_STATE, 35651584)
    checksum = acquire_check_sum(disk)
    print("Part 2: {}".format(checksum))


if __name__ == "__main__":
    day16()
