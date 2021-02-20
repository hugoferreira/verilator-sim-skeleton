`default_nettype none

module top(input i_sw, output o_led);
	assign o_led = ~i_sw;
endmodule
