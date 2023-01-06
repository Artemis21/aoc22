use rustc_hash::FxHashMap;
use std::collections::{BinaryHeap, VecDeque};

use crate::Day;

#[derive(Clone)]
pub struct Day16 {
    valves: Vec<Valve>,
    starting_valve: ValveIdx,
}

#[derive(Clone, Debug)]
struct Valve {
    flow: usize,
    distances: Vec<usize>,
}

impl Valve {
    #[inline]
    fn distance_to(&self, other: ValveIdx) -> usize {
        self.distances[other.0]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ValveIdx(usize);

/*
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct ValveSet(u32);

impl ValveSet {
    fn new() -> Self {
        Self(0)
    }

    fn contains(&self, valve: ValveIdx) -> bool {
        self.0 & (1 << valve.0) != 0
    }

    fn insert(&mut self, valve: ValveIdx) {
        self.0 |= 1 << valve.0;
    }

    fn remove(&mut self, valve: ValveIdx) {
        self.0 &= !(1 << valve.0);
    }

    fn iter(&self) -> impl Iterator<Item = ValveIdx> + '_ {
        (0..32)
            .map(ValveIdx)
            .filter(move |valve| self.contains(*valve))
    }
}
*/

struct RawSystem {
    valves: FxHashMap<&'static str, RawValve>,
}

impl RawSystem {
    fn parse(input: &'static str) -> Self {
        let valves = input.lines().map(RawValve::parse).collect();
        Self { valves }
    }

    fn distances(&self, valve: &'static str) -> FxHashMap<&'static str, usize> {
        let mut distances = FxHashMap::default();
        let mut queue = VecDeque::new();
        distances.insert(valve, 0);
        queue.push_back((valve, 0));
        while let Some((valve, distance)) = queue.pop_front() {
            for tunnel in &self.valves[valve].tunnels {
                if !distances.contains_key(tunnel) {
                    distances.insert(tunnel, distance + 2);
                    queue.push_back((tunnel, distance + 1));
                }
            }
        }
        distances
    }
}

struct RawValve {
    flow: usize,
    tunnels: Vec<&'static str>,
}

impl RawValve {
    fn parse(line: &'static str) -> (&'static str, Self) {
        // Sample line: "Valve GG has flow rate=0; tunnels lead to valves FF, HH"
        let mut parts = line.split_whitespace();
        // Skip "Valve"
        let name = parts.nth(1).unwrap();
        // Skip "has", "flow"
        let flow = parts
            .nth(2)
            .unwrap()
            .strip_prefix("rate=")
            .unwrap()
            .strip_suffix(';')
            .unwrap()
            .parse()
            .unwrap();
        // Skip "tunnel[s]", "lead", "to", "valve[s]"
        let tunnels = parts.skip(4).map(|s| s.trim_end_matches(',')).collect();
        (name, Self { flow, tunnels })
    }
}

impl TryFrom<RawSystem> for Day16 {
    type Error = ();

    fn try_from(raw: RawSystem) -> Result<Self, Self::Error> {
        let names: Vec<&'static str> = raw
            .valves
            .iter()
            .filter(|(&name, valve)| name == "AA" || valve.flow != 0)
            .map(|(name, _)| *name)
            .collect();
        let mut starting_valve = None;
        let valves = names
            .iter()
            .enumerate()
            .map(|(idx, name)| {
                if name == &"AA" {
                    starting_valve = Some(ValveIdx(idx));
                }
                let raw_distances = raw.distances(name);
                let distances = names
                    .iter()
                    .filter_map(|name| raw_distances.get(name).copied())
                    .collect();
                Valve {
                    flow: raw.valves[name].flow,
                    distances,
                }
            })
            .collect();
        starting_valve
            .map(|starting_valve| Self {
                valves,
                starting_valve,
            })
            .ok_or(())
    }
}

impl Day16 {
    fn valve(&self, idx: ValveIdx) -> &Valve {
        &self.valves[idx.0]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State<const TURNS_PER_AGENT: usize> {
    total_released: usize,
    closed_valves: Vec<ValveIdx>,
    location: ValveIdx,
    turns_remaining: usize,
    agents_remaining: usize,
}

impl<const TURNS_PER_AGENT: usize> PartialOrd for State<TURNS_PER_AGENT> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total_released.partial_cmp(&other.total_released)
    }
}

impl<const TURNS_PER_AGENT: usize> Ord for State<TURNS_PER_AGENT> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_released.cmp(&other.total_released)
    }
}

impl<const TURNS_PER_AGENT: usize> State<TURNS_PER_AGENT> {
    fn new(valves: &Day16, agents: usize) -> Self {
        let mut closed_valves = Vec::new();
        for valve in 0..valves.valves.len() {
            closed_valves.push(ValveIdx(valve));
        }
        Self {
            total_released: 0,
            closed_valves,
            location: valves.starting_valve,
            turns_remaining: TURNS_PER_AGENT,
            agents_remaining: agents - 1,
        }
    }

    fn upper_bound(&self, valves: &Day16) -> usize {
        self.total_released
            + self
                .closed_valves
                .iter()
                .map(|&valve| {
                    valves.valve(valve).flow
                        * self
                            .turns_remaining
                            .saturating_sub(valves.valve(self.location).distance_to(valve))
                            .max(if self.agents_remaining > 0 {
                                TURNS_PER_AGENT.saturating_sub(
                                    valves.valve(valves.starting_valve).distance_to(valve),
                                )
                            } else {
                                0
                            })
                })
                .sum::<usize>()
    }

    fn continuations<'a>(&'a self, valves: &'a Day16) -> impl Iterator<Item = Self> + 'a {
        self.closed_valves
            .iter()
            .enumerate()
            .filter_map(|(idx, &valve)| {
                self.turns_remaining
                    .checked_sub(valves.valve(self.location).distance_to(valve))
                    .map(|turns_remaining| {
                        let mut closed_valves = self.closed_valves.clone();
                        closed_valves.swap_remove(idx);
                        Self {
                            total_released: self.total_released
                                + valves.valve(valve).flow * turns_remaining,
                            closed_valves,
                            location: valve,
                            turns_remaining,
                            ..*self
                        }
                    })
            })
            .chain(if self.agents_remaining > 0 {
                Some(Self {
                    location: valves.starting_valve,
                    turns_remaining: TURNS_PER_AGENT,
                    agents_remaining: self.agents_remaining - 1,
                    ..self.clone()
                })
            } else {
                None
            })
    }

    fn max_release(self, valves: &Day16) -> usize {
        let mut queue = BinaryHeap::new();
        queue.push(self);
        let mut max_release = 0;
        while let Some(state) = queue.pop() {
            max_release = max_release.max(state.total_released);
            for continuation in state.continuations(valves) {
                if continuation.upper_bound(valves) > max_release {
                    queue.push(continuation);
                }
            }
        }
        max_release
    }
}

impl Day for Day16 {
    fn parse(input: &'static str) -> Self {
        RawSystem::parse(input).try_into().unwrap()
    }

    fn part1(&self) -> String {
        State::<30>::new(self, 1).max_release(self).to_string()
    }

    fn part2(&self) -> String {
        State::<26>::new(self, 2).max_release(self).to_string()
    }
}
