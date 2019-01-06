from typing import Set, Dict, Optional, List, Tuple
import enum
import operator
import re
import sys


class Army(enum.Enum):
    IMMUNE = enum.auto()
    INFECTION = enum.auto()


class AttackType(enum.Enum):
    BLUDGEONING = 'bludgeoning'
    FIRE = 'fire'
    RADIATION = 'radiation'
    SLASHING = 'slashing'
    COLD = 'cold'


class Group:
    def __init__(self, *,
                 per_army_group_id: int,
                 army: Army,
                 per_unit_hp: int,
                 attack_dmg: int,
                 attack_type: AttackType,
                 initiative: int,
                 weaknesses: Set[AttackType],
                 immunities: Set[AttackType],
                 unit_count: int) -> None:
        self.per_army_group_id = per_army_group_id
        self.army = army
        self.per_unit_hp = per_unit_hp
        self.attack_dmg = attack_dmg
        self.attack_type = attack_type
        self.initiative = initiative
        self.weaknesses = weaknesses
        self.immunities = immunities
        self.hp = per_unit_hp * unit_count

    @property
    def is_alive(self) -> bool:
        return self.hp > 0

    @property
    def unit_count(self) -> int:
        return self.hp // self.per_unit_hp

    @property
    def effective_power(self) -> int:
        return self.unit_count * self.attack_dmg

    def takes(self, damage: int) -> None:
        self.hp -= damage // self.per_unit_hp * self.per_unit_hp
        self.hp = max(self.hp, 0)
        assert self.hp % self.per_unit_hp == 0

    def is_enemy_of(self, other: 'Group') -> bool:
        return self.army is not other.army

    def __repr__(self) -> str:
        return f'Group {self.per_army_group_id} contains {self.unit_count} units.'


def get_groups():
    def read_file(filepath):
        if 'test' not in sys.argv:
            with open(filepath) as f:
                for line in f.readlines():
                    yield line.rstrip()
        else:
            TEST_DATA = [
                'Immune System:',
                '17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2',
                '989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3',
                '',
                'Infection:',
                '801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1',
                '4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4',
            ]
            # TEST_DATA = [
            #     'Immune System:',
            #     '10 units each with 110 hit points with an attack that does 50 fire damage at initiative 9',
            #     '',
            #     '102 units each with 100 hit points with an attack that does 1 bludgeoning damage at initiative 1',
            #     '101 units each with 100 hit points with an attack that does 1 slashing damage at initiative 4',
            # ]

            for line in TEST_DATA:
                yield line

    def get_immunities_and_weaknesses(s: str) -> Tuple[Set[AttackType], Set[AttackType]]:
        immunities = set()
        weaknesses = set()
        if s:
            for s_group in s.split('; '):
                if s_group.startswith('weak to '):
                    for weakness in s_group[len('weak to '):].split(', '):
                        weaknesses.add(AttackType(weakness))
                elif s_group.startswith('immune to '):
                    for immunity in s_group[len('immune to '):].split(', '):
                        immunities.add(AttackType(immunity))
        return immunities, weaknesses

    prog = re.compile(
        r'^(?P<unit_count>\d+) units each with (?P<per_unit_hp>\d+) hit points '
        r'(?:\((?P<resistances>[\w\s,;]+)\) )?'
        r'with an attack that does (?P<attack_dmg>\d+) (?P<attack_type>\w+) damage '
        r'at initiative (?P<initiative>\d+)$'
    )
    keys = ['army', 'per_army_group_id', 'unit_count', 'per_unit_hp',
            'immunities', 'weaknesses', 'attack_dmg', 'attack_type', 'initiative']

    army = Army.IMMUNE
    groups = {}
    overall_group_count = 1
    per_army_group_count = 1

    for line in read_file('input.txt'):
        if line == '':
            # We reached the change in the input text
            army = Army.INFECTION
            per_army_group_count = 1
            continue
        elif line in ('Immune System:', 'Infection:'):
            continue
        match = prog.match(line)
        assert match is not None
        match_groups = match.groupdict()

        immunities, weaknesses = get_immunities_and_weaknesses(match_groups['resistances'])
        attack_type = AttackType(match_groups['attack_type'])
        data = {
            'army': army,
            'per_army_group_id': per_army_group_count,
            'immunities': immunities,
            'weaknesses': weaknesses,
            'attack_type': attack_type,
            **{key: int(value) for key, value in match_groups.items()
               if key in ['unit_count', 'per_unit_hp', 'attack_dmg', 'initiative']}
        }
        groups[overall_group_count] = Group(**data)
        per_army_group_count += 1
        overall_group_count += 1
    return groups


def target_selection_phase(groups: Dict[int, Group],
                           target_selection_order: List[int]
                           ) -> Dict[int, Optional[int]]:
    attacker_to_defender_map: Dict[int, Optional[int]] = {}
    for group_id in target_selection_order:
        current_group = groups[group_id]
        if not current_group.is_alive:
            continue

        current_damage = None
        current_target = None
        current_target_id = None
        for target_group_id, target_group in groups.items():
            # same team
            if not current_group.is_enemy_of(target_group):
                continue

            # target already dead
            if not target_group.is_alive:
                continue

            # target already being attacked
            if target_group_id in attacker_to_defender_map.values():
                continue

            # target is immune to our attack type
            if current_group.attack_type in target_group.immunities:
                continue

            # we can hit the target, calculate the estimated damage
            potential_damage = current_group.effective_power
            if current_group.attack_type in target_group.weaknesses:
                potential_damage *= 2

            if current_target is None:
                current_target = target_group
                current_target_id = target_group_id
                current_damage = potential_damage
                continue
            elif potential_damage > current_damage:
                current_target = target_group
                current_target_id = target_group_id
                current_damage = potential_damage
                continue
            elif (potential_damage == current_damage
                  and target_group.effective_power > current_target.effective_power):
                current_target = target_group
                current_target_id = target_group_id
                current_damage = potential_damage
                continue
            elif (potential_damage == current_damage
                  and target_group.effective_power == current_target.effective_power
                  and target_group.initiative > current_target.initiative):
                current_target = target_group
                current_target_id = target_group_id
                current_damage = potential_damage
                continue
        attacker_to_defender_map[group_id] = current_target_id
    return attacker_to_defender_map


def damage_phase(groups: Dict[int, Group],
                 damage_selection_order: List[int],
                 attacker_to_defender_map: Dict[int, int]) -> None:
    for attacker_id in damage_selection_order:
        attacker = groups[attacker_id]

        if not attacker.is_alive:
            continue

        defender_id = attacker_to_defender_map[attacker_id]

        if not defender_id:
            continue

        defender = groups[defender_id]
        multiplier = 2 if attacker.attack_type in defender.weaknesses else 1
        multiplier = 0 if attacker.attack_type in defender.immunities else multiplier
        damage = attacker.effective_power * multiplier
        defender.takes(damage)


def solution():
    groups = get_groups()

    armies = {
        Army.IMMUNE: [group for group in groups.values()
                      if group.army is Army.IMMUNE],
        Army.INFECTION: [group for group in groups.values()
                         if group.army is Army.INFECTION],
    }

    target_selection_criteria = operator.attrgetter('effective_power', 'initiative')
    target_selection_func = lambda group_id: target_selection_criteria(groups[group_id])

    damage_selection_criteria = operator.attrgetter('initiative')
    damage_selection_func = lambda group_id: damage_selection_criteria(groups[group_id])
    damage_selection_order = sorted(groups, key=damage_selection_func, reverse=True)

    while all(any(group.is_alive for group in army) for army in armies.values()):
        target_selection_order = sorted(groups, key=target_selection_func, reverse=True)
        attacker_to_defender_map = target_selection_phase(groups, target_selection_order)
        damage_phase(groups, damage_selection_order, attacker_to_defender_map)

    part1 = sum(group.unit_count for group_id, group in groups.items())
    return part1


print(solution())
