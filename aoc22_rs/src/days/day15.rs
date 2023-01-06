use crate::{vec2::Vec2, Day};

const PREFIX: &str = "Sensor at x=";
const COORD_SEP: &str = ", y=";
const INFIX: &str = ": closest beacon is at x=";

#[derive(Clone)]
pub struct Day15(Vec<UnshiftedSensor>);

#[inline]
fn parse_coord(input: &str) -> Vec2<Unshifted> {
    let (raw_x, raw_y) = input.split_once(COORD_SEP).unwrap();
    let x = raw_x.parse().unwrap();
    let y = raw_y.parse().unwrap();
    Vec2::new(x, y)
}

const fn shift_coord(coord: Vec2<Unshifted>) -> Vec2<Shifted> {
    Vec2::new(coord.0 - coord.1, coord.0 + coord.1)
}

const fn unshift_coord(coord: Vec2<Shifted>) -> Vec2<Unshifted> {
    let y = (coord.1 - coord.0) / 2;
    let x = coord.0 + y;
    Vec2::new(x, y)
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
struct Shifted;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
struct Unshifted;

#[derive(Copy, Clone)]
struct UnshiftedSensor {
    pos: Vec2<Unshifted>,
    beacon: Vec2<Unshifted>,
}

impl UnshiftedSensor {
    const fn range_in_row(&self, row: isize) -> Option<std::ops::RangeInclusive<isize>> {
        let max_x_dist = (self.pos.0 - self.beacon.0).abs() + (self.pos.1 - self.beacon.1).abs()
            - (self.pos.1 - row).abs();
        if max_x_dist > 0 {
            Some((self.pos.0 - max_x_dist)..=(self.pos.0 + max_x_dist))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct ShiftedSensor {
    top_left: Vec2<Shifted>,
    bottom_right: Vec2<Shifted>,
}

const AROUND: [Vec2<Shifted>; 4] = [
    Vec2::new(-1, -1),
    Vec2::new(1, -1),
    Vec2::new(1, 1),
    Vec2::new(-1, 1),
];

impl From<UnshiftedSensor> for ShiftedSensor {
    fn from(unshifted: UnshiftedSensor) -> Self {
        let distance = (unshifted.beacon - unshifted.pos).manhattan();
        let left_corner = unshifted.pos - Vec2::new(distance as isize, 0);
        let right_corner = unshifted.pos + Vec2::new(distance as isize, 0);
        Self {
            top_left: shift_coord(left_corner),
            bottom_right: shift_coord(right_corner),
        }
    }
}

impl ShiftedSensor {
    fn intersections(&self, other: Self) -> Vec<Vec2<Shifted>> {
        [
            self.top_left.0,
            self.bottom_right.0,
            other.top_left.0,
            other.bottom_right.0,
        ]
        .into_iter()
        .flat_map(|x| {
            [
                self.top_left.1,
                self.bottom_right.1,
                other.top_left.1,
                other.bottom_right.1,
            ]
            .into_iter()
            .map(move |y| Vec2::new(x, y))
        })
        .collect()
    }

    fn contains(&self, point: Vec2<Shifted>) -> bool {
        (self.top_left.0..=self.bottom_right.0).contains(&point.0)
            && (self.top_left.1..=self.bottom_right.1).contains(&point.1)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum RangeEnd {
    Open(isize),
    Close(isize),
}

impl RangeEnd {
    const fn value(&self) -> isize {
        match self {
            Self::Open(v) | Self::Close(v) => *v,
        }
    }
}

impl PartialOrd for RangeEnd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangeEnd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl Day for Day15 {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let (raw_sensor, raw_beacon) = line[PREFIX.len()..].split_once(INFIX).unwrap();
                    let pos = parse_coord(raw_sensor);
                    let beacon = parse_coord(raw_beacon);
                    UnshiftedSensor { pos, beacon }
                })
                .collect(),
        )
    }

    fn part1(&self) -> String {
        let mut points_of_interest: Vec<_> = self
            .0
            .iter()
            .filter_map(|sensor| sensor.range_in_row(2_000_000))
            .flat_map(|range| {
                [
                    RangeEnd::Open(*range.start()),
                    RangeEnd::Close(*range.end()),
                ]
            })
            .collect();
        points_of_interest.sort();
        let (mut total, mut depth, mut top_open) = (0, 0, 42);
        for point in points_of_interest {
            match point {
                RangeEnd::Open(open) => {
                    if depth == 0 {
                        assert!(top_open == 42);
                        top_open = open;
                    }
                    depth += 1;
                }
                RangeEnd::Close(close) => {
                    depth -= 1;
                    if depth == 0 {
                        assert!(top_open != 42);
                        total += close - top_open;
                        top_open = 42;
                    }
                }
            }
        }
        total.to_string()
    }

    fn part2(&self) -> String {
        const SIZE: isize = 4_000_000;
        let shifted: Vec<_> = self.0.iter().copied().map(ShiftedSensor::from).collect();
        shifted
            .iter()
            .flat_map(|sensor| shifted.iter().map(move |other| (sensor, other)))
            .filter(|(sensor, other)| sensor != other)
            .flat_map(|(sensor, other)| sensor.intersections(*other))
            .flat_map(|point| AROUND.iter().map(move |offset| point + *offset))
            .filter(|point| !shifted.iter().any(|sensor| sensor.contains(*point)))
            .map(unshift_coord)
            .find(|point| (0..=SIZE).contains(&point.0) && (0..=SIZE).contains(&point.1))
            .map(|point| (SIZE * point.0 + point.1).to_string())
            .unwrap()
    }
}
