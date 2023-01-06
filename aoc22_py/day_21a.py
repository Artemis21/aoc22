"""The solution to day 21."""
from __future__ import annotations

import operator
from dataclasses import dataclass
from typing import Callable

import sympy

from .aoc_helper import Solution


@dataclass
class Operation:
    """An operation involving two variables."""

    left: str
    right: str
    op: Callable[[int, int], int]

    @classmethod
    def parse(cls, raw: str) -> Operation:
        """Parse an operation from a string."""
        left, raw_op, right = raw.split()
        match raw_op:
            case "+":
                op = operator.add
            case "-":
                op = operator.sub
            case "*":
                op = operator.mul
            case "/":
                op = operator.truediv
            case _:
                raise ValueError(f"Unknown operator: {raw_op}")
        return cls(left, right, op)


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
                self.monkeys[name] = Operation.parse(value)

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        def evaluate(name: str):
            match self.monkeys[name]:
                case int() as value:
                    return value
                case Operation() as op:
                    return op.op(evaluate(op.left), evaluate(op.right))

        return int(evaluate("root"))

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        x = sympy.var("x")
        
        def evaluate(name: str):
            if name == "humn":
                return x
            match self.monkeys[name]:
                case int() as value:
                    return value
                case Operation() as op:
                    return op.op(evaluate(op.left), evaluate(op.right))

        root = self.monkeys["root"]
        left = evaluate(root.left)
        right = evaluate(root.right)
        return int(round(sympy.solve(left - right, x)[0]))


if __name__ == "__main__":
    Day.submit()
