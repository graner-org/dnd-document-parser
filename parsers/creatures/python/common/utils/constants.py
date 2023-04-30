from dataclasses import dataclass, field
from enum import Enum
from typing import Optional


@dataclass
class Paragraph:
    lang: str
    fontname: str
    fontsize: str
    body: str


class SectionType(Enum):
    description = "description"
    stat_block = "stat_block"


class RowType(Enum):
    name = "name"
    creature_type = "type"
    ac = "armor class"
    hp = "hit points"
    speed = "speed"
    stat_list = "stats"
    saves = "saving throws"
    skills = "skills"
    dam_res = "damage resistances"
    dam_imm = "damage immunities"
    cond_imm = "condition immunities"
    senses = "senses"
    langs = "languages"
    cr = "challenge"
    ability = "ability"
    innate_spellcasting = "innate spellcasting."
    spellcasting = "spellcasting."
    action_start = "actions"
    bonus_action = "bonus action"
    leg_action_start = "legendary actions"
    other = "other"
    unrecognized = "unrecognized"
    action = "action"
    leg_action = "legendary action"


class Stat(Enum):
    str = "str"
    dex = "dex"
    con = "con"
    int = "int"
    wis = "wis"
    cha = "cha"


class Size(Enum):
    tiny = "tiny"
    small = "small"
    medium = "medium"
    large = "large"
    huge = "huge"
    gargantuan = "gargantuan"


class DamageType(Enum):
    acid = "acid"
    bludgeoning = "bludgeoning"
    cold = "cold"
    fire = "fire"
    force = "force"
    lightning = "lightning"
    necrotic = "necrotic"
    piercing = "piercing"
    poison = "poison"
    psychic = "psychic"
    radiant = "radiant"
    slashing = "slashing"
    thunder = "thunder"


class DamageException(Enum):
    """Excepted damage sources that ignore resistance or immunity."""

    magical = "from nonmagical attacks"
    silvered = "from nonmagical attacks that aren't silvered"


@dataclass
class RestrictedDamageType:
    type: DamageType
    exception: Optional[DamageException] = None


class Sense(Enum):
    darkvision = "darkvision"
    truesight = "truesight"
    passive_perc = "passive perception"
    blindsight = "blindsight"
    blindsight_only = "blindsight only"


@dataclass
class Language:
    lang: str
    range: int = -1


class Condition(Enum):
    blinded = "blinded"
    charmed = "charmed"
    deafened = "deafened"
    exhaustion = "exhaustion"
    frightened = "frightened"
    grappled = "grappled"
    incapacitated = "incapacitated"
    invisible = "invisible"
    paralyzed = "paralyzed"
    petrified = "petrified"
    poisoned = "poisoned"
    prone = "prone"
    restrained = "restrained"
    stunned = "stunned"
    unconscious = "unconscious"


@dataclass
class SpellLevel:
    level: int
    slots: int
    spell_list: list[str]

    def format_spellcasting(self) -> dict:
        entry: dict[str, list[str] | int] = {"spells": self.spell_list}
        if self.level > 0:
            entry["slots"] = self.slots
        return {str(self.level): entry}

    def format_innate(self) -> dict:
        if self.slots == -1:
            return {"will": self.spell_list}
        return {"daily": {str(self.slots): self.spell_list}}


stat_to_str = {
    Stat.str: "Strength",
    Stat.dex: "Dexterity",
    Stat.con: "Constitution",
    Stat.int: "Intelligence",
    Stat.wis: "Wisdom",
    Stat.cha: "Charisma",
}


@dataclass
class Spellcasting:
    caster_level: int
    ability: Stat
    dc: int
    to_hit: int
    casting_class: str
    spells: list[SpellLevel]

    def format(self, name: str) -> dict:
        json = {}
        json["name"] = "Spellcasting"
        header = (
            f"The {name} is a {self.caster_level}th-level spellcaster. "
            + f"Its spellcasting ability is {stat_to_str[self.ability]} "
            + f"(spell save {self.dc}, {self.to_hit} to hit with spell attacks). "
            + f"The {name} has the following {self.casting_class} spells prepared:"
        )
        json["headerEntries"] = [header]
        spells = {}
        for level in self.spells:
            spells.update(level.format_spellcasting())
        json["spells"] = spells
        json["ability"] = self.ability.value
        return json


@dataclass
class InnateSpellCasting:
    ability: Stat
    dc: int
    to_hit: int
    spells: list[SpellLevel]  # level is not used, slots are uses per day per spell

    def format(self, name: str) -> dict:
        json = {}
        json["name"] = "Innate Spellcasting"
        header = (
            f"The {name}'s spellcasting ability is {stat_to_str[self.ability]} "
            + f"(spell save {self.dc}). "
            + f"The {name} can innately cast the following spells, requiring no material components:"
        )
        json["headerEntries"] = [header]
        json["ability"] = self.ability.value
        at_will_spells = [
            level.format_innate() for level in self.spells if level.slots == -1
        ]
        if at_will_spells:
            json.update(at_will_spells[0])
        daily_spells = [
            level.format_innate()["daily"] for level in self.spells if level.slots != -1
        ]
        if daily_spells:
            for entry in daily_spells:
                json["daily"] = json.get("daily", {}) | entry
        return json


class Recharge(Enum):
    day = "day"
    week = "week"
    long_rest = "long rest"
    short_rest = "short rest"
    turn = "turn"
    dice = "dice"


@dataclass
class LimitedUse:
    """If recharge type is dice, charges is what must be rolled on d6 to recharge."""

    charges: int
    recharge: Recharge = Recharge.day

    def format(self) -> str:
        if self.recharge == Recharge.dice:
            return f"Recharge {self.charges}" + ("-6" if self.charges != 6 else "")
        if self.recharge == Recharge.turn:
            return f"Costs {self.charges} Actions"
        return f"{self.charges}/{self.recharge.value}"


@dataclass
class Ability:
    name: str
    body: str
    uses: Optional[LimitedUse] = None

    def format(self) -> dict[str, str]:
        return {
            "name": self.name
            if not self.uses
            else f"{self.name} ({self.uses.format()})",
            "entries": [self.body],
        }


@dataclass
class Action:
    name: str
    body: str
    uses: Optional[LimitedUse] = None

    def format(self) -> dict[str, str]:
        return {
            "name": self.name
            if not self.uses
            else f"{self.name} ({self.uses.format()})",
            "entries": [self.body],
        }


@dataclass
class LegActions:
    """action.uses is charge consumption for action."""

    actions_per_turn: int
    actions: list[Action] = field(default_factory=list)


@dataclass
class ArmorClass:
    ac: int
    source: str


@dataclass
class Speed:
    value: int
    mode: str


@dataclass
class HitPoints:
    avg: int
    formula: str


@dataclass
class CreatureType:
    type: str
    subtype: str


@dataclass
class StatBlock:
    name: str = ""
    size: Size = None
    creature_type: CreatureType = None
    alignment: str = ""
    ac: ArmorClass = None
    hp: HitPoints = None
    speed: list[Speed] = field(default_factory=list)
    stats: dict[Stat, int] = field(default_factory=dict)
    saves: dict[Stat, int] = field(default_factory=dict)
    skills: dict[str, int] = field(default_factory=dict)
    resistances: list[RestrictedDamageType] = field(default_factory=list)
    dmg_immunities: list[RestrictedDamageType] = field(default_factory=list)
    cond_immunities: list[Condition] = field(default_factory=list)
    senses: dict[Sense, int] = field(default_factory=dict)
    languages: list[Language] = field(default_factory=list)
    cr: int | str = -1
    abilities: list[Ability] = field(default_factory=list)
    spellcasting: Optional[Spellcasting] = None
    innate_spellcasting: Optional[InnateSpellCasting] = None
    actions: list[Action] = field(default_factory=list)
    bonus_actions: list[Action] = field(default_factory=list)
    legendary_actions: Optional[LegActions] = None
