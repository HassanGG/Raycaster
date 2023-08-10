use crate::{
    game::{GameMap, CELL_WIDTH},
    util::convert_range,
};

#[derive(Debug)]
pub struct Ray {
    pub origin: [f32; 2],
    pub length: f32,
    pub rotation: f32,
}

#[derive(Debug)]
enum RayDirection {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug)]
struct StepLength {
    using_x: f32,
    using_y: f32,
}

#[derive(Debug, Clone, Copy)]
struct StepDirection {
    x: f32,
    y: f32,
}

pub enum Lighting {
    Shaded,
    Lit,
}

type RayLength = StepLength;
type Position = StepDirection;

impl Ray {
    fn get_direction(&self) -> RayDirection {
        match self.rotation % 360.0 {
            r if r > 0.0 && r <= 90.0 => RayDirection::TopLeft,
            r if r > 90.0 && r <= 180.0 => RayDirection::BottomLeft,
            r if r > 180.0 && r <= 270.0 => RayDirection::BottomRight,
            _ => RayDirection::TopRight,
        }
    }

    pub fn collision(&self, map: GameMap) -> (f32, Lighting) {
        let unit = CELL_WIDTH as f32;

        let radians = self.rotation.to_radians();

        let origin = Position {
            x: convert_range(self.origin[0], [-1.0, 1.0], [0.0, map.len() as f32]),
            y: convert_range(self.origin[1], [-1.0, 1.0], [0.0, map.len() as f32]),
        }; // Using map coordinate space as it is easier when using a GameMap

        let ray_direction = self.get_direction();

        let step_direction: StepDirection = match ray_direction {
            RayDirection::TopLeft => StepDirection { x: -1.0, y: 1.0 },
            RayDirection::TopRight => StepDirection { x: 1.0, y: 1.0 },
            RayDirection::BottomLeft => StepDirection { x: -1.0, y: -1.0 },
            RayDirection::BottomRight => StepDirection { x: 1.0, y: -1.0 },
        };

        let step_length = StepLength {
            using_x: (unit.powi(2) + (unit / radians.tan()).powi(2)).sqrt(),
            using_y: (unit.powi(2) + (unit * radians.tan()).powi(2)).sqrt(),
        };

        let mut position = StepDirection {
            y: origin.y.floor(),
            x: origin.x.floor(),
        };

        let mut ray_length = match ray_direction {
            RayDirection::TopLeft => RayLength {
                using_x: (origin.x - position.x) * step_length.using_x,
                using_y: ((position.y + 1.0) - origin.y) * step_length.using_y,
            },
            RayDirection::TopRight => RayLength {
                using_x: ((position.x + 1.0) - origin.x) * step_length.using_x,
                using_y: ((position.y + 1.0) - origin.y) * step_length.using_y,
            },
            RayDirection::BottomLeft => RayLength {
                using_x: (origin.x - position.x) * step_length.using_x,
                using_y: (origin.y - position.y) * step_length.using_y,
            },
            RayDirection::BottomRight => RayLength {
                using_x: ((position.x + 1.0) - origin.x) * step_length.using_x,
                using_y: (origin.y - position.y) * step_length.using_y,
            },
        };

        let mut tile_found = false;
        let max_iter = 50;
        let mut iter = 0;
        let mut final_length = 0.0;
        let mut lighting: Lighting = Lighting::Shaded;
        while !tile_found && iter < max_iter {
            if ray_length.using_x < ray_length.using_y {
                position.x += step_direction.x;
                final_length = ray_length.using_x;
                lighting = Lighting::Shaded;
                ray_length.using_x += step_length.using_x;
            } else {
                position.y += step_direction.y;
                final_length = ray_length.using_y;
                lighting = Lighting::Lit;
                ray_length.using_y += step_length.using_y;
            }

            let (i, j) = (position.x as usize, position.y as usize);

            if map[i][j] == 1 {
                tile_found = true;
            }
            iter += 1;
        }
        (final_length, lighting)
    }
}
