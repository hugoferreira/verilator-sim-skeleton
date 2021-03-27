#include <stdio.h>
#include <stdlib.h>
#include "Vtop.h"
#include "verilated.h"

int main(int argc, char **argv) {
	Verilated::commandArgs(argc, argv);
	Vtop *tb = new Vtop;

	tb->i_sw = 0;

	for(int k=0; k < 20; k++) {
		tb->i_sw = k & 1; // Set the switch input to the LSB of our counter
		tb->eval();

		printf("k = %2d, sw = %d, led = %d\n", k, tb->i_sw, tb->o_led);
	}
}
