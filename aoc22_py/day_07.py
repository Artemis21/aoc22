"""The solution to day 7."""
import collections

from .aoc_helper import Solution


class Folder:
    def __init__(self, folders: dict[str, "Folder"]):
        self.folders = folders
        self.file_size = 0
        self.dirs: list[str] = []

    def add_file(self, size: int):
        self.file_size += size

    def add_dir(self, folder: str):
        self.dirs.append(folder)

    def size(self) ->int:
        total = self.file_size
        for dir in self.dirs:
            folder = self.folders[dir]
            total += folder.size()
        return total


class Day(Solution):
    """The solution to day 7."""

    day = 7

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.lines = raw.split("\n")
        self.folders: dict[str, Folder] = collections.defaultdict(lambda: Folder(self.folders))
        cwd: list[str] = []
        lines = list(self.lines)
        while lines:
            line = lines.pop(0)
            if line == "$ cd ..":
                cwd.pop()
            elif line.startswith("$ cd"):
                cwd.append(line.split(" ")[2])
            elif line.startswith("$ ls"):
                while lines:
                    line = lines.pop(0)
                    if line.startswith("$") or not line.strip():
                        lines.insert(0, line)
                        break
                    if line.startswith("dir"):
                        name = line.split(" ")[1]
                        self.folders["/".join(cwd)].add_dir("/".join([*cwd, name]))
                    else:
                        self.folders["/".join(cwd)].add_file(int(line.split()[0]))

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        total = 0
        for folder in list(self.folders):
            size = self.folders[folder].size()
            if size <= 100000:
                total += size
        return total

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        free = 70_000_000 - self.folders["/"].size()
        sizes = [folder.size() for folder in self.folders.values()]
        return min(size for size in sizes if (free + size) >= 30_000_000)


if __name__ == "__main__":
    Day.submit()
