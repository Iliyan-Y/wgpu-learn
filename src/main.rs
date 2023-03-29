mod pipeline;
mod state;
mod window_runner;
use window_runner::run;

fn main() {
  pollster::block_on(run());
}
