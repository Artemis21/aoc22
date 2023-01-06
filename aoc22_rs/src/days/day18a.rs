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
        let mut sides = SideSet::new();
        self.0.iter()
            .flat_map(|&pos| Side::all_from(pos))
            .for_each(|side| sides.toggle(&side));
        sides.count()
    }

    fn outer_surface_area(&self) -> usize {
        let mut inner = SideSet::new();
        self.0.iter()
            .flat_map(|&pos| Side::all_from(pos))
            .for_each(|side| inner.toggle(&side));
        let mut outer = SideSet::new();
        self.floodfill().iter()
            .flat_map(|&pos| Side::all_from(pos))
            .for_each(|side| outer.toggle(&side));
        inner.and(&outer);
        inner.count()
    }

    fn floodfill(&self) -> Vec<Vec3> {
        let min = self.0.iter().copied().fold(Vec3::MAX, Vec3::piecewise_min) + Vec3 { x: -1, y: -1, z: -1 };
        let max = self.0.iter().copied().fold(Vec3::MIN, Vec3::piecewise_max) + Vec3 { x: 1, y: 1, z: 1 };
        let mut open = Vec::from(min.product(max));
        let mut visited = PointSet::new();
        let mut edge = Vec::new();
        while let Some(pos) = open.pop() {
            if visited.get(pos) {
                continue;
            }
            visited.set(pos);
            let mut is_edge = false;
            for adj in pos.adjacents() {
                let contains = self.0.contains(&adj);
                if contains && !is_edge {
                    edge.push(pos);
                    is_edge = true;
                }
                println!("{min:?} {max:?} {adj:?}");
                if (min.x..=max.x).contains(&adj.x) && (min.y..=max.y).contains(&adj.y) && (min.z..=max.z).contains(&adj.z) && !contains {
                    open.push(adj);
                }
            }
        }
        edge
    }
}

struct PointSet([u32; 462]);

impl PointSet {
    fn new() -> Self {
        Self([0; 462])
    }

    fn pos_index(pos: Vec3) -> usize {
        ((pos.x + 1) as usize) * 21 + (pos.y + 1) as usize
    }

    fn pos_mask(pos: Vec3) -> u32 {
        1 << ((pos.z + 1) as u32)
    }

    fn set(&mut self, pos: Vec3) {
        let index = Self::pos_index(pos);
        let mask = Self::pos_mask(pos);
        self.0[index] |= mask;
    }

    fn get(&self, pos: Vec3) -> bool {
        let index = Self::pos_index(pos);
        let mask = Self::pos_mask(pos);
        self.0[index] & mask != 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vec3 {
    x: i8,
    y: i8,
    z: i8,
}

impl Vec3 {
    const MIN: Vec3 = Vec3 { x: -128, y: -128, z: -128 };
    const MAX: Vec3 = Vec3 { x: 127, y: 127, z: 127 };

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

    fn product(self, other: Self) -> [Self; 8] {
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
        CARDINALS.map(|&cardinal| self + cardinal)
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

struct SideSet([u64; 462]);

#[derive(Debug)]
struct Side {
    x: i8,
    y: i8,
    z: i8,
    plane: Plane,
}

impl Side {
    fn all_from(pos: Vec3) -> [Self; 6] {
        [
            Self {
                x: pos.x,
                y: pos.y,
                z: pos.z,
                plane: Plane::XY,
            },
            Self {
                x: pos.x,
                y: pos.y,
                z: pos.z,
                plane: Plane::XZ,
            },
            Self {
                x: pos.x,
                y: pos.y,
                z: pos.z,
                plane: Plane::YZ,
            },
            Self {
                x: pos.x + 1,
                y: pos.y,
                z: pos.z,
                plane: Plane::YZ,
            },
            Self {
                x: pos.x,
                y: pos.y + 1,
                z: pos.z,
                plane: Plane::XZ,
            },
            Self {
                x: pos.x,
                y: pos.y,
                z: pos.z + 1,
                plane: Plane::XY,
            },
        ]
    }
}

#[derive(Debug)]
enum Plane {
    XY,
    XZ,
    YZ,
}

impl SideSet {
    fn new() -> Self {
        Self([0; 462])
    }

    fn and(&mut self, other: &SideSet) {
        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a &= *b;
        }
    }

    fn toggle(&mut self, side: &Side) {
        self.0[Self::side_index(side)] ^= Self::side_mask(side);
    }

    fn count(&self) -> usize {
        self.0.iter().map(|&x| x.count_ones() as usize).sum()
    }

    fn side_index(side: &Side) -> usize {
        ((side.x + 1) as usize) * 20 + ((side.y + 1) as usize)
    }

    fn side_mask(side: &Side) -> u64 {
        let plane = match side.plane {
            Plane::XY => 0,
            Plane::XZ => 1,
            Plane::YZ => 2,
        };
        println!("{:?}", side);
        1 << (((side.z + 1) as usize) * 3 + plane)
    }
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
