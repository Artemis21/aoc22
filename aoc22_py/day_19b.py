"""The solution to day 19."""
from typing import Iterator
from collections import defaultdict

from .aoc_helper import Solution


class Recipe:
    """A recipe to build a certain type of robot."""

    def __init__(self, robot: str, cost: defaultdict[str, int]):
        """Store the recipe."""
        self.robot = robot
        self.cost = cost

    def __repr__(self) -> str:
        """Display the recipe in debug format."""
        return f"Recipe({self.robot!r}, {self.cost!r})"

    def __str__(self) -> str:
        """Display the recipe in puzzle format."""
        recipe = " and ".join(f"{amount} {ingredient}" for ingredient, amount in self.cost.items() if amount > 0)
        return f"Each {self.robot} robot costs {recipe}."

    @classmethod
    def parse(cls, raw: str) -> "Recipe":
        """Parse a recipe from puzzle format."""
        robot, raw_recipe = raw.removeprefix("Each ").removesuffix(".").replace(" robot costs ", ":").split(":")
        cost: defaultdict[str, int] = defaultdict(int)
        for raw_ingredient in raw_recipe.split(" and "):
            amount, ingredient = raw_ingredient.split(" ")
            cost[ingredient] = int(amount)
        return cls(robot, cost)

    def can_afford(self, inventory: defaultdict[str, int]) -> bool:
        """Return True if the recipe can be afforded with the given ingredient inventory."""
        return all(inventory[ingredient] >= amount for ingredient, amount in self.cost.items())

    def turns_required(self, inventory: defaultdict[str, int], robots: defaultdict[str, int]) -> int | None:
        """Calculate the number of turns required to build this recipe, None if it will never be possible."""
        cost = self.cost.copy()
        for ingredient in cost:
            if cost[ingredient] and not robots[ingredient]:
                return None
        for ingredient, amount in inventory.items():
            cost[ingredient] -= amount
        turns = 1
        while any(amount > 0 for amount in cost.values()):
            turns += 1
            for ingredient, amount in robots.items():
                cost[ingredient] -= amount
        return turns

    def build(self, inventory: defaultdict[str, int]) -> defaultdict[str, int]:
        """Build the robot and return the new ingredient inventory."""
        new_inventory = inventory.copy()
        for ingredient, amount in self.cost.items():
            new_inventory[ingredient] -= amount
        return new_inventory


class Blueprint:
    """A blueprint for building each type of robot."""

    def __init__(self, id: int, recipes: dict[str, Recipe]):
        """Store the blueprint."""
        self.id = id
        self.recipes = recipes

    def __repr__(self) -> str:
        """Display the blueprint in debug format."""
        return f"Blueprint({self.id!r}, {self.recipes!r})"

    def __str__(self) -> str:
        """Display the blueprint in puzzle format."""
        return f"Blueprint {self.id}: " + " ".join(map(str, self.recipes.values()))

    @classmethod
    def parse(cls, raw: str) -> "Blueprint":
        """Parse a blueprint from puzzle format."""
        id, raw_recipes = raw.removeprefix("Blueprint ").split(": ")
        recipes = {}
        recipe_buffer = ""
        for word in raw_recipes.split(" "):
            if word.endswith("."):
                recipe_buffer += word
                recipe = Recipe.parse(recipe_buffer)
                recipes[recipe.robot] = recipe
                recipe_buffer = ""
            else:
                recipe_buffer += word + " "
        return cls(int(id), recipes)

    def affordable_robots(self, inventory: defaultdict[str, int]) -> list[str]:
        """Return a list of robots that can be created with the given ingredient inventory."""
        return [recipe.robot for recipe in self.recipes.values() if recipe.can_afford(inventory)]

    def build(self, robot: str, inventory: defaultdict[str, int]) -> defaultdict[str, int]:
        """Build the given robot and return the new ingredient inventory."""
        return self.recipes[robot].build(inventory)


class Agent:
    """The state of the agent in a simulation."""

    def __init__(self, turns: int, inventory: defaultdict[str, int], robots: defaultdict[str, int], blueprint: Blueprint):
        """Store the state."""
        self.turns = turns
        self.inventory = inventory
        self.robots = robots
        self.blueprint = blueprint

    @classmethod
    def initial(cls, blueprint: Blueprint) -> "Agent":
        """Return the initial state of the simulation."""
        robots: defaultdict[str, int] = defaultdict(int)
        robots["ore"] = 1
        return cls(turns=24, inventory=defaultdict(int), robots=robots, blueprint=blueprint)

    def __repr__(self) -> str:
        """Display the state in debug format."""
        return f"Agent(turns={self.turns}, inventory={self.inventory}, robots={self.robots}, blueprint=<{self.blueprint.id}>)"

    def geode_terminations(self) -> Iterator[int]:
        """Yield all possible geode counts at the end of the simulation."""
        any = False
        for robot, recipe in self.blueprint.recipes.items():
            turns_required = recipe.turns_required(self.inventory, self.robots)
            if turns_required is None:
                continue
            turns = self.turns - turns_required
            if turns < 1:
                continue
            if turns < 3 and robot != "geode":
                continue
            if turns < 5 and robot == "clay":
                continue
            robots = self.robots.copy()
            robots[robot] += 1
            inventory = self.inventory.copy()
            for robot, amount in self.robots.items():
                inventory[robot] += turns_required * amount
            inventory = recipe.build(inventory)
            yield from Agent(
                turns,
                inventory,
                robots,
                self.blueprint,
            ).geode_terminations()
            any = True
        if not any:
            yield self.inventory["geode"] + self.robots["geode"] * self.turns

    def max_geodes(self) -> int:
        """Return the maximum number of geodes that can be opened."""
        return max(self.geode_terminations())


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

# Blueprint 1:
#   Each ore robot costs 4 ore.
#   Each clay robot costs 2 ore.
#   Each obsidian robot costs 3 ore and 14 clay.
#   Each geode robot costs 2 ore and 7 obsidian.

# 1:  [r] 1 ore                       [i] 1 ore
# 2:  [r] 1 ore                       [i] 2 ore
# 3:  [r] 1 ore                       [i] 3 ore
# 4:  [r] 1 ore                       [i] 4 ore
# 5:  [r] 2 ore                       [i] 1 ore
# 6:  [r] 2 ore                       [i] 3 ore
# 7:  [r] 2 ore                       [i] 5 ore
# 8:  [r] 3 ore                       [i] 3 ore
# 9:  [r] 3 ore                       [i] 6 ore
# 11: [r] 4 ore                       [i] 5 ore
# 12: [r] 5 ore                       [i] 5 ore
# 13: [r] 6 ore                       [i] 6 ore
# 14: [r] 7 ore                       [i] 8 ore
# 15: [r] 8 ore                       [i] 11 ore
# 16: [r] 9 ore                       [i] 15 ore
# 17: [r] 10 ore                      [i] 20 ore
# 18: [r] 11 ore                      [i] 26 ore
# 19: [r] 12 ore                      [i] 33 ore
# 20: [r] 13 ore                      [i] 41 ore
# 21: [r] 13 ore                      [i] 54 ore
# 22: [r] 13 ore                      [i] 67 ore
# 23: [r] 13 ore                      [i] 80 ore
# 24: [r] 13 ore                      [i] 93 ore
