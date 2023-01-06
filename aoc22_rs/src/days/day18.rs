use rustc_hash::FxHashSet;

use crate::Day;

#[derive(Clone)]
pub struct Day18(Scan);

#[derive(Clone)]
struct Scan(Vec<Vec3>);

impl Scan {
    fn parse(raw: &str) -> Self {
        Self(raw.lines().map(Vec3::parse).collect())
    }

    fn surface_area(&self) -> usize {
        let mut sides = FxHashSet::default();
        for pos in &self.0 {
            for side in Side::all_from(*pos) {
                if sides.contains(&side) {
                    sides.remove(&side);
                } else {
                    sides.insert(side);
                }
            }
        }
        sides.len()
    }

    fn outer_surface_area(&self) -> usize {
        let edge: FxHashSet<_> = self
            .floodfill()
            .into_iter()
            .flat_map(Side::all_from)
            .collect();
        let mut sides = FxHashSet::default();
        for pos in &self.0 {
            for side in Side::all_from(*pos) {
                if !edge.contains(&side) {
                    continue;
                }
                if sides.contains(&side) {
                    sides.remove(&side);
                } else {
                    sides.insert(side);
                }
            }
        }
        sides.len()
    }

    fn floodfill(&self) -> Vec<Vec3> {
        let min = self.0.iter().copied().fold(Vec3::MAX, Vec3::piecewise_min)
            + Vec3 {
                x: -1,
                y: -1,
                z: -1,
            };
        let max =
            self.0.iter().copied().fold(Vec3::MIN, Vec3::piecewise_max) + Vec3 { x: 1, y: 1, z: 1 };
        let mut open = Vec::from(min.product(max));
        let mut visited = FxHashSet::default();
        let mut edge = Vec::new();
        while let Some(pos) = open.pop() {
            if !visited.insert(pos) {
                continue;
            }
            let mut is_edge = false;
            for adj in pos.adjacents() {
                let contains = self.0.contains(&adj);
                if contains && !is_edge {
                    edge.push(pos);
                    is_edge = true;
                }
                if (min.x..=max.x).contains(&adj.x)
                    && (min.y..=max.y).contains(&adj.y)
                    && (min.z..=max.z).contains(&adj.z)
                    && !contains
                {
                    open.push(adj);
                }
            }
        }
        edge
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Vec3 {
    x: i8,
    y: i8,
    z: i8,
}

impl Vec3 {
    const MIN: Self = Self {
        x: -128,
        y: -128,
        z: -128,
    };
    const MAX: Self = Self {
        x: 127,
        y: 127,
        z: 127,
    };

    fn parse(raw: &str) -> Self {
        let mut raw_parts = raw.splitn(3, ',');
        Self {
            x: raw_parts.next().unwrap().parse().unwrap(),
            y: raw_parts.next().unwrap().parse().unwrap(),
            z: raw_parts.next().unwrap().parse().unwrap(),
        }
    }

    fn piecewise_min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    fn piecewise_max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    const fn product(self, other: Self) -> [Self; 8] {
        [
            self,
            other,
            Self { x: self.x, ..other },
            Self { y: self.y, ..other },
            Self { z: self.z, ..other },
            Self { x: other.x, ..self },
            Self { y: other.y, ..self },
            Self { z: other.z, ..self },
        ]
    }

    fn adjacents(self) -> [Self; 6] {
        CARDINALS.map(|cardinal| self + cardinal)
    }
}

const CARDINALS: [Vec3; 6] = [
    Vec3 { x: 1, y: 0, z: 0 },
    Vec3 { x: -1, y: 0, z: 0 },
    Vec3 { x: 0, y: 1, z: 0 },
    Vec3 { x: 0, y: -1, z: 0 },
    Vec3 { x: 0, y: 0, z: 1 },
    Vec3 { x: 0, y: 0, z: -1 },
];

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Side {
    pos: Vec3,
    plane: Plane,
}

impl Side {
    const fn all_from(pos: Vec3) -> [Self; 6] {
        [
            Self {
                pos,
                plane: Plane::XY,
            },
            Self {
                pos,
                plane: Plane::XZ,
            },
            Self {
                pos,
                plane: Plane::YZ,
            },
            Self {
                pos: Vec3 {
                    x: pos.x + 1,
                    ..pos
                },
                plane: Plane::YZ,
            },
            Self {
                pos: Vec3 {
                    y: pos.y + 1,
                    ..pos
                },
                plane: Plane::XZ,
            },
            Self {
                pos: Vec3 {
                    z: pos.z + 1,
                    ..pos
                },
                plane: Plane::XY,
            },
        ]
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Plane {
    XY,
    XZ,
    YZ,
}

impl Day for Day18 {
    fn parse(input: &str) -> Self {
        Self(Scan::parse(input))
    }

    fn part1(&self) -> String {
        self.0.surface_area().to_string()
    }

    fn part2(&self) -> String {
        self.0.outer_surface_area().to_string()
    }
}
