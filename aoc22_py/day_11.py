"""The solution to day 11."""
from .aoc_helper import Solution
from typing import Callable


def parse_prefixed_int(raw: str, prefix: str) -> int:
    return int(raw.strip().removeprefix(prefix).strip())


def parse_operand(raw: str) -> Callable[[int], int]:
    if raw == "old":
        return lambda x: x
    value = int(raw)
    return lambda _: value


def parse_operator(raw: str) -> Callable[[int, int], int]:
    if raw == "+":
        return lambda a, b: a + b
    if raw == "*":
        return lambda a, b: a * b
    raise ValueError(f"Unknown operator: {raw}")


def parse_operation(raw: str) -> Callable[[int], int]:
    a, op, b = raw.removeprefix("new =").strip().split()
    a = parse_operand(a)
    b = parse_operand(b)
    op = parse_operator(op)
    return lambda x: op(a(x), b(x))


def inspect_operation(operation: Callable[[int], int]) -> str:
    assert operation.__closure__ is not None
    a = b = op = None
    for name, cellvar in zip(operation.__code__.co_freevars, operation.__closure__):
        if name == "a":
            a = inspect_operand(cellvar.cell_contents)
        elif name == "b":
            b = inspect_operand(cellvar.cell_contents)
        elif name == "op":
            op = inspect_operator(cellvar.cell_contents)
        else:
            raise ValueError(f"Unknown free variable: {name}")
    return f"new = {a} {op} {b}"


def inspect_operand(operand: Callable[[int], int]) -> str:
    if clo := operand.__closure__:
        return str(clo[0].cell_contents)
    return "old"


def inspect_operator(operator: Callable[[int, int], int]) -> str:
    import dis
    for instr in dis.get_instructions(operator):
        if instr.opname == "BINARY_ADD":
            return "+"
        if instr.opname == "BINARY_MULTIPLY":
            return "*"
    raise ValueError("Unknown operator")


class Monkey:
    @classmethod
    def from_raw_spec(cls, lines: list[str], monkeys: "Monkeys"):
        items = operation = test_divisor = if_true = if_false = None
        for line in lines:
            if not line.strip():
                continue
            field, raw_value = line.strip().split(":")
            match field.strip().lower():
                case "starting items":
                    items = [int(item.strip()) for item in raw_value.split(",")]
                case "operation":
                    operation = parse_operation(raw_value.strip())
                case "test":
                    test_divisor = parse_prefixed_int(raw_value, "divisible by")
                case "if true":
                    if_true = parse_prefixed_int(raw_value, "throw to monkey")
                case "if false":
                    if_false = parse_prefixed_int(raw_value, "throw to monkey")
                case _:
                    raise ValueError(f"Unknown field: {field}")
        assert items is not None
        assert operation is not None
        assert test_divisor is not None
        assert if_true is not None
        assert if_false is not None
        return cls(items=items, operation=operation, test_divisor=test_divisor, if_true=if_true, if_false=if_false, monkeys=monkeys)

    def __init__(self, *, items: list[int], operation: Callable[[int], int], test_divisor: int, if_true: int, if_false: int, monkeys: "Monkeys"):
        self.items = items
        self.operation = operation
        self.test_divisor = test_divisor
        self.if_true = if_true
        self.if_false = if_false
        self.monkeys = monkeys
        self.activity = 0

    def take_turn(self):
        for item in self.items:
            self.activity += 1
            item = self.operation(item)
            item = self.monkeys.regulate_worry(item)
            if item % self.test_divisor == 0:
                pass_to = self.if_true
            else:
                pass_to = self.if_false
            self.monkeys.give_monkey(monkey=pass_to, item=item)
        self.items = []

    def reset(self, monkeys: "Monkeys") -> "Monkey":
        return Monkey(
            items=list(self.items),
            operation=self.operation,
            test_divisor=self.test_divisor,
            if_true=self.if_true,
            if_false=self.if_false,
            monkeys=monkeys,
        )

    def __repr__(self):
        op = inspect_operation(self.operation)
        return f"Monkey(items={self.items}, operation={op!r}, test_divisor={self.test_divisor}, if_true={self.if_true}, if_false={self.if_false})"


class Monkeys:
    @classmethod
    def parse_monkeys(cls, raw: str) -> "Monkeys":
        monkeys = cls()
        monkey_buffer: list[str] = []
        for line in raw.split("\n"):
            if line.startswith(" "):
                monkey_buffer.append(line)
            elif monkey_buffer:
                monkeys.add_monkey(Monkey.from_raw_spec(monkey_buffer, monkeys))
                monkey_buffer = []
        if monkey_buffer:
            monkeys.add_monkey(Monkey.from_raw_spec(monkey_buffer, monkeys))
        return monkeys

    def __init__(self):
        self.monkeys: list[Monkey] = []
        self.modulo = 1
        self.worry_divisor = 1

    def add_monkey(self, monkey: Monkey):
        self.monkeys.append(monkey)
        self.modulo *= monkey.test_divisor

    def give_monkey(self, *, monkey: int, item: int):
        self.monkeys[monkey].items.append(item)

    def run_rounds(self, n: int = 1):
        for _ in range(n):
            for monkey in self.monkeys:
                monkey.take_turn()

    def regulate_worry(self, worry: int) -> int:
        return (worry // self.worry_divisor) % self.modulo

    def reset(self) -> "Monkeys":
        new = Monkeys()
        for monkey in self.monkeys:
            new.add_monkey(monkey.reset(new))
        return new

    @property
    def monkey_business(self) -> int:
        activity = sorted(monkey.activity for monkey in self.monkeys)
        return activity[-1] * activity[-2]


class Day(Solution):
    """The solution to day 11."""

    day = 11

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.monkeys = Monkeys.parse_monkeys(raw)

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        monkeys = self.monkeys.reset()
        monkeys.worry_divisor = 3
        monkeys.run_rounds(20)
        return monkeys.monkey_business

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        monkeys = self.monkeys.reset()
        monkeys.run_rounds(10_000)
        return monkeys.monkey_business


if __name__ == "__main__":
    Day.submit()
