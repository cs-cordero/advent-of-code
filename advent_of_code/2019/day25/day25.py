import os
from itertools import combinations
from typing import List

from intcode import IntcodeComputer


"""
   3
   21456
    078
  HGF9AD
    IEBC

0: Hull Breach
1: Hot Chocolate Fountain       Wreath
2: Arcade
3: Warp Drive Maintenance
4: Sick Bay                     Photons (Death)
5: Gift Wrapping Center
6: Navigation                   Weather Machine
7: Observatory                  Food Ration
8: Storage                      Infinite Loop (Death)
9: Science Lab                  Molten Lava (Death)
A: Corridor                     Escape Pod (Death)
B: Engineering                  Hypercube
C: Crew Quarters                Space Law Space Brochure
D: Security Checkpoint
E: Kitchen
F: Hallway                      Giant Electromagnet (Death)
G: Holodeck                     Prime Number
H: Stables                      Astrolabe
I: Passages                     Candy Cane
"""


def solution(data: List[int]) -> int:
    os.system("clear")
    computer = IntcodeComputer(data)
    instructions = [
        "north",
        "take wreath",
        "east",
        "east",
        "east",
        "take weather machine",
        "west",
        "west",
        "west",
        "south",
        "south",
        "west",
        "take prime number",
        "west",
        "take astrolabe",
        "east",
        "east",
        "south",
        "take candy cane",
        "north",
        "north",
        "east",
        "take food ration",
        "south",
        "east",
        "south",
        "take hypercube",
        "east",
        "take space law space brochure",
        "north",
    ]

    # Gather all non-killing items and get to the security checkpoint
    for instruction in instructions:
        computer.send_long(instruction)

    items = [
        "candy cane",
        "wreath",
        "hypercube",
        "food ration",
        "weather machine",
        "space law space brochure",
        "prime number",
        "astrolabe",
    ]

    # Drop all items to begin iterations
    for item in items:
        computer.send_long(f"drop {item}")

    computer.run_until_blocked()
    computer.readlines()

    while not computer.halted:
        computer.run_until_blocked()
        for item_count in range(1, len(items) + 1):
            print(f"Trying combinations of {item_count} items")
            for combination in combinations(items, item_count):

                # Pick up all items in the combination
                for item in combination:
                    computer.send_long(f"take {item}")
                computer.run_until_blocked()
                computer.stdout.clear()

                # Make attempt
                computer.send_long("west")
                computer.run_until_blocked()
                lines = list(computer.readlines())

                # Check for success
                if not any("heavier" in line or "lighter" in line for line in lines):
                    return "\n".join(lines[8:])

                # Drop up all items in the combination
                for item in combination:
                    computer.send_long(f"drop {item}")
                computer.run_until_blocked()
                computer.stdout.clear()


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().split(",")))
    print(f"{solution(data)}")
