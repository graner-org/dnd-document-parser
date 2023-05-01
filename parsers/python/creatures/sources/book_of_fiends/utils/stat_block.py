from typing import Optional
from copy import deepcopy
from sources.book_of_fiends.utils.helpers import Paragraph
from common.utils.str_utils import multireplace
from sources.book_of_fiends.utils.stat_block_handlers import (
    handle_action_start,
    handle_hp,
    handle_speed,
    handle_creature_type,
    handle_ac,
    handle_innate_spellcasting,
    handle_cr,
    handle_spellcasting,
    handle_stat_list,
    handle_saves,
    handle_skills,
    handle_dam_res,
    handle_dam_imm,
    handle_cond_imm,
    handle_senses,
    handle_langs,
    handle_ability,
    handle_leg_action,
)
from common.utils.constants import (
    RowType,
    Size,
    StatBlock,
)
from sources.book_of_fiends.utils.constants import NAME_FONT, STAT_NAMES, translations


def carry_type_forward(rows: list[Paragraph], types: list[RowType]) -> list[RowType]:
    stat_i = types.index(RowType.stat_list)
    sense_i = types.index(RowType.senses)
    lang_i = types.index(RowType.langs)
    cr_i = types.index(RowType.cr)
    cond_imm_i = types.index(RowType.cond_imm) if RowType.cond_imm in types else sense_i
    dam_imm_i = types.index(RowType.dam_imm) if RowType.dam_imm in types else cond_imm_i
    dam_res_i = types.index(RowType.dam_res) if RowType.dam_res in types else dam_imm_i
    action_i = types.index(RowType.action_start)
    innate_spellcasting_i = (
        types.index(RowType.innate_spellcasting)
        if RowType.innate_spellcasting in types
        else action_i
    )
    spellcasting_i = (
        types.index(RowType.spellcasting) if RowType.spellcasting in types else action_i
    )
    leg_action_i = (
        types.index(RowType.leg_action_start)
        if RowType.leg_action_start in types
        else len(types)
    )
    innate_spellcasting_stop = (
        innate_spellcasting_i
        + 2
        + [
            i
            for i, row in enumerate(rows[innate_spellcasting_i + 2 :])
            if not (
                row.body.lower().startswith("at will")
                or row.body[1:].lower().startswith("/day each")
            )
        ][0]
        if RowType.innate_spellcasting in types
        else action_i
    )
    stat_range = range(stat_i, stat_i + 2)
    dam_res_range = range(dam_res_i, dam_imm_i)
    dam_imm_range = range(dam_imm_i, cond_imm_i)
    sense_range = range(sense_i, lang_i)
    innate_spellcasting_range = range(innate_spellcasting_i, innate_spellcasting_stop)
    ability_range = range(cr_i + 1, spellcasting_i)
    spellcasting_range = range(spellcasting_i, action_i)
    action_range = range(action_i + 1, leg_action_i)

    for i in range(len(types)):
        match i:
            case _ as i if i in stat_range:
                types[i] = RowType.stat_list
            case _ as i if i in dam_res_range:
                types[i] = RowType.dam_res
            case _ as i if i in dam_imm_range:
                types[i] = RowType.dam_imm
            case _ as i if i in sense_range:
                types[i] = RowType.senses
            case _ as i if i in innate_spellcasting_range:
                types[i] = RowType.innate_spellcasting
            case _ as i if i in ability_range:
                types[i] = RowType.ability
            case _ as i if i in spellcasting_range:
                types[i] = RowType.spellcasting
            case _ as i if i in action_range:
                types[i] = RowType.action
            case _ as i if i > leg_action_i:
                types[i] = RowType.leg_action
    return types


def translate_rows(rows: list[Paragraph]) -> list[Paragraph]:
    def translate_body(row: Paragraph) -> Paragraph:
        row.body = multireplace(row.body, translations)
        return row

    return [translate_body(row) for row in rows]


def _body_merger(row_list: list[Paragraph], merge_str: str = " ") -> list[Paragraph]:
    if not row_list:
        return []
    merged_body = merge_str.join([row.body for row in row_list])
    result = deepcopy(row_list[0])
    result.body = merged_body
    return [result]


def _spell_merger(row_list: list[Paragraph]) -> list[Paragraph]:
    """Make sure that there is a single row for header and spell levels."""
    if not row_list:
        return []

    def is_spell_level(s):
        return s[0].isdigit() or s.lower().startswith("cantrip")

    split_indices = [
        i for i, row in enumerate(row_list) if is_spell_level(row.body)
    ] + [len(row_list)]
    split_ranges = [
        (split_indices[i], split_indices[i + 1]) for i in range(len(split_indices) - 1)
    ]
    header = _body_merger(row_list[: split_indices[0]])
    levels = [_body_merger(row_list[s:e])[0] for s, e in split_ranges]
    return header + levels


def _innate_spell_merger(row_list: list[Paragraph]) -> list[Paragraph]:
    """Make sure that there is a single row for header and spell levels."""
    if not row_list:
        return []

    def is_spell_level(s):
        return s[0].isdigit() or s.lower().startswith("at will")

    split_indices = [
        i for i, row in enumerate(row_list) if is_spell_level(row.body)
    ] + [len(row_list)]
    split_ranges = [
        (split_indices[i], split_indices[i + 1]) for i in range(len(split_indices) - 1)
    ]
    header = _body_merger(row_list[: split_indices[0]])
    levels = [_body_merger(row_list[s:e])[0] for s, e in split_ranges]
    return header + levels


def _ability_merger(row_list: list[Paragraph]) -> list[Paragraph]:
    if not row_list:
        return []

    def is_ability_start(s):
        return "." in " ".join(s.split()[:3]) or ")." in " ".join(s.split()[:6])

    def is_ability_end(s):
        return s.endswith(".")

    start_indices = [
        i for i, row in enumerate(row_list) if i == 0 or is_ability_start(row.body)
    ]
    end_indices = [
        i
        for i, row in enumerate(row_list)
        if i + 1 == len(row_list)
        or (is_ability_end(row.body) and i + 1 in start_indices)
    ]
    start_indices = [
        i for i in start_indices if {i - 1, i}.intersection(end_indices) or i == 0
    ]
    abilities = [
        _body_merger(row_list[s : e + 1])[0] for s, e in zip(start_indices, end_indices)
    ]
    return abilities


def merge_rows(
    rows: list[Paragraph], row_types: list[RowType]
) -> tuple[list[Paragraph], list[RowType]]:
    """Merge certain rows based on row type.

    Information other that body may be lost for merged rows.
    """
    row_type_handlers = {
        RowType.dam_res: _body_merger,
        RowType.dam_imm: _body_merger,
        RowType.senses: _body_merger,
        RowType.spellcasting: _spell_merger,
        RowType.innate_spellcasting: _innate_spell_merger,
        RowType.ability: _ability_merger,
        RowType.action: _ability_merger,
        RowType.leg_action: _ability_merger,
    }
    row_types_to_merge = row_type_handlers.keys()
    typed_rows = list(zip(rows, row_types))
    rows_to_merge = [row for row in typed_rows if row[1] in row_types_to_merge]
    rows_to_not_merge = [row for row in typed_rows if row[1] not in row_types_to_merge]
    # Use the appropriate merge handler to merge rows.
    merged_rows_by_type = {
        row_type: row_type_handlers[row_type](
            [row for row, type_ in rows_to_merge if type_ == row_type]
        )
        for row_type in row_types_to_merge
    }
    merged_rows_with_type = [
        (row, row_type)
        for row_type, row_list in merged_rows_by_type.items()
        for row in row_list
    ]

    merged_rows, merged_row_types = tuple(zip(*merged_rows_with_type))
    unmerged_rows, unmerged_row_types = tuple(zip(*rows_to_not_merge))
    return unmerged_rows + merged_rows, unmerged_row_types + merged_row_types


def create_stat_block(rows: list[Paragraph]) -> Optional[StatBlock]:
    stat_block = StatBlock()
    rows = translate_rows(rows)
    raw_row_types = [determine_row_type(row) for row in rows]
    if RowType.creature_type not in raw_row_types:
        return None
    row_types = carry_type_forward(rows, raw_row_types)
    rows, row_types = merge_rows(rows, row_types)
    rows_with_types = zip(rows, row_types)
    for row, row_type in rows_with_types:
        match row_type:
            case RowType.name:
                stat_block.name = row.body
            case RowType.creature_type:
                stat_block = handle_creature_type(stat_block, row)
            case RowType.ac:
                stat_block = handle_ac(stat_block, row)
            case RowType.hp:
                stat_block = handle_hp(stat_block, row)
            case RowType.speed:
                stat_block = handle_speed(stat_block, row)
            case RowType.stat_list:
                stat_block = handle_stat_list(stat_block, row)
            case RowType.saves:
                stat_block = handle_saves(stat_block, row)
            case RowType.skills:
                stat_block = handle_skills(stat_block, row)
            case RowType.dam_res:
                stat_block = handle_dam_res(stat_block, row)
            case RowType.dam_imm:
                stat_block = handle_dam_imm(stat_block, row)
            case RowType.cond_imm:
                stat_block = handle_cond_imm(stat_block, row)
            case RowType.senses:
                stat_block = handle_senses(stat_block, row)
            case RowType.langs:
                stat_block = handle_langs(stat_block, row)
            case RowType.cr:
                stat_block = handle_cr(stat_block, row)
            case RowType.ability:
                stat_block = handle_ability(stat_block, row)
            case RowType.innate_spellcasting:
                stat_block = handle_innate_spellcasting(stat_block, row)
            case RowType.spellcasting:
                stat_block = handle_spellcasting(stat_block, row)
            case RowType.action:
                stat_block = handle_action_start(stat_block, row)
            case RowType.leg_action:
                stat_block = handle_leg_action(stat_block, row)
            case _:
                pass
    return stat_block


def determine_row_type(row: Paragraph) -> RowType:
    if (row.fontname, row.fontsize) == NAME_FONT:
        return RowType.name
    body = row.body.lower()

    if body == STAT_NAMES:
        return RowType.stat_list

    match body:
        case RowType.action_start.value:
            return RowType.action_start
        case RowType.leg_action_start.value:
            return RowType.leg_action_start
        case _:
            pass

    body_words = body.split()
    first_word = body_words[0]
    first_2_words = " ".join(body_words[:2])
    sizes = [size.value for size in Size]

    match first_word:
        case _ as first_word if first_word in sizes:
            return RowType.creature_type
        case RowType.speed.value:
            return RowType.speed
        case RowType.senses.value:
            return RowType.senses
        case RowType.langs.value:
            return RowType.langs
        case RowType.cr.value:
            return RowType.cr
        case RowType.spellcasting.value:
            return RowType.spellcasting
        case RowType.skills.value:
            return RowType.skills
        case _:
            pass

    match first_2_words:
        case RowType.ac.value:
            return RowType.ac
        case RowType.hp.value:
            return RowType.hp
        case RowType.saves.value:
            return RowType.saves
        case RowType.innate_spellcasting.value:
            return RowType.innate_spellcasting
        case RowType.dam_res.value:
            return RowType.dam_res
        case RowType.dam_imm.value:
            return RowType.dam_imm
        case RowType.cond_imm.value:
            return RowType.cond_imm
        case _:
            pass

    return RowType.other
