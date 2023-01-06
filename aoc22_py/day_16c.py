"""The solution to day 16."""
from __future__ import annotations

import re
from dataclasses import dataclass
from collections import deque
from typing import Iterable

from .aoc_helper import Solution


@dataclass(frozen=True)
class Valve:
    """Information about a valve"""

    # The valve's two-letter code.
    name: str

    # The amount of pressure released per turn the valve is open.
    flow_rate: int

    # The shortest distance to each valve from this valve.
    distances: dict[str, int]


@dataclass
class Agent:
    """An agent (either 'you' or the trained elephant)."""

    # The valve the agent is currently at.
    valve: Valve

    # The number of turns the agent has left.
    turns: int


class Day(Solution):
    """The solution to day 16."""

    day = 16

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        # { valve: (flow_rate, [tunnels]) }
        self.valves: dict[str, tuple[int, list[str]]] = {}
        for line in raw.splitlines():
            match = re.match(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (..(?:, ..)*)", line)
            if match:
                valve, flow_rate, tunnels = match.groups()
                self.valves[valve] = (int(flow_rate), tunnels.split(", "))
            else:
                raise ValueError(f"Invalid input: {line}")

    def distances(self, valve: str) -> Valve:
        """Get the minimum distance to each valve from the given valve and produce a Valve object."""
        distances: dict[str, int] = {valve: 0}
        queue = deque([valve])
        while queue:
            current_valve = queue.popleft()
            for tunnel in self.valves[current_valve][1]:
                if tunnel not in distances:
                    distances[tunnel] = distances[current_valve] + 1
                    queue.append(tunnel)
        return Valve(name=valve, flow_rate=self.valves[valve][0], distances=distances)

    def max_pressure_release(self, agents: list[Agent], closed_valves: list[Valve]) -> int:
        """Get the maximum pressure release from opening the given valves in any order."""
        self.count += 1
        max_release = 0
        for valve in closed_valves:
            for idx, agent in enumerate(agents):
                turns = agent.turns - agent.valve.distances[valve.name] - 1
                if turns <= 0:
                    continue
                new_agent = Agent(valve, turns)
                new_agents = [new_agent] + agents[idx + 1:]
                new_closed_valves = [i for i in closed_valves if i != valve]
                release = turns * valve.flow_rate + self.max_pressure_release(new_agents, new_closed_valves)
                if release > max_release:
                    max_release = release
        return max_release

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        valves: list[Valve] = []
        for valve, (flow_rate, _) in self.valves.items():
            if flow_rate == 0:
                continue
            valves.append(self.distances(valve))
        start_valve = self.distances("AA")
        me = Agent(start_valve, 30)
        self.count = 0
        return self.max_pressure_release([me], valves)

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        valves: list[Valve] = []
        for valve, (flow_rate, _) in self.valves.items():
            if flow_rate == 0:
                continue
            valves.append(self.distances(valve))
        start_valve = self.distances("AA")
        me = Agent(start_valve, 26)
        elephant = Agent(start_valve, 26)
        self.count = 0
        return self.max_pressure_release([me, elephant], valves)

    def partitions(self, valves: list[Valve]) -> Iterable[tuple[list[Valve], list[Valve]]]:
        """Get all possible partitions of the given list of valves."""
        max = 1 << len(valves)
        for bit_key in range(1 << len(valves)):
            print(f"Progress: {bit_key / max * 100:.2f}%", end="\r")
            partitions: tuple[list[Valve], list[Valve]] = ([], [])
            for i, valve in enumerate(valves):
                partitions[bit_key >> i & 1].append(valve)
            yield partitions


if __name__ == "__main__":
    Day.submit()
