"""The solution to day 19."""
from __future__ import annotations
from dataclasses import dataclass
import heapq

from .aoc_helper import Solution


@dataclass
class ResourceCount:
    """Counts for each type of resource (used for inventories, recipes, robot counts...)."""

    ore: int = 0
    clay: int = 0
    obsidian: int = 0
    geode: int = 0

    def __add__(self, other: ResourceCount) -> ResourceCount:
        """Add two resource counts together."""
        return ResourceCount(
            ore=self.ore + other.ore,
            clay=self.clay + other.clay,
            obsidian=self.obsidian + other.obsidian,
            geode=self.geode + other.geode,
        )

    def __mul__(self, other: int) -> ResourceCount:
        """Multiply a resource count by a number."""
        return ResourceCount(
            ore=self.ore * other,
            clay=self.clay * other,
            obsidian=self.obsidian * other,
            geode=self.geode * other,
        )

    __rmul__ = __mul__


def parse_recipe(raw: str) -> ResourceCount:
    """Parse a recipe from puzzle format."""
    _, raw_recipe = raw.removeprefix("Each ").removesuffix(".").replace(" robot costs ", ":").split(":")
    cost: dict[str, int] = {}
    for raw_ingredient in raw_recipe.split(" and "):
        amount, ingredient = raw_ingredient.split(" ")
        cost[ingredient] = int(amount)
    return ResourceCount(**cost)


class Blueprint:
    """A blueprint for building each type of robot."""

    def __init__(self, id: int, recipes: list[ResourceCount]):
        """Store the blueprint."""
        self.id = id
        self.recipes = recipes
        self.robot_ore_amount_ore = self.recipes[0].ore
        self.robot_clay_amount_ore = self.recipes[1].ore
        self.robot_obsidian_amount_ore = self.recipes[2].ore
        self.robot_obsidian_amount_clay = self.recipes[2].clay
        self.robot_geode_amount_ore = self.recipes[3].ore
        self.robot_geode_amount_obsidian = self.recipes[3].obsidian
        self.max_ore_cost = max(recipe.ore for recipe in self.recipes)
        self.max_clay_cost = self.robot_obsidian_amount_clay
        self.max_obsidian_cost = self.robot_geode_amount_obsidian

    @classmethod
    def parse(cls, raw: str) -> Blueprint:
        """Parse a blueprint from puzzle format."""
        id, raw_recipes = raw.removeprefix("Blueprint ").split(": ")
        recipes: list[ResourceCount] = []
        recipe_buffer = ""
        for word in raw_recipes.split(" "):
            if word.endswith("."):
                recipe_buffer += word
                recipes.append(parse_recipe(recipe_buffer))
                recipe_buffer = ""
            else:
                recipe_buffer += word + " "
        return cls(int(id), recipes)


@dataclass
class Agent:
    """The state of the agent in a simulation."""

    turns: int
    inventory: ResourceCount
    robots: ResourceCount
    blueprint: Blueprint

    @classmethod
    def initial(cls, blueprint: Blueprint, turns: int) -> Agent:
        """Return the initial state of the simulation."""
        return cls(turns=turns, inventory=ResourceCount(), robots=ResourceCount(ore=1), blueprint=blueprint)

    def upper_bound(self) -> int:
        """Return an upper bound on the number of geodes that can be made."""
        robot_ore_inv_ore = robot_clay_inv_ore = robot_obsidian_inv_ore = robot_geode_inv_ore = self.inventory.ore
        robot_obsidian_inv_clay = self.inventory.clay
        robot_geode_inv_obsidian = self.inventory.obsidian
        ore_robots, clay_robots, obsidian_robots, geode_robots = self.robots.ore, self.robots.clay, self.robots.obsidian, self.robots.geode
        geode = self.inventory.geode
        for _ in range(self.turns):
            new_ore_robots, new_clay_robots, new_obsidian_robots, new_geode_robots = ore_robots, clay_robots, obsidian_robots, geode_robots
            if robot_ore_inv_ore >= self.blueprint.robot_ore_amount_ore:
                robot_ore_inv_ore -= self.blueprint.robot_ore_amount_ore
                new_ore_robots += 1
            if robot_clay_inv_ore >= self.blueprint.robot_clay_amount_ore:
                robot_clay_inv_ore -= self.blueprint.robot_clay_amount_ore
                new_clay_robots += 1
            if robot_obsidian_inv_ore >= self.blueprint.robot_obsidian_amount_ore and robot_obsidian_inv_clay >= self.blueprint.robot_obsidian_amount_clay:
                robot_obsidian_inv_ore -= self.blueprint.robot_obsidian_amount_ore
                robot_obsidian_inv_clay -= self.blueprint.robot_obsidian_amount_clay
                new_obsidian_robots += 1
            if robot_geode_inv_ore >= self.blueprint.robot_geode_amount_ore and robot_geode_inv_obsidian >= self.blueprint.robot_geode_amount_obsidian:
                robot_geode_inv_ore -= self.blueprint.robot_geode_amount_ore
                robot_geode_inv_obsidian -= self.blueprint.robot_geode_amount_obsidian
                new_geode_robots += 1
            robot_ore_inv_ore += ore_robots
            robot_clay_inv_ore += ore_robots
            robot_obsidian_inv_ore += ore_robots
            robot_geode_inv_ore += ore_robots
            robot_obsidian_inv_clay += clay_robots
            robot_geode_inv_obsidian += obsidian_robots
            geode += geode_robots
            ore_robots, clay_robots, obsidian_robots, geode_robots = new_ore_robots, new_clay_robots, new_obsidian_robots, new_geode_robots
        return geode

    def __lt__(self, other: Agent) -> bool:
        """Compare the agents by their lower bounds in reverse, for a minheap."""
        return self.inventory.geode + self.robots.geode * self.turns > other.inventory.geode + other.robots.geode * other.turns

    def ore_continuation(self) -> Agent | None:
        """Get the agent after making an ore robot."""
        if self.robots.ore >= self.blueprint.max_ore_cost:
            return None
        cost = self.inventory.ore - self.blueprint.robot_ore_amount_ore
        cost = cost if cost > 0 else 0
        turns_required = 1 - (cost // -self.robots.ore)
        turns = self.turns - turns_required
        if turns < 1:
            return None
        robots = self.robots
        robots.ore += 1
        inventory = self.inventory + self.robots * turns_required
        inventory.ore -= self.blueprint.robot_ore_amount_ore
        return Agent(turns=turns, inventory=inventory, robots=robots, blueprint=self.blueprint)

    def clay_continuation(self) -> Agent | None:
        """Get the agent after making a clay robot."""
        if self.robots.clay >= self.blueprint.max_clay_cost:
            return None
        cost = self.inventory.ore - self.blueprint.robot_clay_amount_ore
        cost = cost if cost > 0 else 0
        turns_required = 1 - (cost // -self.robots.ore)
        turns = self.turns - turns_required
        if turns < 5:
            return None
        robots = self.robots
        robots.clay += 1
        inventory = self.inventory + self.robots * turns_required
        inventory.ore -= self.blueprint.robot_clay_amount_ore
        return Agent(turns=turns, inventory=inventory, robots=robots, blueprint=self.blueprint)

    def obsidian_continuation(self) -> Agent | None:
        """Get the agent after making an obsidian robot."""
        if not self.robots.clay:
            return None
        if self.robots.obsidian >= self.blueprint.max_obsidian_cost:
            return None
        cost_ore = self.inventory.ore - self.blueprint.robot_obsidian_amount_ore
        cost_ore = cost_ore if cost_ore > 0 else 0
        cost_clay = self.inventory.clay - self.blueprint.robot_obsidian_amount_clay
        cost_clay = cost_clay if cost_clay > 0 else 0
        ore_turns_req = -(cost_ore // -self.robots.ore)
        clay_turns_req = -(cost_clay // -self.robots.clay)
        turns_required = 1 + (ore_turns_req if ore_turns_req > clay_turns_req else clay_turns_req)
        turns = self.turns - turns_required
        if turns < 3:
            return None
        robots = self.robots
        robots.obsidian += 1
        inventory = self.inventory + self.robots * turns_required
        inventory.ore -= self.blueprint.robot_obsidian_amount_ore
        inventory.clay -= self.blueprint.robot_obsidian_amount_clay
        return Agent(turns=turns, inventory=inventory, robots=robots, blueprint=self.blueprint)

    def geode_continuation(self) -> Agent | None:
        """Get the agent after making a geode robot."""
        if not self.robots.obsidian:
            return None
        cost_ore = self.inventory.ore - self.blueprint.robot_geode_amount_ore
        cost_ore = cost_ore if cost_ore > 0 else 0
        cost_obsidian = self.inventory.obsidian - self.blueprint.robot_geode_amount_obsidian
        cost_obsidian = cost_obsidian if cost_obsidian > 0 else 0
        ore_turns_req = -(cost_ore // -self.robots.ore)
        obsidian_turns_req = -(cost_obsidian // -self.robots.obsidian)
        turns_required = 1 + (ore_turns_req if ore_turns_req > obsidian_turns_req else obsidian_turns_req)
        turns = self.turns - turns_required
        if turns < 1:
            return None
        robots = self.robots
        robots.geode += 1
        inventory = self.inventory + self.robots * turns_required
        inventory.ore -= self.blueprint.robot_geode_amount_ore
        inventory.obsidian -= self.blueprint.robot_geode_amount_obsidian
        return Agent(turns=turns, inventory=inventory, robots=robots, blueprint=self.blueprint)

    def geode_continuations(self) -> list[Agent] | int:
        """Get all possible continuations, or the geode count if there are none."""
        continuations: list[Agent] = []
        if c := self.ore_continuation():
            continuations.append(c)
        if c := self.clay_continuation():
            continuations.append(c)
        if c := self.obsidian_continuation():
            continuations.append(c)
        if c := self.geode_continuation():
            continuations.append(c)
        if continuations:
            return continuations
        return self.inventory.geode + self.robots.geode * self.turns

    def max_geodes(self) -> int:
        """Return the maximum number of geodes that can be opened."""
        queue: list[Agent] = [self]
        best = 0
        while queue:
            agent = heapq.heappop(queue)
            if agent.turns < 1:
                continue
            conts = agent.geode_continuations()
            if isinstance(conts, int):
                if conts > best:
                    best = conts
            else:
                for cont in conts:
                    if cont.upper_bound() > best:
                        heapq.heappush(queue, cont)
        return best


class Day(Solution):
    """The solution to day 19."""

    day = 19

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.blueprints: list[Blueprint] = []
        for line in raw.splitlines():
            self.blueprints.append(Blueprint.parse(line))

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        total = 0
        for blueprint in self.blueprints:
            agent = Agent.initial(blueprint, turns=24)
            total += agent.max_geodes() * blueprint.id
        return total

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        total = 1
        for blueprint in self.blueprints[:3]:
            agent = Agent.initial(blueprint, turns=32)
            total *= agent.max_geodes()
        return total


if __name__ == "__main__":
    Day.submit()
