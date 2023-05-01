from pathlib import Path
import numpy as np
import pandas as pd
from sources.book_of_fiends.utils.helpers import (
    parse_paragraph,
    section_type,
    concat_lists,
    Paragraph,
    SectionType,
)


def html_to_paragraphs(document_path: Path) -> list[list[Paragraph]]:
    with open(document_path, "r") as raw:
        raw_lines = raw.readlines()

    raw_paragraphs = np.array_split(
        raw_lines, [i for i, line in enumerate(raw_lines) if line.startswith("<p ")]
    )[1:]

    paragraphs = [
        parse_paragraph(par[0], par[1:-1])
        for par in [np_par.tolist() for np_par in raw_paragraphs]
    ]

    df = pd.DataFrame(concat_lists(paragraphs))

    # lang="fy" are all footers
    df = df.query("lang != 'fy'").reset_index(drop=True)

    header_size = "PHJVTN"
    section_breakpoints = df[df.fontsize == header_size].index.to_numpy()
    sections = [pd.DataFrame(sec) for sec in np.array_split(df, section_breakpoints)]

    for i, sec in enumerate(sections):
        sec["id"] = i
        sec["type"] = section_type(sec).value
        sections[i] = sec

    sections_as_paragraphs = [
        [
            Paragraph(
                row[1]["lang"], row[1]["fontname"], row[1]["fontsize"], row[1]["body"]
            )
            for row in sec.iterrows()
        ]
        for sec in [
            sec
            for sec in sections[1:]
            if sec["type"].iloc[0] == SectionType.stat_block.value
        ]
    ]

    return sections_as_paragraphs
