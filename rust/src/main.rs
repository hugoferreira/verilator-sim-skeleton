extern crate verilated;
extern crate verilated_module;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use verilated_module::module;

const WIDTH: usize = 320;
const HEIGHT: usize = 241;

#[module(top)]
pub struct Top {
    #[port(clock)]
    pub clk_i: bool,
    #[port(reset)]
    pub rst_i: bool,
    #[port(output)]
    pub hsync: bool,
    #[port(output)]
    pub vsync: bool,
    #[port(output)]
    pub rgb: [bool; 24],
}

fn tickdesign_by(tb: &mut Top, clocks: &mut u64, duration: u64) {
    let target_clock = *clocks + duration;
    while *clocks < target_clock { tickdesign(tb, clocks); }
}

fn tickdesign(tb: &mut Top, clocks: &mut u64) {
    // tb.trace_at(Duration::from_nanos(10 * clocks));
    tb.clock_toggle();
    tb.eval();
    *clocks += 1;
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Screen (ESC to Exit)",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| { panic!("{}", e); });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut tb = Top::default();
    tb.eval();
    tb.eval();

    // tb.open_trace("counter.vcd", 99).unwrap();

    let mut clocks: u64 = 0;

    tb.reset_toggle();
    tickdesign_by(&mut tb, &mut clocks, 10);
    tb.reset_toggle();

    let mut hpos: u32 = 0;
    let mut vpos: u32 = 0;
    let mut frame: u32 = 0;
    let mut vblank = true;
    let mut hblank = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        tickdesign_by(&mut tb, &mut clocks, 2);

        if tb.vsync() != 0 && !vblank {
            vblank = true;
            vpos = 0;
            frame += 1;
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap_or_else(|e| { 
                tb.finish();
                panic!("{}", e); 
            });
            println!("Frame {}", frame);
        }

        if tb.vsync() == 0 && vblank { vblank = false; }

        if !vblank {
            if tb.hsync() != 0 && !hblank { hpos = 0; hblank = true; vpos += 1; } else { hpos += 1; }
            if tb.hsync() == 0 && hblank { hblank = false }
            if !hblank { buffer[(vpos * 320 + hpos) as usize] = u32::from(tb.rgb()); }
        } 
    }

    // tb.trace_at(Duration::from_nanos(20 * clocks));
    tb.finish();
}
