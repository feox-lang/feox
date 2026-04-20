from pathlib import Path
import subprocess
try:
    from tqdm import tqdm
    print = tqdm.write
except ImportError:
    tqdm = None

CASES_DIR = Path(__file__).parent / "cases"
CACHE_DIR = Path(__file__).parent / "cache"
CACHE_DIR.mkdir(exist_ok=True)

cases = sorted(
    {f.stem for f in CASES_DIR.iterdir()}
)

for case in tqdm(cases, unit=" tests") if tqdm else cases:
    py_file = CASES_DIR / f"{case}.py"
    cache_file = CACHE_DIR / f"{case}.txt"

    result = subprocess.run(
        ["python3", py_file],
        capture_output=True, text=True
    )
    output = (result.stdout + result.stderr).strip()

    cache_file.write_text(output)

    print(f"Generated output for {case}")