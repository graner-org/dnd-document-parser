from common.utils.constants import StatBlock, Sense


def _capital_first_letter(s: str) -> str:
    return s.upper()[0]


def stat_block_to_json(stat_block: StatBlock, source_book: str) -> dict:
    json = {}
    json["source"] = source_book
    json["page"] = 0
    json["name"] = stat_block.name
    json["size"] = [_capital_first_letter(stat_block.size.value)]
    if stat_block.creature_type.subtype:
        json["type"] = {
            "type": stat_block.creature_type.type,
            "tags": [stat_block.creature_type.subtype],
        }
    else:
        json["type"] = stat_block.creature_type.type
    json["alignment"] = [
        _capital_first_letter(align_part) for align_part in stat_block.alignment.split()
    ]
    if stat_block.ac.source:
        json["ac"] = [{"ac": stat_block.ac.ac, "from": [stat_block.ac.source]}]
    else:
        json["ac"] = [stat_block.ac.ac]
    json["hp"] = {"average": stat_block.hp.avg, "formula": stat_block.hp.formula}
    json["speed"] = {speed.mode: speed.value for speed in stat_block.speed}
    for stat, value in stat_block.stats.items():
        json[stat.value] = value
    if stat_block.saves:
        json["save"] = {
            stat.value: f"+{value}" for stat, value in stat_block.saves.items()
        }
    if stat_block.skills:
        json["skill"] = {
            skill: f"+{value}" for skill, value in stat_block.skills.items()
        }
    if [sense for sense in stat_block.senses if sense != Sense.passive_perc]:
        json["senses"] = [
            f"{sense.value} {range_} ft."
            if sense != Sense.blindsight_only
            else f"blindsight {range_} ft. (blind beyond this radius)"
            for sense, range_ in stat_block.senses.items()
            if sense != Sense.passive_perc
        ]
    json["passive"] = stat_block.senses[Sense.passive_perc]
    if stat_block.resistances:
        normal_res = [
            res.type.value for res in stat_block.resistances if not res.exception
        ]
        dmg_exceptions = {
            res.exception for res in stat_block.resistances if res.exception
        }
        restricted_res = {
            exception: [
                res.type.value
                for res in stat_block.resistances
                if res.exception == exception
            ]
            for exception in dmg_exceptions
        }
        json["resists"] = normal_res + [
            {
                "resist": types,
                "note": exception.value,
                "cond": True,
            }
            for exception, types in restricted_res.items()
        ]
    if stat_block.dmg_immunities:
        normal_imm = [
            imm.type.value for imm in stat_block.dmg_immunities if not imm.exception
        ]
        dmg_exceptions = {
            imm.exception for imm in stat_block.dmg_immunities if imm.exception
        }
        restricted_imm = {
            exception: [
                imm.type.value
                for imm in stat_block.dmg_immunities
                if imm.exception == exception
            ]
            for exception in dmg_exceptions
        }
        json["immune"] = normal_imm + [
            {
                "immune": types,
                "note": exception.value,
                "cond": True,
            }
            for exception, types in restricted_imm.items()
        ]
    if stat_block.cond_immunities:
        json["conditionImmune"] = [cond.value for cond in stat_block.cond_immunities]
    json["languages"] = [
        lang.lang.capitalize() if lang.range == -1 else f"{lang.lang} {lang.range} ft."
        for lang in stat_block.languages
    ]
    json["cr"] = str(stat_block.cr)
    if stat_block.abilities:
        json["trait"] = [ability.format() for ability in stat_block.abilities]
    if stat_block.actions:
        json["action"] = [action.format() for action in stat_block.actions]
    if stat_block.legendary_actions:
        if stat_block.legendary_actions.actions_per_turn != 3:
            json["legendaryActions"] = stat_block.legendary_actions.actions_per_turn
        json["legendary"] = [
            action.format() for action in stat_block.legendary_actions.actions
        ]
    if stat_block.bonus_actions:
        json["bonus"] = [action.format() for action in stat_block.bonus_actions]
    if stat_block.spellcasting or stat_block.innate_spellcasting:
        # TODO: implement format for these
        innate_spellcasting = (
            [stat_block.innate_spellcasting.format(stat_block.name)]
            if stat_block.innate_spellcasting
            else []
        )
        spellcasting = (
            [stat_block.spellcasting.format(stat_block.name)]
            if stat_block.spellcasting
            else []
        )
        json["spellcasting"] = innate_spellcasting + spellcasting
    return json
