BOSS_HP = 109
BOSS_DM = 8
BOSS_AC = 2

WEAPONS = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)]
ARMORS = [(0, 0, 0), (13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)]
RING_COMBINATIONS = [
    [0, 0, 0],
    [0, 0, 0],
    [25, 1, 0],
    [50, 2, 0],
    [100, 3, 0],
    [20, 0, 1],
    [40, 0, 2],
    [80, 0, 3],
]


def player_wins(player, boss):
    php, pdm, pac = player
    bhp, bdm, bac = boss

    player_strikes = (bhp // max(1, pdm - bac)) + (
        bhp % max(1, pdm - bac) > 0 and pdm - bac > 1
    )
    boss_strikes = (php // max(1, bdm - pac)) + (
        php % max(1, bdm - pac) > 0 and bdm - pac > 1
    )

    return player_strikes <= boss_strikes


def combinations():
    for weapon in WEAPONS:
        for armor in ARMORS:
            for ring1 in RING_COMBINATIONS:
                for ring2 in RING_COMBINATIONS:
                    if ring2 is ring1:
                        continue
                    yield map(sum, zip(weapon, armor, ring1, ring2))


def brute_force():
    min_cost = 1 << 31
    max_cost = 0
    boss = (BOSS_HP, BOSS_DM, BOSS_AC)
    for combination in combinations():
        cost, damage, armor = combination
        player = (100, damage, armor)
        if player_wins(player, boss):
            min_cost = min(min_cost, cost)
        else:
            max_cost = max(max_cost, cost)
    return min_cost, max_cost


print(brute_force())
