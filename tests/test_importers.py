import importlib
import sys
from pathlib import Path
from types import SimpleNamespace

import pytest

from scripts import import_epub


def create_epub(tmp_path: Path) -> Path:
    epub_path = tmp_path / 'sample.epub'
    import zipfile

    with zipfile.ZipFile(epub_path, 'w') as zf:
        zf.writestr('mimetype', 'application/epub+zip')
        zf.writestr('content.xhtml', '<html><body><h1>Título</h1><p>Hola mundo</p></body></html>')
    return epub_path


def test_extract_epub_text(tmp_path: Path) -> None:
    epub_path = create_epub(tmp_path)
    content = import_epub.extract_epub_text(epub_path)
    assert 'Hola mundo' in content


def test_extract_pdf_text(monkeypatch, tmp_path: Path) -> None:
    pdf_path = tmp_path / 'doc.pdf'
    pdf_path.write_bytes(b'%PDF-1.4\n1 0 obj<<>>endobj\n%%EOF')

    class FakePage:
        def extract_text(self) -> str:
            return 'Contenido PDF'

    class FakeReader:
        def __init__(self, path: str) -> None:
            self.pages = [FakePage()]

    fake_module = SimpleNamespace(PdfReader=FakeReader)
    monkeypatch.setitem(sys.modules, 'pypdf', fake_module)
    import scripts.import_pdf as import_pdf

    importlib.reload(import_pdf)
    text = import_pdf.extract_pdf_text(pdf_path)
    assert 'Contenido PDF' in text


def test_extract_pdf_text_missing_dependency(monkeypatch, tmp_path: Path) -> None:
    pdf_path = tmp_path / 'doc.pdf'
    pdf_path.write_bytes(b'%PDF-1.4\n1 0 obj<<>>endobj\n%%EOF')
    monkeypatch.setitem(sys.modules, 'pypdf', None)
    monkeypatch.setitem(sys.modules, 'PyPDF2', None)
    import scripts.import_pdf as import_pdf

    importlib.reload(import_pdf)
    with pytest.raises(import_pdf.DependencyError):
        import_pdf.extract_pdf_text(pdf_path)
