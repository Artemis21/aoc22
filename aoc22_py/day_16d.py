"""The solution to day 16."""
from __future__ import annotations

import re
from dataclasses import dataclass
from collections import deque
from heapq import heappush, heappop
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


@dataclass(frozen=True)
class Agent:
    """An agent (either 'you' or the trained elephant)."""

    # The valve the agent is currently at.
    valve: Valve

    # The number of turns the agent has left.
    turns: int


@dataclass(frozen=True)
class State:
    """A state of the simulation."""

    # The total pressure that would be released if no more valves were opened.
    total_released: int

    # The valves that are currently still closed.
    closed_valves: list[Valve]

    # The current state of each agent.
    agents: list[Agent]

    @property
    def lower_bound(self) -> int:
        """Get the lowest amount of pressure this state could end up releasing."""
        return self.total_released

    @property
    def upper_bound(self) -> int:
        """Get a quick upper bound on the amount of pressure this state could end up releasing."""
        if not self.agents:
            return self.total_released
        turns = max(0, min(i.turns for i in self.agents))
        if turns <= 2:
            return self.total_released
        total = self.total_released
        for valve in self.closed_valves:
            cost = min(agent.valve.distances[valve.name] for agent in self.agents) + 1
            if cost <= turns:
                total += valve.flow_rate * (turns - cost)
        return total

    def __lt__(self, other: State) -> bool:
        """Compare states by their lower bound.

        Gives the inverse answer, for use in a min-heap.
        """
        return self.lower_bound > other.lower_bound

    def continuations(self) -> Iterable[State]:
        """Get possible continuations from this state."""
        for valve in self.closed_valves:
            for idx, agent in enumerate(self.agents):
                turns = agent.turns - agent.valve.distances[valve.name] - 1
                if turns <= 0:
                    continue
                new_agent = Agent(valve, turns)
                # For speed we want to do all of one agent's turns before moving on to the next agent.
                # So, if we are moving on to the next agent, discard the current agent.
                new_agents = [new_agent, *self.agents[idx + 1:]]
                new_closed_valves = [i for i in self.closed_valves if i != valve]
                yield State(
                    self.total_released + turns * valve.flow_rate,
                    new_closed_valves,
                    new_agents,
                )


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

    def max_pressure_release(self, start_state: State) -> int:
        """Get the maximum pressure release from opening the given valves in any order."""
        queue: list[State] = [start_state]
        best = 0
        while queue:
            state = heappop(queue)
            if state.upper_bound < best:
                continue
            best = max(best, state.lower_bound)
            for continuation in state.continuations():
                heappush(queue, continuation)
        return best

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        valves: list[Valve] = []
        for valve, (flow_rate, _) in self.valves.items():
            if flow_rate == 0:
                continue
            valves.append(self.distances(valve))
        start_valve = self.distances("AA")
        me = Agent(start_valve, 30)
        return self.max_pressure_release(State(0, valves, [me]))

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
        return self.max_pressure_release(State(0, valves, [me, elephant]))


if __name__ == "__main__":
    Day.submit()
