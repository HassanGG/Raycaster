pub mod gpu;

use ray_caster::run;

fn main() {
    pollster::block_on(run());
}
