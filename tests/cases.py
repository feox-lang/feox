from pathlib import Path

CASES_DIR = (Path(__file__).parent / "cases").resolve()
CACHE_DIR = (Path(__file__).parent / "cache").resolve()
CACHE_DIR.mkdir(exist_ok=True)

FEOX_PATH = Path(__file__).parent.parent / "target"
release = FEOX_PATH / "release" / "feox"
debug = FEOX_PATH / "debug" / "feox"

FEOX_PATH = release if release.exists() else debug

CASES = sorted(
    (p.parent.resolve() for p in CASES_DIR.rglob("test.fe")),
    key=lambda p: str(p)
)