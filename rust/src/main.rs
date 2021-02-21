extern crate verilated;
extern crate verilated_module;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use verilated_module::module;

const WIDTH: usize = 320;
const HEIGHT: usize = 320;

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
    while clocks <= 10 { tickdesign(&mut tb, &mut clocks); }
    tb.reset_toggle();

    let mut hpos: u32 = 0;
    let mut vpos: u32 = 0;
    let mut vblank = true;
    let mut hblank = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        tickdesign(&mut tb, &mut clocks);
        tickdesign(&mut tb, &mut clocks);

        if tb.vsync() != 0 && !vblank {
            vblank = true;
            vpos = 0;
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap_or_else(|e| { 
                tb.finish();
                panic!("{}", e); 
            });
        }

        if tb.vsync() == 0 && vblank { vblank = false; }

        if !vblank {
            if tb.hsync() != 0 && !hblank { hpos = 0; hblank = true; vpos += 1; } else { hpos += 1; }
            if tb.hsync() == 0 && hblank { hblank = false }

            if !hblank {
                buffer[(vpos * 320 + hpos) as usize] = u32::from(tb.rgb());
            }
        } 

        println!("Frame {}", clocks);
    }

    // tb.trace_at(Duration::from_nanos(20 * clocks));
    tb.finish();
}
