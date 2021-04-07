module top(input clk_i, input rst_i, output hsync, output vsync, output [23:0] rgb);
  localparam WIDTH = 320, HEIGHT = 240;

  logic       clk_4;
  logic [7:0] hpos;
  logic [6:0] vpos;
    
  /* verilator lint_off PINMISSING */
  slower_clk clk1(.cin(clk_i), .clk_div4(clk_4), .reset(rst_i));
  hvsync_generator hvsync_gen(.clk(clk_4), .reset(rst_i), .hsync, .vsync, .hpos, .vpos);
  chip #(.RED(8), .GREEN(8), .BLUE(8), .FILE("palette888.bin")) chip(.clk_1(clk_i), .clk_2(clk_4), .reset(rst_i), .vsync, .hsync, .vpos, .hpos, .rgb);
endmodule
