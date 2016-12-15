#!/bin/python3
# -*- coding: utf-8 -*-

import re


def load_inputs():
    floors = {}
    with open('aoc_day_11_input.txt') as f:
        i = 1
        for line in f:
            floors[i] = re.findall(
                r'(\w+\s*(?:-compatible microchip|generator))',
                line.strip())
            i += 1
    return floors

print(load_inputs())