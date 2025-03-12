use wgpu_game_engine::run;

fn main() {
    pollster::block_on(run());
}