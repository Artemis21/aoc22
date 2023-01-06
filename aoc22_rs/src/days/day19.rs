use std::collections::BinaryHeap;

use crate::Day;

#[derive(Clone)]
pub struct Day19(Vec<Blueprint>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Material {
    fn parse(raw: &str) -> Self {
        match raw {
            "ore" => Self::Ore,
            "clay" => Self::Clay,
            "obsidian" => Self::Obsidian,
            "geode" => Self::Geode,
            _ => panic!("Unknown material: {raw}"),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MaterialCounts {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

const ONE_ORE: MaterialCounts = MaterialCounts {
    ore: 1,
    clay: 0,
    obsidian: 0,
    geode: 0,
};
const ONE_CLAY: MaterialCounts = MaterialCounts {
    ore: 0,
    clay: 1,
    obsidian: 0,
    geode: 0,
};
const ONE_OBSIDIAN: MaterialCounts = MaterialCounts {
    ore: 0,
    clay: 0,
    obsidian: 1,
    geode: 0,
};
const ONE_GEODE: MaterialCounts = MaterialCounts {
    ore: 0,
    clay: 0,
    obsidian: 0,
    geode: 1,
};

impl MaterialCounts {
    fn from_recipe(raw: &'static str) -> Self {
        let mut new = Self::default();
        raw.trim_end_matches('.')
            .split_once(" costs ")
            .unwrap()
            .1
            .split(" and ")
            .map(|raw_ingredient| raw_ingredient.split_once(' ').unwrap())
            .map(|(raw_amount, raw_ingredient)| {
                (Material::parse(raw_ingredient), raw_amount.parse().unwrap())
            })
            .for_each(|(material, amount)| new.increase(material, amount));
        new
    }

    fn increase(&mut self, material: Material, amount: usize) {
        match material {
            Material::Ore => self.ore += amount,
            Material::Clay => self.clay += amount,
            Material::Obsidian => self.obsidian += amount,
            Material::Geode => self.geode += amount,
        }
    }

    fn min_copies_of(&self, other: &Self) -> Option<usize> {
        let copies = [
            (self.ore, other.ore),
            (self.clay, other.clay),
            (self.obsidian, other.obsidian),
            (self.geode, other.geode),
        ]
        .into_iter()
        .filter(|(this, _)| this != &0)
        .map(|(this, that)| (this + that - 1).checked_div(that)) // ceil(this / that)
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .max()
        .unwrap_or_default();
        Some(copies)
    }

    const fn is_superset(&self, other: &Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    const fn saturating_sub(&self, other: &Self) -> Self {
        Self {
            ore: self.ore.saturating_sub(other.ore),
            clay: self.clay.saturating_sub(other.clay),
            obsidian: self.obsidian.saturating_sub(other.obsidian),
            geode: self.geode.saturating_sub(other.geode),
        }
    }
}

impl std::ops::Add for MaterialCounts {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl std::ops::AddAssign for MaterialCounts {
    fn add_assign(&mut self, other: Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }
}

impl std::ops::Mul<usize> for MaterialCounts {
    type Output = Self;

    fn mul(self, other: usize) -> Self {
        Self {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
            geode: self.geode * other,
        }
    }
}

impl std::ops::Sub for MaterialCounts {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl std::ops::SubAssign for MaterialCounts {
    fn sub_assign(&mut self, other: Self) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    id: usize,
    ore: MaterialCounts,
    clay: MaterialCounts,
    obsidian: MaterialCounts,
    geode: MaterialCounts,
}

impl Blueprint {
    fn parse(raw: &'static str) -> Self {
        let (prefix, raw) = raw.split_once(": ").unwrap();
        let mut robots = raw
            .strip_suffix('.')
            .unwrap()
            .splitn(4, ". ")
            .map(MaterialCounts::from_recipe);
        let id = prefix.strip_prefix("Blueprint ").unwrap().parse().unwrap();
        Self {
            id,
            ore: robots.next().unwrap(),
            clay: robots.next().unwrap(),
            obsidian: robots.next().unwrap(),
            geode: robots.next().unwrap(),
        }
    }

    const fn recipes(&self) -> [(MaterialCounts, MaterialCounts); 4] {
        [
            (ONE_ORE, self.ore),
            (ONE_CLAY, self.clay),
            (ONE_OBSIDIAN, self.obsidian),
            (ONE_GEODE, self.geode),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    turns: usize,
    inventory: MaterialCounts,
    robots: MaterialCounts,
    blueprint: Blueprint,
}

impl State {
    fn initial(blueprint: Blueprint, turns: usize) -> Self {
        Self {
            turns,
            inventory: MaterialCounts::default(),
            robots: MaterialCounts {
                ore: 1,
                ..MaterialCounts::default()
            },
            blueprint,
        }
    }

    fn upper_bound(&self) -> usize {
        let mut ore_inv = self.inventory;
        let mut clay_inv = self.inventory;
        let mut obsidian_inv = self.inventory;
        let mut geode_inv = self.inventory;
        let mut robots = self.robots;
        for _ in 0..self.turns {
            for (inv, recipe, robot) in [
                (&mut ore_inv, self.blueprint.ore, ONE_ORE),
                (&mut clay_inv, self.blueprint.clay, ONE_CLAY),
                (&mut obsidian_inv, self.blueprint.obsidian, ONE_OBSIDIAN),
                (&mut geode_inv, self.blueprint.geode, ONE_GEODE),
            ] {
                *inv += robots;
                if inv.is_superset(&recipe) {
                    *inv -= recipe;
                    robots += robot;
                }
            }
        }
        ore_inv.geode
    }

    const fn lower_bound(&self) -> usize {
        self.inventory.geode + self.robots.geode * self.turns
    }

    fn children(&self) -> Vec<Self> {
        self.blueprint
            .recipes()
            .into_iter()
            .filter_map(|(robot, recipe)| {
                Some((
                    robot,
                    recipe,
                    1 + recipe
                        .saturating_sub(&self.inventory)
                        .min_copies_of(&self.robots)?,
                ))
            })
            .filter(|(_, _, turns_needed)| turns_needed < &self.turns)
            .map(|(robot, recipe, turns_needed)| Self {
                turns: self.turns - turns_needed,
                inventory: self.inventory + self.robots * turns_needed - recipe,
                robots: self.robots + robot,
                ..*self
            })
            .collect()
    }

    fn best_geodes(self) -> usize {
        let mut queue = BinaryHeap::new();
        queue.push(self);
        let mut max_geodes = 0;
        while let Some(state) = queue.pop() {
            max_geodes = max_geodes.max(state.lower_bound());
            for child in state.children() {
                if child.upper_bound() > max_geodes {
                    queue.push(child);
                }
            }
        }
        max_geodes
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.lower_bound().cmp(&other.lower_bound())
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Day for Day19 {
    fn parse(input: &'static str) -> Self {
        Self(input.lines().map(Blueprint::parse).collect())
    }

    fn part1(&self) -> String {
        self.0
            .iter()
            .map(|&blueprint| State::initial(blueprint, 24).best_geodes() * blueprint.id)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.0
            .iter()
            .take(3)
            .map(|&blueprint| State::initial(blueprint, 32).best_geodes())
            .product::<usize>()
            .to_string()
    }
}
