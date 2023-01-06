"""The solution to day 21."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Literal

from .aoc_helper import Solution


@dataclass
class Operation:
    """An operation involving two variables."""

    left: str
    right: str
    op: Literal["+", "-", "*", "/"]

    def __call__(self, left: int, right: int) -> int:
        """Evaluate the operation."""
        match self.op:
            case "+":
                return left + right
            case "-":
                return left - right
            case "*":
                return left * right
            case "/":
                return left // right

    def solve_for_right(self, value: int, left: int) -> int:
        """Find the value of the right operand, given the outcome and left operand."""
        match self.op:
            case "+":
                return value - left
            case "-":
                return left - value
            case "*":
                return value // left
            case "/":
                return left // value

    def solve_for_left(self, value: int, right: int) -> int:
        """Find the value of the left operand, given the outcome and right operand."""
        match self.op:
            case "+":
                return value - right
            case "-":
                return value + right
            case "*":
                return value // right
            case "/":
                return value * right

    @classmethod
    def eval_operand(cls, name: str, monkeys: dict[str, int | Operation]) -> int | None:
        """Evaluate an operand, returning None if it is unknown."""
        if name not in monkeys:
            return None
        match monkeys[name]:
            case int() as value:
                return value
            case Operation() as op:
                left = cls.eval_operand(op.left, monkeys)
                right = cls.eval_operand(op.right, monkeys)
                return left and right and op(left, right)

    def solve(self, monkeys: dict[str, int | Operation], value: int) -> int:
        """Assuming that this operation expands to contain exactly one unknown, solve for it."""
        left = self.eval_operand(self.left, monkeys)
        right = self.eval_operand(self.right, monkeys)
        match left, right:
            case int(left), None:
                unknown_value = self.solve_for_right(value, left)
                unknown_name = self.right
            case None, int(right):
                unknown_value = self.solve_for_left(value, right)
                unknown_name = self.left
            case _:
                raise ValueError("Expected exactly one unknown.")
        if unknown_name not in monkeys:
            return unknown_value
        op = monkeys[unknown_name]
        assert isinstance(op, Operation)
        return op.solve(monkeys, unknown_value)


class Day(Solution):
    """The solution to day 21."""

    day = 21

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.monkeys: dict[str, int | Operation] = {}
        for line in raw.splitlines():
            name, value = line.split(": ")
            if value[0].isdigit():
                self.monkeys[name] = int(value)
            else:
                left, raw_op, right = value.split()
                assert raw_op in "+-*/"
                op: Literal["+", "-", "*", "/"] = raw_op  # type: ignore
                self.monkeys[name] = Operation(left, right, op)

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        value = Operation.eval_operand("root", self.monkeys)
        assert value is not None   # There are no unknowns in the input.
        return value

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        # Make `humn` the unknown variable.
        del self.monkeys["humn"]
        root = self.monkeys["root"]
        assert isinstance(root, Operation)
        # Solve `root.left - root.right = 0` for `humn`.
        return Operation(root.left, root.right, "-").solve(self.monkeys, 0)


if __name__ == "__main__":
    Day.submit()
