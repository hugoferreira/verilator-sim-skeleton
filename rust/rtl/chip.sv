module chip(input logic clk_1, input logic clk_2, input logic reset, 
            input logic vsync, input logic hsync, input logic [6:0] vpos, input logic  [7:0] hpos, 
            output logic [RGB-1:0] rgb);

  parameter RED = 5, GREEN = 6, BLUE = 5, RGB = RED + GREEN + BLUE, FILE = "palette565.bin";

  // Addressing and Peripherals
  logic [15:0] addr;
  logic  [7:0] cpu_di, cpu_do, tb_do, sp_do, ram_do;
  logic        tb_cs, sp_cs, ram_cs;    
  logic        rw;

  addressdecoder decoder(.addr, .rw, .cpu_di, .tb_do, .sp_do, .ram_do, .tb_cs, .sp_cs, .ram_cs);

  // 8x64kbit Async RAM
  ram_async #(.A(12), .D(8)) ram(.clk(~clk_1), .cs(ram_cs), .rw, .addr(addr[11:0]), .di(cpu_do), .dout(ram_do));

  // Control Unit
  control c0(.clk(clk_1), .reset, .vsync, .addr, .data(cpu_do), .din(cpu_di), .rw);

  // Text Video Buffer  
  logic [3:0] text_color;
  logic [RGB-1:0] trgb; 
  textbuffer tb(.clk(~clk_1), .reset, .addr(addr[9:0]), .cs(tb_cs), .rw, .di(cpu_do), .dout(tb_do), .hpos, .vpos, .vsync, .hsync, .color(text_color));
  palette #(.RED(RED), .GREEN(GREEN), .BLUE(BLUE), .FILE(FILE)) pal_text(.clk(clk_1), .color(text_color), .rgb(trgb));

  // Video Sprites  
  logic pixel;
  logic [RGB-1:0] srgb;
  sprite s0(.clk(~clk_1), .reset, .addr(addr[3:0]), .cs(sp_cs), .rw, .di(cpu_do), .dout(sp_do), .hpos, .vpos, .hsync, .vsync, .pixel);  
  palette #(.RED(RED), .GREEN(GREEN), .BLUE(BLUE), .FILE(FILE)) pal_sprite(.clk(clk_1), .color(pixel ? 4'h9 : 4'h0), .rgb(srgb));

  // Basic Video Signals 
  assign rgb = srgb | trgb; 
endmodule
