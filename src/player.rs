use crate::{game::{GameMap, WALL_WIDTH, MOVE_AMOUNT}, graphics::{Line, Ray}};

#[derive(Debug)]
pub struct Player {
    pub pos: [f32; 2],
    pub width: f32,
    pub rotation: f32,
    pub view: Ray,

}

pub const LINE_LENGTH: f32 = 0.05;
impl Player {
    fn validate_move(&mut self, x: f32, y: f32, map: GameMap) -> bool {
        let n = map.len();
        for i in (0..n) {
            for j in (0..n) {
                if map[i][j] == 1 {
                    let (beg_x, end_x) = (
                        i as f32 * WALL_WIDTH - 1.0,
                        (i as f32 + 1.0) * WALL_WIDTH - 1.0,
                    );
                    let (beg_y, end_y) = (
                        j as f32 * WALL_WIDTH - 1.0,
                        (j as f32 + 1.0) * WALL_WIDTH - 1.0,
                    );
                    let h_p_width = (self.width - 0.05) / 2.0;
                    if x + h_p_width < end_x
                        && x + h_p_width > beg_x
                        && y + h_p_width > beg_y
                        && y + h_p_width < end_y
                    {
                        return false;
                    }
                    if x - h_p_width < end_x
                        && x - h_p_width > beg_x
                        && y - h_p_width > beg_y
                        && y - h_p_width < end_y
                    {
                        return false;
                    }
                    if x + h_p_width < end_x
                        && x + h_p_width > beg_x
                        && y - h_p_width > beg_y
                        && y - h_p_width < end_y
                    {
                        return false;
                    }
                    if x - h_p_width < end_x
                        && x - h_p_width > beg_x
                        && y + h_p_width > beg_y
                        && y + h_p_width < end_y
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
    pub fn move_forward(&mut self, map: GameMap) {
        let rad = self.rotation.to_radians();
        let hypot = MOVE_AMOUNT;

        let x = self.pos[0] - hypot * rad.sin();
        let y = self.pos[1] + hypot * rad.cos();

        if self.validate_move(x, y, map) {
            self.pos[0] = x;
            self.pos[1] = y;

            self.view.origin[0] = x;
            self.view.origin[1] = y;

        }
    }

    pub fn move_backward(&mut self, map: GameMap) {
        let rad = self.rotation.to_radians();
        let hypot = MOVE_AMOUNT;

        let x = self.pos[0] + hypot * rad.sin();
        let y = self.pos[1] - hypot * rad.cos();
        if self.validate_move(x, y, map) {
            self.pos[0] = x;
            self.pos[1] = y;

            self.view.origin[0] = x;
            self.view.origin[1] = y;
        }
    }
}
