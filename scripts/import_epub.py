#!/usr/bin/env python3
"""Extract plain text from an EPUB file."""

from __future__ import annotations

import argparse
import sys
import zipfile
from html.parser import HTMLParser
from pathlib import Path
from typing import Iterable


class _TextExtractor(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self._chunks: list[str] = []

    def handle_data(self, data: str) -> None:
        data = data.strip()
        if data:
            self._chunks.append(data)

    def get_text(self) -> str:
        return " ".join(self._chunks)


def html_to_text(markup: str) -> str:
    parser = _TextExtractor()
    parser.feed(markup)
    return parser.get_text()


def extract_epub_text(path: Path) -> str:
    if not path.exists():
        raise FileNotFoundError(path)
    with zipfile.ZipFile(path) as zf:
        texts: list[str] = []
        for name in zf.namelist():
            lower = name.lower()
            if lower.endswith(('.xhtml', '.html', '.htm')):
                try:
                    content = zf.read(name).decode('utf-8', errors='ignore')
                except KeyError:
                    continue
                texts.append(html_to_text(content))
        if not texts:
            raise ValueError('No se encontraron documentos legibles en el EPUB')
        return "\n\n".join(filter(None, texts))


def main(argv: Iterable[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--file', required=True, help='Ruta al archivo EPUB')
    args = parser.parse_args(argv)
    try:
        content = extract_epub_text(Path(args.file))
    except Exception as exc:  # noqa: BLE001
        print(str(exc), file=sys.stderr)
        return 1
    print(content)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
