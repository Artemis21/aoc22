use crate::vec2::Vec2;

use super::{BasicMap, BasicPosition, Cube, Face, Map};

pub struct CubeMap {
    net: BasicMap,
    cube: Cube,
}

#[derive(Copy, Clone, Debug)]
pub struct FacePosition {
    face: Face,
    position: BasicPosition,
}

impl FacePosition {
    fn as_net_position(&self, face_size: isize) -> BasicPosition {
        let pos = self.face.position * face_size + self.position.pos;
        BasicPosition {
            pos,
            ..self.position
        }
    }

    fn from_net_position(position: BasicPosition, cube: &Cube) -> Self {
        let face = *cube.get_face(position.pos / cube.face_size);
        let pos = position.pos % cube.face_size;
        Self {
            face,
            position: BasicPosition { pos, ..position },
        }
    }

    fn face_wrap(&self, cube: &Cube) -> Self {
        let max = (cube.face_size - 1) as isize;
        let Vec2(x, y, _) = self.position.pos;
        let (face_idx, distance) = if x < 0 {
            (self.face.left(), max - y)
        } else if y < 0 {
            (self.face.top(), x)
        } else if x > max {
            (self.face.right(), y)
        } else if y > max {
            (self.face.bottom(), max - x)
        } else {
            return *self;
        };
        let face = *cube.get_face(face_idx);
        let position = if self.face.position == face.left() {
            BasicPosition {
                pos: Vec2::new(0, distance),
                dir: Vec2::new(1, 0),
            }
        } else if self.face.position == face.top() {
            BasicPosition {
                pos: Vec2::new(max - distance, 0),
                dir: Vec2::new(0, 1),
            }
        } else if self.face.position == face.right() {
            BasicPosition {
                pos: Vec2::new(max, max - distance),
                dir: Vec2::new(-1, 0),
            }
        } else if self.face.position == face.bottom() {
            BasicPosition {
                pos: Vec2::new(distance, max),
                dir: Vec2::new(0, -1),
            }
        } else {
            panic!("face adjacency is not mutual");
        };
        Self { face, position }
    }
}

impl From<BasicMap> for CubeMap {
    fn from(net: BasicMap) -> Self {
        let cube = Cube::from_lines(&net.map, net.face_size);
        Self { net, cube }
    }
}

impl Map for CubeMap {
    type Position = FacePosition;

    fn turn_position(mut pos: Self::Position, turn: super::Turn) -> Self::Position {
        pos.position.dir = BasicMap::turn_position(pos.position, turn).dir;
        pos
    }

    fn score_position(&self, pos: Self::Position) -> usize {
        let pos = pos.as_net_position(self.cube.face_size as isize);
        self.net.score_position(pos)
    }

    fn start_point(&self) -> Self::Position {
        let net_position = self.net.start_point();
        FacePosition::from_net_position(net_position, &self.cube)
    }

    fn tile_at(&self, pos: Self::Position) -> super::Tile {
        let pos = pos.as_net_position(self.cube.face_size as isize);
        self.net.tile_at(pos)
    }

    fn next_position(&self, mut pos: Self::Position) -> Self::Position {
        pos.position.pos += pos.position.dir;
        pos.face_wrap(&self.cube)
    }
}
