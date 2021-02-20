OBJ_DIR = obj_dir
TOP_LEVEL = top
SIM_FILE = simulator.cpp
VERILATOR_PATH = /usr/local/share/verilator

.PHONY: all
all: simulator

${OBJ_DIR}/V${TOP_LEVEL}.cpp: ${TOP_LEVEL}.v
	verilator -Wall -cc ${TOP_LEVEL}.v

${OBJ_DIR}/V${TOP_LEVEL}__ALL.a: ${OBJ_DIR}/V${TOP_LEVEL}.cpp
	make -C ${OBJ_DIR} -f V${TOP_LEVEL}.mk

simulator: ${SIM_FILE} ${OBJ_DIR}/V${TOP_LEVEL}__ALL.a
	g++ -std=c++14 -I${VERILATOR_PATH}/include -I ${OBJ_DIR} \
		${VERILATOR_PATH}/include/verilated.cpp \
		${SIM_FILE} ${OBJ_DIR}/V${TOP_LEVEL}__ALL.a \
		-Os -o simulator
	
.PHONY: clean
clean:
	rm -rf ${OBJ_DIR} simulator
