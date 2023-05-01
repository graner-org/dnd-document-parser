from pathlib import Path
from sources.book_of_fiends.setup import html_to_paragraphs
from sources.book_of_fiends.utils.stat_block import create_stat_block
from common.stat_block_print import stat_block_to_json
import json


current_path = Path(__file__).parent


def run(document_path: Path):
    sections_as_paragraphs = html_to_paragraphs(document_path)
    stat_blocks = []
    failures = []
    for i, section in enumerate(sections_as_paragraphs):
        try:
            stat_blocks.append(create_stat_block(section))
        except Exception as e:
            print(f"failed to create stat block {i}")
            failures.append(e)

    if failures:
        print(f"{len(failures)} of {len(sections_as_paragraphs)} failed.")
        raise failures[0]
    else:
        source_book = "bof"
        dict_stat_blocks = [
            stat_block_to_json(stat_block, source_book)
            for stat_block in stat_blocks
            if stat_block
        ]
        with open(current_path / "meta.json", "r") as f:
            meta_dict = json.loads(f.read())
        merged_dict = meta_dict | {"monster": dict_stat_blocks}
        json_stat_blocks = json.dumps(merged_dict, indent=4)
        with open("bof.json", "w") as f:
            f.write(json_stat_blocks)
        print(f"Successfully parsed {len(stat_blocks)} creatures.")
