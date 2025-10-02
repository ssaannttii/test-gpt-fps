#!/usr/bin/env python3
"""Extract text from a PDF document using pypdf/PyPDF2."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import Iterable


class DependencyError(RuntimeError):
    """Raised when the PDF dependency is missing."""


def _get_reader():
    try:
        from pypdf import PdfReader  # type: ignore
    except ImportError:
        try:
            from PyPDF2 import PdfReader  # type: ignore
        except ImportError as exc:  # noqa: F401
            raise DependencyError(
                'Instala la librería "pypdf" para habilitar la importación de PDF.'
            ) from exc
    return PdfReader


def extract_pdf_text(path: Path) -> str:
    if not path.exists():
        raise FileNotFoundError(path)
    PdfReader = _get_reader()
    reader = PdfReader(str(path))
    parts: list[str] = []
    for page in getattr(reader, 'pages', []):
        text = page.extract_text() if hasattr(page, 'extract_text') else ''
        if text:
            parts.append(text.strip())
    if not parts:
        raise ValueError('No se pudo extraer texto del PDF')
    return '\n\n'.join(parts)


def main(argv: Iterable[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--file', required=True, help='Ruta al archivo PDF')
    args = parser.parse_args(argv)
    try:
        content = extract_pdf_text(Path(args.file))
    except DependencyError as exc:
        print(str(exc), file=sys.stderr)
        return 2
    except Exception as exc:  # noqa: BLE001
        print(str(exc), file=sys.stderr)
        return 1
    print(content)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
