import json
from typing import Dict

BOSS_HIT_POINTS = 51
BOSS_DAMAGE = 9


def backtracker(
    player_hp: int,
    player_mana: int,
    boss_hp: int,
    boss_damage: int,
    hard_mode: bool = False,
) -> int:
    memo = {}

    def helper(
        player_hp: int,
        player_mana: int,
        boss_hp: int,
        effects: Dict[str, int],
        spent_mana: int = 0,
        player_turn: bool = True,
    ) -> int:
        memo_key = (
            player_hp,
            player_mana,
            boss_hp,
            boss_damage,
            json.dumps(sorted(effects.items())),
            player_turn,
        )
        if memo_key in memo:
            return memo[memo_key]

        if hard_mode and player_turn:
            player_hp -= 1
            if player_hp <= 0:
                memo[memo_key] = float("inf")
                return memo[memo_key]

        effects = effects.copy()
        effects["shield"] = max(effects["shield"] - 1, 0)
        if effects["poison"]:
            effects["poison"] -= 1
            boss_hp -= 3
        if effects["recharge"]:
            effects["recharge"] -= 1
            player_mana += 101

        if boss_hp <= 0:
            return spent_mana

        if not player_turn:
            player_hp -= boss_damage - (7 if effects["shield"] else 0)
            if player_hp <= 0:
                memo[memo_key] = float("inf")
            else:
                memo[memo_key] = helper(
                    player_hp, player_mana, boss_hp, effects, spent_mana, True
                )
        else:
            missile = float("inf")
            drain = float("inf")
            shield = float("inf")
            poison = float("inf")
            recharge = float("inf")
            if player_mana >= 53:
                missile = helper(
                    player_hp,
                    player_mana - 53,
                    boss_hp - 4,
                    effects,
                    spent_mana + 53,
                    False,
                )
            if player_mana >= 73:
                drain = helper(
                    player_hp + 2,
                    player_mana - 73,
                    boss_hp - 2,
                    effects,
                    spent_mana + 73,
                    False,
                )
            if player_mana >= 113 and not effects["shield"]:
                new_effects_map = effects.copy()
                new_effects_map["shield"] = 6
                shield = helper(
                    player_hp,
                    player_mana - 113,
                    boss_hp,
                    new_effects_map,
                    spent_mana + 113,
                    False,
                )
            if player_mana >= 173 and not effects["poison"]:
                new_effects_map = effects.copy()
                new_effects_map["poison"] = 6
                poison = helper(
                    player_hp,
                    player_mana - 173,
                    boss_hp,
                    new_effects_map,
                    spent_mana + 173,
                    False,
                )
            if player_mana >= 229 and not effects["recharge"]:
                new_effects_map = effects.copy()
                new_effects_map["recharge"] = 5
                recharge = helper(
                    player_hp,
                    player_mana - 229,
                    boss_hp,
                    new_effects_map,
                    spent_mana + 229,
                    False,
                )
            memo[memo_key] = min(missile, drain, shield, poison, recharge)
        return memo[memo_key]

    effects = {"shield": 0, "poison": 0, "recharge": 0}
    return helper(player_hp, player_mana, boss_hp, effects, 0, True)


def solution1() -> int:
    return backtracker(50, 500, BOSS_HIT_POINTS, BOSS_DAMAGE, hard_mode=False)


def solution2() -> int:
    return backtracker(50, 500, BOSS_HIT_POINTS, BOSS_DAMAGE, hard_mode=True)


if __name__ == "__main__":
    print(f"Solution 1: {solution1()}")
    print(f"Solution 2: {solution2()}")
