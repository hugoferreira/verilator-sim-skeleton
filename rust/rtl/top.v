module top(clk_i, rst_i, hsync, vsync, rgb);
  input clk_i, rst_i;
  output hsync, vsync;	// H/V sync signals (output)
  output [23:0] rgb;	  // RGB output
  wire display_on;	    // display_on signal
  wire [8:0] hpos;	    // 9-bit horizontal position
  wire [8:0] vpos;	    // 9-bit vertical position

  hvsync_generator hvsync_gen(
    .clk(clk_i),
    .reset(0),
    .hsync(hsync),
    .vsync(vsync),
    .display_on(display_on),
    .hpos(hpos),
    .vpos(vpos)
  );

  // Assign each color bit to individual wires.
  wire r = display_on && (((hpos&7)==0) || ((vpos&7)==0));
  wire g = display_on && vpos[4];
  wire b = display_on && hpos[4];
  
  assign rgb = {r ? 8'hFF : 8'h00 , g ? 8'hFF : 8'h00, b ? 8'hFF : 8'h00 };
endmodule
