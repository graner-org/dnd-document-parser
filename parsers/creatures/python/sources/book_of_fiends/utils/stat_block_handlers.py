from sources.book_of_fiends.utils.constants import STAT_NAMES, OPEN_PARAN, NON_NUM
from common.utils.constants import (
    StatBlock,
    Size,
    Paragraph,
    DamageType,
    Stat,
    Sense,
    Language,
    Condition,
    RestrictedDamageType,
    DamageException,
    Spellcasting,
    SpellLevel,
    Ability,
    LimitedUse,
    Recharge,
    Action,
    LegActions,
    InnateSpellCasting,
    ArmorClass,
    Speed,
    HitPoints,
    CreatureType,
)
import re


def handle_cr(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    cr = row.body.split()[1]
    # CR can be int or e.g. "1/4"
    stat_block.cr = int(cr) if "/" not in cr else cr
    return stat_block


def handle_stat_list(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    body_list = row.body.split()
    if body_list[0] == "Str":
        # We are on the row that lists stat names
        return stat_block
    # Odd indices are modifiers, e.g. "20 (+5) ..."
    stat_names = [Stat(stat) for stat in STAT_NAMES.split()]
    stats = [int(stat) for i, stat in enumerate(body_list) if i % 2 == 0]
    stat_block.stats = dict(zip(stat_names, stats))
    return stat_block


def handle_creature_type(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    body_list = row.body.split()
    size = Size(body_list[0].lower())
    if not size:
        # row is not creature type
        return stat_block
    stat_block.size = size

    # Main creature type and subtype, e.g. "fiend (devil)"
    creature_type_str = " ".join(body_list[1:3]).replace(",", "").lower()
    creature_type = CreatureType(
        type=creature_type_str.split(OPEN_PARAN)[0].strip(),
        subtype=" ".join(creature_type_str.split(OPEN_PARAN)[1:])
        .replace(")", "")
        .strip(),
    )
    stat_block.creature_type = creature_type

    alignment = " ".join(body_list[3:])
    stat_block.alignment = alignment
    return stat_block


def handle_spellcasting(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    if not stat_block.spellcasting:
        # Initialize spellcasting block
        body = row.body.lower().replace("spellcasting.", "")
        sentences = body.split(".")
        # Caster level is the only numeric substring in the first sentence.
        caster_level = _remove_non_num(sentences[0])
        # Caster ability is the last word before opening parantheses.
        ability = sentences[1].split(OPEN_PARAN)[0].split()[-1]
        # Parantheses in second sentence is "(spell save DC XX, +XX to hit...)"
        parans = sentences[1].split(OPEN_PARAN)[1].split(")")[0]
        save_dc = _remove_non_num(parans.split(",")[0])
        to_hit = -1
        if len(parans.split(",")) > 1:
            # There is to hit
            to_hit = _remove_non_num(parans.split(",")[1])
        caster_class = ""
        if "spells prepared" in sentences[2]:
            # There is a caster class
            caster_class = sentences[2].split("following")[1].split()[0]
        spellcasting = Spellcasting(
            caster_level=caster_level,
            ability=Stat(ability[:3]),
            dc=save_dc,
            to_hit=to_hit,
            casting_class=caster_class,
            spells=[],
        )
        stat_block.spellcasting = spellcasting
    else:
        body_list = row.body.lower().split(":")
        level_and_slots = body_list[0]
        level = 0
        slots = -1
        if "cantrip" not in level_and_slots:
            level = int(level_and_slots[0])
            slots = int(level_and_slots.split(OPEN_PARAN)[1][0])
        spells = [spell.strip() for spell in body_list[1].split(", ")]
        spellcasting = stat_block.spellcasting
        updated_spells = spellcasting.spells + [
            SpellLevel(level=level, slots=slots, spell_list=spells)
        ]
        spellcasting.spells = updated_spells
        stat_block.spellcasting = spellcasting
    return stat_block


def handle_innate_spellcasting(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    if not stat_block.innate_spellcasting:
        # Initialize spellcasting block
        body = row.body.lower().replace("innate spellcasting.", "")
        sentences = body.split(".")
        # Caster ability is the last word before opening parantheses.
        ability = sentences[0].split(OPEN_PARAN)[0].split()[-1]
        # Parantheses in second sentence is "(spell save DC XX, +XX to hit...)"
        parans = sentences[0].split(OPEN_PARAN)[1].split(")")[0]
        save_dc = _remove_non_num(parans.split(",")[0])
        to_hit = -1
        if len(parans.split(",")) > 1:
            # There is to hit
            to_hit = _remove_non_num(parans.split(",")[1])
        innate_spellcasting = InnateSpellCasting(
            ability=Stat(ability[:3]), dc=save_dc, to_hit=to_hit, spells=[]
        )
        stat_block.innate_spellcasting = innate_spellcasting
    else:
        body_list = row.body.lower().split(":")
        level_and_slots = body_list[0]
        level = 0
        slots = -1
        if "at will" not in level_and_slots:
            slots = int(level_and_slots[0])
        spells = [spell.strip() for spell in body_list[1].split(", ")]
        innate_spellcasting = stat_block.innate_spellcasting
        updated_spells = innate_spellcasting.spells + [
            SpellLevel(level=level, slots=slots, spell_list=spells)
        ]
        innate_spellcasting.spells = updated_spells
        stat_block.innate_spellcasting = innate_spellcasting
    return stat_block


def handle_ac(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    split_body = row.body.lower().replace("armor class", "").split()
    ac = int(split_body[0])
    source = " ".join(split_body[1:]).replace(OPEN_PARAN, "").replace(")", "")
    armor_class = ArmorClass(ac=ac, source=source)
    stat_block.ac = armor_class
    return stat_block


def handle_speed(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    def _str_to_speed(s: str) -> Speed:
        value = _remove_non_num(s)
        mode = s.replace("ft.", "").replace(str(value), "")
        mode = " ".join(mode.split()).replace("speed", "walk")
        return Speed(value, mode)

    raw_speeds = row.body.lower().split(",")
    speeds = [_str_to_speed(speed) for speed in raw_speeds]
    stat_block.speed = speeds
    return stat_block


def handle_hp(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    body = row.body.lower().replace("hit points", "").strip()
    avg = int(body.split()[0])
    formula = " ".join(body.split()[1:]).replace(OPEN_PARAN, "").replace(")", "")
    hp = HitPoints(avg, formula)
    stat_block.hp = hp
    return stat_block


def _remove_non_num(s: str) -> int:
    return int(re.sub(NON_NUM, "", s))


def handle_saves(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    # First two words are "Saving Throws"
    indexed_body_list = list(enumerate(row.body.lower().split()[2:]))
    stats = [Stat(stat) for i, stat in indexed_body_list if i % 2 == 0]
    saves = [_remove_non_num(save) for i, save in indexed_body_list if i % 2 != 0]
    save_dict = dict(zip(stats, saves))
    stat_block.saves = save_dict
    return stat_block


def handle_skills(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    # First word is "Skills"
    skill_list = row.body.lower().replace("skills", "").split(",")
    skill_dict = {
        # Modifier is last word in skill string
        " ".join(skill.split()[:-1]).strip(): _remove_non_num(skill)
        for skill in skill_list
    }
    stat_block.skills = skill_dict
    return stat_block


def _dam_type_helper(damage_type: str) -> list[RestrictedDamageType]:
    is_restricted_type = "from" in damage_type and (
        "attacks" in damage_type or "weapons" in damage_type
    )
    if not is_restricted_type:
        # Just a normal res., imm., or vuln.
        dam_type = DamageType(damage_type.strip())
        return [RestrictedDamageType(dam_type)]

    # Determine which damage types are affected.
    damage_types = [
        dam_type for dam_type in DamageType if dam_type.value in damage_type
    ]
    # Determine exception to res./imm./vuln.
    exception = (
        DamageException.silvered if "silver" in damage_type else DamageException.magical
    )

    return [
        RestrictedDamageType(type=dam_type, exception=exception)
        for dam_type in damage_types
    ]


def handle_dam_res(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    # Split into normal and restricted resistances.
    body_list = row.body.lower().replace("damage resistances", "").split(";")
    normal_resistances = body_list[0]
    restricted_resistances = body_list[1:]
    res_lists = [
        _dam_type_helper(dam_res)
        for dam_res in normal_resistances.split(",") + restricted_resistances
    ]
    # TODO: Handle resistance to non-magical physical damage etc.
    resistances = [dam_res for res_list in res_lists for dam_res in res_list]
    stat_block.resistances = stat_block.resistances + resistances
    return stat_block


def handle_dam_imm(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    # Split into normal and restricted immunities.
    body_list = row.body.lower().replace("damage immunities", "").split(";")
    normal_immunities = body_list[0]
    restricted_immunities = body_list[1:]
    imm_lists = [
        _dam_type_helper(dam_imm)
        for dam_imm in normal_immunities.split(",") + restricted_immunities
    ]
    # TODO: Handle immunity to non-magical physical damage etc.
    immunities = [dam_imm for imm_list in imm_lists for dam_imm in imm_list]
    stat_block.dmg_immunities = stat_block.dmg_immunities + immunities
    return stat_block


def handle_cond_imm(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    cond_list = row.body.lower().replace("condition immunities", "").split(",")
    cond_immunities = [Condition(cond.strip()) for cond in cond_list]
    stat_block.cond_immunities = cond_immunities
    return stat_block


def _sense_helper(sense: str) -> tuple[Sense, int]:
    sense = sense.lower().replace(" ft.", "")
    split_sense = sense.split()
    if "blind beyond this" in sense:
        return Sense.blindsight_only, split_sense[1]
    if "passive perception" in sense:
        return Sense.passive_perc, split_sense[-1]

    # Normal sense: "{sense} {range}"
    return Sense(split_sense[0]), split_sense[1]


def handle_senses(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    senses_list = row.body.replace("Senses ", "").split(",")
    senses = dict([_sense_helper(sense) for sense in senses_list])
    stat_block.senses = senses
    return stat_block


def _lang_helper(lang: str) -> Language:
    if "ft." not in lang:
        # Normal Language
        return Language(lang.strip())

    lang_fields = lang.replace("ft.", "").split()
    return Language(lang=" ".join(lang_fields[:-1]), range=lang_fields[-1])


def handle_langs(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    language_list = row.body.lower().replace("languages ", "").split(",")
    languages = [_lang_helper(lang) for lang in language_list]
    stat_block.languages = languages
    return stat_block


def handle_ability(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    if "bonus action" in row.body.lower():
        # Bonus actions are given as abilities.
        handle_bonus_action(stat_block, row)
        return stat_block
    body_list = row.body.split(".")
    header = body_list[0]
    body = ".".join(body_list[1:]).strip()
    name = header
    ability = Ability(name, body)
    if OPEN_PARAN in name:
        # Ability has limited uses
        name = name.split(OPEN_PARAN)[0].strip()
        if "/" in header:
            # e.g. (1/day)
            uses = header.split(OPEN_PARAN)[1].replace(")", "").split("/")
            limited_use = LimitedUse(
                charges=int(uses[0]), recharge=Recharge(uses[1].lower())
            )
        else:
            # e.g. (Recharge 5-6)
            uses = header.split(OPEN_PARAN)[1].lower().replace("recharge ", "")
            limited_use = LimitedUse(charges=int(uses[0]), recharge=Recharge.dice)
        ability.name = name
        ability.uses = limited_use
    abilities = stat_block.abilities
    abilities.append(ability)
    stat_block.abilities = abilities
    return stat_block


def handle_leg_action(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    if not stat_block.legendary_actions:
        # Initialize legendary actions block.
        actions_per_turn = _remove_non_num(row.body)
        leg_actions = LegActions(actions_per_turn=actions_per_turn)
        stat_block.legendary_actions = leg_actions
        return stat_block

    sentences = row.body.split(".")
    header = sentences[0].strip()
    name = header.split(OPEN_PARAN)[0].strip()
    cost = _remove_non_num(header.split(OPEN_PARAN)[1]) if OPEN_PARAN in header else 1
    limited_use = LimitedUse(charges=cost, recharge=Recharge.turn)
    body = ".".join(sentences[1:]).strip()
    action = Action(name=name, body=body, uses=limited_use)
    leg_actions = stat_block.legendary_actions
    leg_actions.actions.append(action)
    stat_block.legendary_actions = leg_actions
    return stat_block


def handle_bonus_action(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    sentences = row.body.split(".")
    header = sentences[0].strip()
    name = header
    body = ".".join(sentences[1:]).strip()
    action = Action(name, body)

    if OPEN_PARAN in name:
        # Action has limited uses
        name = name.split(OPEN_PARAN)[0].strip()
        if "/" in header:
            # e.g. (1/day)
            uses = header.split(OPEN_PARAN)[1].replace(")", "").split("/")
            limited_use = LimitedUse(
                charges=int(uses[0]), recharge=Recharge(uses[1].lower())
            )
        else:
            # e.g. (Recharge 5-6)
            uses = header.split(OPEN_PARAN)[1].lower().replace("recharge ", "")
            limited_use = LimitedUse(charges=int(uses[0]), recharge=Recharge.dice)
        action.name = name
        action.uses = limited_use

    bonus_actions = stat_block.bonus_actions
    bonus_actions.append(action)
    stat_block.bonus_actions = bonus_actions
    return stat_block


def handle_action_start(stat_block: StatBlock, row: Paragraph) -> StatBlock:
    sentences = row.body.split(".")
    header = sentences[0].strip()
    name = header
    body = ".".join(sentences[1:]).strip()
    action = Action(name, body)

    if OPEN_PARAN in name:
        # Action has limited uses
        name = name.split(OPEN_PARAN)[0].strip()
        if "/" in header:
            # e.g. (1/day)
            uses = header.split(OPEN_PARAN)[1].replace(")", "").split("/")
            limited_use = LimitedUse(
                charges=int(uses[0]), recharge=Recharge(uses[1].lower())
            )
        else:
            # e.g. (Recharge 5-6)
            uses = header.split(OPEN_PARAN)[1].lower().replace("recharge ", "")
            limited_use = LimitedUse(charges=int(uses[0]), recharge=Recharge.dice)
        action.name = name
        action.uses = limited_use

    actions = stat_block.actions
    actions.append(action)
    stat_block.actions = actions
    return stat_block
