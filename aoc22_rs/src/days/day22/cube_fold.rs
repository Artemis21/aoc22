use rustc_hash::{FxHashMap, FxHashSet};

use crate::vec2::Vec2;

use super::Tile;

/// We don't use the constant from [`crate::vec2`] because we care about the order.
const DIRECTIONS: [Vec2; 4] = [
    Vec2::new(-1, 0),
    Vec2::new(0, -1),
    Vec2::new(1, 0),
    Vec2::new(0, 1),
];

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub face_size: usize,
    pub faces: [Face; 6],
}

impl Cube {
    pub fn from_lines(net: &[Vec<Tile>], face_size: usize) -> Self {
        let mut partial_face_by_position = find_faces(net, face_size);
        let positions: FxHashSet<_> = partial_face_by_position.keys().copied().collect();
        for (&pos, face) in &mut partial_face_by_position {
            face.adjacent = DIRECTIONS.map(|dir| positions.get(&(pos + dir)).copied());
        }
        let mut partial_cube = PartialCube::default();
        partial_cube.place_face(
            *partial_face_by_position.values().next().unwrap(),
            FaceSlot::Front,
            &partial_face_by_position,
        );
        Self {
            face_size,
            faces: partial_cube.fill_faces(),
        }
    }

    pub fn get_face(&self, position: Vec2) -> &Face {
        self.faces
            .iter()
            .find(|face| face.position == position)
            .unwrap()
    }
}

pub fn face_size(net_width: usize, net_height: usize) -> usize {
    let long = net_width.max(net_height);
    let short = net_width.min(net_height);
    if long > short * 2 {
        // The bounding box of any cube net is either 3 by 4 or 2 by 5.
        // Here, it must be 2 by 5.
        short / 2
    } else {
        // Otherwise it must be 3 by 4.
        long / 4
    }
}

fn find_faces(net: &[Vec<Tile>], face_size: usize) -> FxHashMap<Vec2, PartialFace> {
    (0..net.len())
        .step_by(face_size)
        .flat_map(|y| {
            (0..net[0].len())
                .step_by(face_size)
                .map(move |x| Vec2::new(x as isize, y as isize))
        })
        .filter(|pos| *pos.index(net) != Tile::Void)
        .map(|pos| (pos / face_size, PartialFace::new(pos / face_size)))
        .collect()
}

#[derive(Clone, Copy, Debug)]
pub struct Face {
    /// Position on the net, such that `position * face_size` are the
    /// coordinates of the top-left corner of the face in the net.
    pub position: Vec2,

    /// The faces which are adjacent to this face, in the following order on
    /// the net: left, top, right, bottom.
    pub adjacent: [Vec2; 4],
}

impl Face {
    pub const fn left(&self) -> Vec2 {
        self.adjacent[0]
    }

    pub const fn top(&self) -> Vec2 {
        self.adjacent[1]
    }

    pub const fn right(&self) -> Vec2 {
        self.adjacent[2]
    }

    pub const fn bottom(&self) -> Vec2 {
        self.adjacent[3]
    }
}

#[derive(Default)]
struct PartialCube {
    faces: [Option<PartialFace>; 6],
}

#[derive(Clone, Copy, Debug)]
enum FaceSlot {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

const SLOTS: [FaceSlot; 6] = [
    FaceSlot::Front,
    FaceSlot::Back,
    FaceSlot::Left,
    FaceSlot::Right,
    FaceSlot::Top,
    FaceSlot::Bottom,
];

impl FaceSlot {
    const fn adjacent(self) -> [Self; 4] {
        match self {
            Self::Front => [Self::Left, Self::Top, Self::Right, Self::Bottom],
            Self::Back => [Self::Right, Self::Top, Self::Left, Self::Bottom],
            Self::Left => [Self::Back, Self::Top, Self::Front, Self::Bottom],
            Self::Right => [Self::Front, Self::Top, Self::Back, Self::Bottom],
            Self::Top => [Self::Left, Self::Back, Self::Right, Self::Front],
            Self::Bottom => [Self::Left, Self::Front, Self::Right, Self::Back],
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::Front => 0,
            Self::Back => 1,
            Self::Left => 2,
            Self::Right => 3,
            Self::Top => 4,
            Self::Bottom => 5,
        }
    }
}

impl PartialCube {
    fn place_face(
        &mut self,
        face: PartialFace,
        slot: FaceSlot,
        faces: &FxHashMap<Vec2, PartialFace>,
    ) {
        self.faces[slot.index()] = Some(face);
        let mut rotation = 0;
        for (side_idx, adjacent_slot) in slot.adjacent().iter().enumerate() {
            if let Some(adjacent_face) = &self.faces[adjacent_slot.index()] {
                if let Some(offset) = face
                    .adjacent
                    .iter()
                    .position(|&pos| pos == Some(adjacent_face.position))
                {
                    rotation = (side_idx + 4 - offset) % 4;
                    break;
                }
            }
        }
        for (side_idx, &adjacent_slot) in slot.adjacent().iter().enumerate() {
            if self.faces[adjacent_slot.index()].is_none() {
                if let Some(new_face) = face.adjacent[(side_idx + 4 - rotation) % 4] {
                    self.place_face(faces[&new_face], adjacent_slot, faces);
                }
            }
        }
    }

    fn fill_faces(self) -> [Face; 6] {
        let partial_faces: [PartialFace; 6] = self.faces.map(Option::unwrap);
        SLOTS.map(|slot| {
            let partial_face = partial_faces[slot.index()];
            let mut adjacent_faces = slot
                .adjacent()
                .map(|adj_slot| partial_faces[adj_slot.index()].position);
            let rotation = slot
                .adjacent()
                .iter()
                .enumerate()
                .filter_map(|(side_idx, &adj_slot)| {
                    self.faces[adj_slot.index()].map(|face| (side_idx, face.position))
                })
                .filter_map(|(side_idx, adjacent)| {
                    partial_face
                        .adjacent
                        .iter()
                        .position(|&pos| pos == Some(adjacent))
                        .map(|offset| (side_idx, offset))
                })
                .map(|(side_idx, offset)| (side_idx as isize) - (offset as isize))
                .next()
                .unwrap();
            adjacent_faces.rotate_left(rotation.rem_euclid(4) as usize);
            Face {
                position: partial_face.position,
                adjacent: adjacent_faces,
            }
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct PartialFace {
    /// Same meaning as [`Face::position`].
    position: Vec2,

    /// Same meaning as [`Face::adjacent`], but with `None` for faces which
    /// have not yet been found.
    adjacent: [Option<Vec2>; 4],
}

impl PartialFace {
    const fn new(position: Vec2) -> Self {
        Self {
            position,
            adjacent: [None; 4],
        }
    }
}
