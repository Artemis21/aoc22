"""The solution to day 19."""
from __future__ import annotations
from typing import Iterator
from dataclasses import dataclass
import heapq

from .aoc_helper import Solution


def floor_div_zero(a: int, b: int) -> int:
    """Return a // b, or 0 if a and b are both 0.

    Zero division error is still raised if b is 0 and a is not.
    """
    return a // b if b else 0


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

    def __sub__(self, other: ResourceCount) -> ResourceCount:
        """Subtract on resource count from another."""
        return ResourceCount(
            ore=self.ore - other.ore,
            clay=self.clay - other.clay,
            obsidian=self.obsidian - other.obsidian,
            geode=self.geode - other.geode,
        )

    def __mul__(self, other: int) -> ResourceCount:
        """Multiply a resource count by a number."""
        return ResourceCount(
            ore=self.ore * other,
            clay=self.clay * other,
            obsidian=self.obsidian * other,
            geode=self.geode * other,
        )

    def __floordiv__(self, other: ResourceCount) -> int:
        """Get the minimum number of another resource count needed to make at least this many of each resource."""
        out = max(
            floor_div_zero(self.ore, other.ore),
            floor_div_zero(self.clay, other.clay),
            floor_div_zero(self.obsidian, other.obsidian),
            floor_div_zero(self.geode, other.geode),
        )
        return out

    __rmul__ = __mul__


class Recipe:
    """A recipe to build a certain type of robot."""

    def __init__(self, robot: str, cost: ResourceCount):
        """Store the recipe."""
        self.robot_name = robot
        self.robot = ResourceCount(
            ore=robot == "ore",
            clay=robot == "clay",
            obsidian=robot == "obsidian",
            geode=robot == "geode",
        )
        self.cost = cost

    @classmethod
    def parse(cls, raw: str) -> Recipe:
        """Parse a recipe from puzzle format."""
        robot, raw_recipe = raw.removeprefix("Each ").removesuffix(".").replace(" robot costs ", ":").split(":")
        cost: dict[str, int] = {}
        for raw_ingredient in raw_recipe.split(" and "):
            amount, ingredient = raw_ingredient.split(" ")
            cost[ingredient] = int(amount)
        return cls(robot, ResourceCount(**cost))

    def turns_required(self, inventory: ResourceCount, robots: ResourceCount) -> int | None:
        """Calculate the number of turns required to build this recipe, None if it will never be possible."""
        cost = self.cost - inventory
        if (
            (robots.ore == 0 and cost.ore > 0)
            or (robots.clay == 0 and cost.clay > 0)
            or (robots.obsidian == 0 and cost.obsidian > 0)
        ):
            return None
        return 1 + cost // robots

    def build(self, inventory: ResourceCount) -> ResourceCount:
        """Build the robot and return the new ingredient inventory."""
        return inventory - self.cost


class Blueprint:
    """A blueprint for building each type of robot."""

    def __init__(self, id: int, recipes: dict[str, Recipe]):
        """Store the blueprint."""
        self.id = id
        self.recipes = recipes
        self.max_costs = ResourceCount(
            ore=max(recipe.cost.ore for recipe in recipes.values()),
            clay=max(recipe.cost.clay for recipe in recipes.values()),
            obsidian=max(recipe.cost.obsidian for recipe in recipes.values()),
            geode=max(recipe.cost.geode for recipe in recipes.values()),
        )

    @classmethod
    def parse(cls, raw: str) -> Blueprint:
        """Parse a blueprint from puzzle format."""
        id, raw_recipes = raw.removeprefix("Blueprint ").split(": ")
        recipes: dict[str, Recipe] = {}
        recipe_buffer = ""
        for word in raw_recipes.split(" "):
            if word.endswith("."):
                recipe_buffer += word
                recipe = Recipe.parse(recipe_buffer)
                recipes[recipe.robot_name] = recipe
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
        inventories = {
            "ore": self.inventory,
            "clay": self.inventory,
            "obsidian": self.inventory,
            "geode": self.inventory,
        }
        robots = {
            "ore": self.robots,
            "clay": self.robots,
            "obsidian": self.robots,
            "geode": self.robots,
        }
        for _ in range(self.turns):
            new_robots = robots
            
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

    def geode_continuations(self) -> list[Agent] | int:
        """Get all possible continuations, or the geode count if there are none."""
        continuations: list[Agent] = []
        for robot_name, recipe in self.blueprint.recipes.items():
            if robot_name != "geode" and getattr(self.robots, robot_name) >= getattr(self.blueprint.max_costs, robot_name):
                continue
            turns_required = recipe.turns_required(self.inventory, self.robots)
            if turns_required is None:
                continue
            turns = self.turns - turns_required
            if turns < 1:
                continue
            if turns < 3 and robot_name != "geode":
                continue
            if turns < 5 and robot_name == "clay":
                continue
            robots = self.robots + recipe.robot
            inventory = recipe.build(self.inventory + self.robots * turns_required)
            continuations.append(Agent(
                turns,
                inventory,
                robots,
                self.blueprint,
            ))
        return continuations or self.inventory.geode + self.robots.geode * self.turns

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
            agent = Agent.initial(blueprint)
            total += agent.max_geodes() * blueprint.id
        return total

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""


if __name__ == "__main__":
    Day.submit()
