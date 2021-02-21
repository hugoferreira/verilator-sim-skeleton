module top(input clk_i, input rst_i, output [7:0] count_o);
  counter uut(.clk_i, .rst_i, .count_o);
endmodule
