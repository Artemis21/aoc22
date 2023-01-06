"""The solution to day 13."""
from __future__ import annotations

from functools import cmp_to_key
from typing import Callable, TypeVar, Union, ParamSpec

from .aoc_helper import Solution

Packet = list[Union[int, "Packet"]]
T = TypeVar("T")
P = ParamSpec("P")

def copy_packet(packet: Packet) -> Packet:
    """Return a copy of the packet."""
    return [copy_packet(item) if isinstance(item, list) else item for item in packet]


class ParseError(ValueError):
    """An error raised when parsing fails."""


class PacketParser:
    """A utility to parse a packet from input."""

    def __init__(self, raw: str):
        """Set up the parser."""
        self.raw = raw
        self.position = 0

    def parse(self) -> Packet:
        """Parse the full input as a packet, handling errors by displaying them nicely then exiting."""
        try:
            packet = self.packet()
            self.eoi()
            return packet
        except ParseError as e:
            print(f"Error parsing packet at position {self.position}: {e}")
            print(self.raw)
            print(" " * self.position + "^")
            exit(1)

    def try_(self, func: Callable[P, T], *args: P.args, **kwargs: P.kwargs) -> T | None:
        """Wrap a parser in error handling that rolls back state and returns none."""
        position = self.position
        try:
            return func(*args, **kwargs)
        except ParseError:
            self.position = position
            return None

    def wrap(self, while_parsing:str, func: Callable[P, T], *args: P.args, **kwargs: P.kwargs) -> T:
        """Wrap a parser in error handling that adds context."""
        try:
            return func(*args, **kwargs)
        except ParseError as e:
            raise ParseError(f"while parsing {while_parsing}: {e}")

    def one_of(self, *parsers: Callable[[], T]) -> T:
        """Try each parser in order until one succeeds."""
        errors: list[ParseError] = []
        position = self.position
        for parser in parsers:
            self.position = position
            try:
                return parser()
            except ParseError as e:
                errors.append(e)
        messages: list[str] = []
        for error in errors:
            message = str(error).replace("\n", "\n    ")
            messages.append(message)
        raise ParseError("\n" + "\n".join(messages))

    def packet(self) -> Packet:
        """Parse a packet or sub-packet."""
        self.wrap("packet", self.match, "[")
        if self.test("]"):
            return []
        packet: Packet = []
        while True:
            packet.append(self.wrap("packet", self.item))
            if not self.test(","):
                break
        self.wrap("packet", self.match, "]")
        return packet

    def item(self) -> Packet | int:
        """Parse an integer or sub-packet."""
        return self.wrap("packet or number", lambda: self.one_of(self.packet, self.int))

    def int(self) -> int:
        """Parse an integer."""
        num = self.wrap("number", self.digit)
        while True:
            digit = self.try_(self.digit)
            if digit is None:
                break
            num = num * 10 + digit
        return num

    def digit(self) -> int:
        """Parse a digit from the input or error."""
        char = self.char()
        if not char.isdigit():
            raise ParseError(f"expected digit, got {char}")
        return int(char)

    def test(self, expected: str) -> bool:
        """Test if the next character is a specific character.

        Consume it if it is.
        """
        if self.position >= len(self.raw):
            return False
        if self.raw[self.position] == expected:
            self.position += 1
            return True
        return False

    def match(self, expected: str) -> str:
        """Parse a specific character from the input or error."""
        if self.position >= len(self.raw):
            raise ParseError("unexpected end of input")
        char = self.raw[self.position]
        if char != expected:
            raise ParseError(f"expected {expected!r}, got {char!r}")
        self.position += 1
        return char

    def char(self) -> str:
        """Parse a character from the input or error."""
        if self.position >= len(self.raw):
            raise ParseError("unexpected end of input")
        char = self.raw[self.position]
        self.position += 1
        return char

    def eoi(self):
        """Parse the end of input or error."""
        if self.position != len(self.raw):
            raise ParseError("unexpected trailing input")

    def __str__(self) -> str:
        """Show how far through the input we are."""
        return self.raw[: self.position] + "|" + self.raw[self.position :]


def parse_packet(raw: str) -> Packet:
    """Parse a packet from a line of input."""
    parser = PacketParser(raw)
    packet = parser.parse()
    return packet


def compare_packets(packet_a: Packet | int, packet_b: Packet | int) -> int:
    """Give 1 if packet_a > packet_b, 0 if packet_a == packet_b and -1 if packet_a < packet_b."""
    if isinstance(packet_a, int):
        if isinstance(packet_b, int):
            return 1 if packet_a > packet_b else -1 if packet_a < packet_b else 0
        packet_a = [packet_a]
    if isinstance(packet_b, int):
        packet_b = [packet_b]
    packet_a, packet_b = copy_packet(packet_a), copy_packet(packet_b)
    while packet_a and packet_b:
        item_a, item_b = packet_a.pop(0), packet_b.pop(0)
        if cmp := compare_packets(item_a, item_b):
            return cmp
    a = compare_packets(len(packet_a), len(packet_b))
    return a


class Day(Solution):
    """The solution to day 13."""

    day = 13

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.pairs: list[tuple[Packet, Packet]] = []
        for line_pair in raw.split("\n\n"):
            line_a, line_b = line_pair.splitlines()
            self.pairs.append((parse_packet(line_a), parse_packet(line_b)))

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        total = 0
        for i, (packet_a, packet_b) in enumerate(self.pairs):
            if compare_packets(packet_a, packet_b) == -1:
                total += i + 1
        return total

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        packets = [packet for pair in self.pairs for packet in pair]
        packets.append([[2]])
        packets.append([[6]])
        packets.sort(key=cmp_to_key(compare_packets))
        a = packets.index([[2]]) + 1
        b = packets.index([[6]]) + 1
        print(a, b)
        return a * b


if __name__ == "__main__":
    Day.submit()
