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

    def evaluate_with(self, left: int, right: int) -> int:
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
    def evaluate(cls, name: str, monkeys: dict[str, int | Operation], *, unknown: str | None) -> int | None:
        """Evaluate an operand, returning None if it is unknown."""
        if name == unknown:
            return None
        match monkeys[name]:
            case int() as value:
                return value
            case Operation() as op:
                left = cls.evaluate(op.left, monkeys, unknown=unknown)
                right = cls.evaluate(op.right, monkeys, unknown=unknown)
                return left and right and op.evaluate_with(left, right)


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
        value = Operation.evaluate("root", self.monkeys, unknown=None)
        assert value is not None   # We didn't give an unknown.
        return value

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        root = self.monkeys["root"]
        assert isinstance(root, Operation)
        op = Operation(root.left, root.right, "-")
        target = 0
        while isinstance(op, Operation):
            left = op.evaluate(op.left, self.monkeys, unknown="humn")
            right = op.evaluate(op.right, self.monkeys, unknown="humn")
            match left, right:
                case int(left), None:
                    target = op.solve_for_right(target, left)
                    name = op.right
                case None, int(right):
                    target = op.solve_for_left(target, right)
                    name = op.left
                case _:
                    raise ValueError("Expected exactly one unknown.")
            op = self.monkeys.get(name)
        return target


if __name__ == "__main__":
    Day.submit()
