"""The solution to day 20."""
from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 20."""

    day = 20

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.nums = list(map(int, raw.splitlines()))

    def mix_nums(self, nums: list[int], *, iters: int = 1) -> list[int]:
        """Mix a list of numbers a given number of times."""
        mixed = list(nums)
        length = len(nums)
        indexes = list(range(length))
        for _ in range(iters):
            for original_index in range(length):
                idx_from = indexes[original_index]
                num = mixed.pop(idx_from)
                idx_to = (idx_from + num) % (length - 1)
                mixed.insert(idx_to, num)
                for i, idx in enumerate(indexes):
                    if idx == idx_from:
                        indexes[i] = idx_to
                    elif idx_from < idx_to:
                        indexes[i] -= idx_from < idx <= idx_to
                    else:
                        indexes[i] += idx_to <= idx < idx_from
        return mixed

    def score_of(self, nums: list[int]) -> int:
        """Calculate the score of a list of numbers."""
        zero = nums.index(0)
        length = len(nums)
        return nums[(zero + 1000) % length] + nums[(zero + 2000) % length] + nums[(zero + 3000) % length]

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        return self.score_of(self.mix_nums(self.nums))

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        nums = [num * 811589153 for num in self.nums]
        return self.score_of(self.mix_nums(nums, iters=10))


if __name__ == "__main__":
    Day.submit()
