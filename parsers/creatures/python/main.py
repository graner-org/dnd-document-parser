from pathlib import Path
from sources.book_of_fiends.parse_pipeline import run as parse_book_of_fiends


book_of_fiends_path = (
    Path(__file__).parent.parent.parent.parent
    / "documents"
    / "book-of-fiends"
    / "demons-devils.html"
)


def main():
    print(book_of_fiends_path)
    parse_book_of_fiends(book_of_fiends_path)


if __name__ == "__main__":
    main()
