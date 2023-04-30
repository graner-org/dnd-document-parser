from common.utils.constants import Paragraph, SectionType
from typing import Any, List
import pandas as pd


def concat_lists(list_: List[List[Any]]) -> List[Any]:
    tmp: List[Any] = list()
    for li in list_:
        tmp.extend(li)
    return tmp


def _get_lang(header: str) -> str:
    return header.split(" ")[2].split("=")[1].replace('"', "")


def _get_font(header: str) -> str:
    return header.split(" ")[3].split('"')[1]


def parse_paragraph(header: str, body: List[str]) -> List[Paragraph]:
    font = _get_font(header)
    return [
        Paragraph(
            lang=_get_lang(header),
            fontname=font.split("+")[1],
            fontsize=font.split("+")[0],
            body=b.strip(),
        )
        for b in body
    ]


def section_type(section: pd.DataFrame) -> SectionType:
    try:
        if section.fontname.tolist()[1] == "ACaslonPro-Italic":
            return SectionType.stat_block
        return SectionType.description
    except:
        return SectionType.description
