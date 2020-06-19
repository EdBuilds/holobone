// PRU program to communicate to the MCPXXXX family of SPI ADC ICs. The program 
// generates the SPI signals that are required to receive samples. To use this 
// program as is, use the following wiring configuration:
//   Chip Select (CS):   P9_27    pr1_pru0_pru_r30_5  r30.t5
//   MOSI            :   P9_29    pr1_pru0_pru_r30_1  r30.t1
//   MISO            :   P9_28    pr1_pru0_pru_r31_3  r31.t3
//   CLK             :   P9_30    pr1_pru0_pru_r30_2  r30.t2
//   Sample Clock    :   P8_46    pr1_pru1_pru_r30_1  -- for testing only
// This program was writen by Derek Molloy to align with the content of the book 
// Exploring BeagleBone. See exploringbeaglebone.com/chapter13/

.setcallreg  r29.w2         // set a non-default CALL/RET register
.origin 0                        // start of program in PRU memory
.entrypoint START                // program entry point (for a debugger)

#define PRU0_R31_VEC_VALID 32    // allows notification of program completion
#define PRU_EVTOUT_0    3        // the event number that is sent back

// Constants from the MCP3004/3008 datasheet 
#define TIME_CLOCK      0x1312D00       // T_hi and t_lo = 125ns = 25 instructions (min)

START:
        // Enable the OCP master port -- allows transfer of data to Linux userspace
//    LBCO    r0, C4, 4, 4     // load SYSCFG reg into r0 (use c4 const addr)
//    CLR     r0, r0, 4        // clear bit 4 (STANDBY_INIT)
//    SBCO    r0, C4, 4, 4     // store the modified r0 back at the load addr

    CLR    r30.t5         // set the clock high
    CLR    r30.t2         // set the clock high
    CLR    r30.t1         // set the clock high
START2:
    MOV    r10, 0xDEADBEEF     // load the base address into r1
    CALL    SPICLK           // repeat call the SPICLK procedure until all 24-bits written/read

    MOV    r0, TIME_CLOCK     // time for clock high
WAIT1:    
    SUB    r0, r0, 1     // decrement the counter by 1 and loop (next line)
    QBNE    WAIT1, r0, 0     // check if the count is still low                 
    CALL    SPICLK           // repeat call the SPICLK procedure until all 24-bits written/read

    MOV    r0, TIME_CLOCK     // time for clock high
WAIT2:    
    SUB    r0, r0, 1     // decrement the counter by 1 and loop (next line)
    QBNE    WAIT2, r0, 0     // check if the count is still low                 
    QBA    START2
END:
    HALT



// R10 = data out
// R11 = bit counter
SPICLK:
    MOV    r11, 0x00000010     // load bit counter
CYCLEST:
    MOV    r0, TIME_CLOCK     // time for clock low -- assuming clock low before cycle
    CLR    r30.t5         // set the clock low
    //set output
    QBBC   CLEAROUT, r10.t31
    SET    r30.t2
    QBA    OUTEND
CLEAROUT:
    CLR    r30.t2
OUTEND:
    LSL    r10, r10, 1   // shift data to left
    SUB    r11, r11, 1     // decrement bit counter
CLKLOW:    
    SUB    r0, r0, 1     // decrement the counter by 1 and loop (next line)
    QBNE    CLKLOW, r0, 0     // check if the count is still low                 

    QBEQ    SPITRDONE, r11, 0  // The write state needs to be set right here -- bit 31 shifted left

    SET    r30.t5         // set the clock high
    MOV    r0, TIME_CLOCK     // time for clock high
CLKHIGH:    
    SUB    r0, r0, 1     // decrement the counter by 1 and loop (next line)
    QBNE    CLKHIGH, r0, 0     // check if the count is still low                 
    QBA    CYCLEST

SPITRDONE:
    RET
