module counter(input clk_i, input rst_i, output reg [7:0] count_o);
  always @(posedge clk_i) begin
    if (rst_i == 1'b1) begin
      count_o <= 8'b0000;
    end else begin
      count_o <= count_o + 1;
    end
  end   
endmodule
