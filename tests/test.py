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

GREEN  = "\033[32m"
RED    = "\033[31m"
YELLOW = "\033[33m"
RESET  = "\033[0m"

FEOX_PATH = Path(__file__).parent.parent / "target" / "debug" / "feox"

cases = sorted(
    {f.stem for f in CASES_DIR.iterdir()}
)

for case in tqdm(cases, unit=" tests") if tqdm else cases:
    fe_file = CASES_DIR / f"{case}.fe"
    cache_file = CACHE_DIR / f"{case}.txt"

    result = subprocess.run(
        [FEOX_PATH, fe_file],
        capture_output=True, text=True
    )
    output = (result.stdout + result.stderr).strip()

    expected = cache_file.read_text()

    print("="*20)
    if output == expected:
        print(f"Test {case}: {GREEN}passed{RESET}")
    else:
        print(f"Test {case}: {RED}failed{RESET}")
        if len(output) < 50:
            print(f"    Output: {output}")
            print(f"    Expected: {expected}")