module por(input logic clk, input logic reset, output logic user_reset);
  int counter = 'h17D796;
  assign user_reset = 1;

  always_ff @(posedge clk)
  begin
    if (reset) begin 
      counter <= 'h17D796;    // 0.062s @ 25Mhz
      user_reset <= 1;
    end else if (user_reset) begin 
      if (counter == 0) user_reset <= 0;
      counter <= counter - 1;
    end
  end
endmodule