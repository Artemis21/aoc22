"""Set up a base file for each day."""
import pathlib

BASE_PATH = pathlib.Path(__file__).parent
SRC_DIR = BASE_PATH.parent
TEMPLATE_FILE = BASE_PATH / "day_template.txt"

with open(TEMPLATE_FILE) as f:
    template = f.read()

for day in range(1, 26):
    file = SRC_DIR / f"day_{day:02}.py"
    if file.exists():  # We don't overwrite what could be possible solutions.
        continue
    with open(file, "w") as f:
        f.write(template.format(day=day))
