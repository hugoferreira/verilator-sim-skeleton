extern crate verilated;
extern crate verilated_module;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use verilated_module::module;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

#[module(top)]
pub struct Top {
    #[port(clock)]
    pub clk_i: bool,
    #[port(reset)]
    pub rst_i: bool,
    #[port(output)]
    pub count_o: [bool; 8],
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Screen (ESC to Exit)",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut tb = Top::default();
    tb.eval();
    tb.eval();

    tb.open_trace("counter.vcd", 99).unwrap();

    let mut clocks: u64 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = (u32::from(tb.count_o()) << 16) + (u32::from(tb.count_o()) << 8) + u32::from(tb.count_o()); // write something more funny here!
        }

        if clocks == 0 {
            tb.reset_toggle();
        } else if clocks == 2 {
            tb.reset_toggle();
        }

        tb.clock_toggle();
        tb.eval();
        tb.trace_at(Duration::from_nanos(20 * clocks));

        tb.clock_toggle();
        tb.eval();
        tb.trace_at(Duration::from_nanos(20 * clocks + 10));

        println!("{}: count_o = {}", clocks, tb.count_o());

        clocks += 1;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    tb.trace_at(Duration::from_nanos(20 * clocks));
    tb.finish();
}
