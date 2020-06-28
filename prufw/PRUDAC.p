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

#define TIME_CLOCK      0x4C4B40       // T_hi and t_lo = 125ns = 25 instructions (min)
#define BANK_1_START    0x0       // T_hi and t_lo = 125ns = 25 instructions (min)
#define BANK_1_END      0x0014       // T_hi and t_lo = 125ns = 25 instructions (min)
#define BANK_2_START    0x2000       // T_hi and t_lo = 125ns = 25 instructions (min)
#define BANK_2_END      0x2014       // T_hi and t_lo = 125ns = 25 instructions (min)
#define LDAC_DELAY      0x4C4B40       // T_hi and t_lo = 125ns = 25 instructions (min)

#define PRU0_ARM_SYSEVT 19

// R1  = Sync pulse address
// R2  = Sync pulse register
// R3  = LDAC countdonw register
// R4  = PRU to host address
// R5  = PRU to host value
// R7  = BANK1 end
// R8  = BANK2 end
// R9  = double buffer address
// R10 = data out
// R11 = bit counter

START:
        // Enable the OCP master port -- allows transfer of data to Linux userspace
    LBCO    r0, C4, 4, 4     // load SYSCFG reg into r0 (use c4 const addr)
    CLR     r0, r0, 4        // clear bit 4 (STANDBY_INIT)
    SBCO    r0, C4, 4, 4     // store the modified r0 back at the load addr

    SET     r30.t5         // set the clock high
    CLR     r30.t3         // set the clock high
    CLR     r30.t2         // set the clock high
    CLR     r30.t1         // set the clock high
    SET     r30.t0         // set the clock high

    MOV     r7, BANK_1_END   //Store bank 1 end address
    MOV     r8, BANK_2_END   //Store bank 2 end address
    MOV     r9, BANK_1_START //Load bank 1 to dbuf register 
    MOV     r1, 0x10008       // Store sync pulse address
    MOV     r4, 0x1000C      // Store pru to host address
    MOV     r5, 0x0
    SBBO    r5, r4, 0, 4 

WAIT_FOR_SYNC:
    LBBO    r2, r1, 0, 4
    QBBC    WAIT_FOR_SYNC, r2.t0
    MOV     r2, 0
    SBBO    r2, r1, 0, 4 

    MOV     r3, LDAC_DELAY
    CLR     r30.t0         // set LDAC low
WAIT_LDAC_LOW:
    SUB     r3, r3, 1     // decrement the counter by 1 and loop (next line)
    QBNE    WAIT_LDAC_LOW, r3, 0     // check if the count is still low                 
    SET     r30.t0         // set LDAC high

DBUF:
    LBBO    r10, r9, 0, 4     // load current command to output register
    ADD     r9, r9, 4         // Increment dbuf pointer
    QBEQ    BANK1READ, r9, BANK_1_END      // check for cleared buffer 1
    QBEQ    BANK2READ, r9, r8      // check for cleared buffer 2
    QBA     TRANSMIT
BANK1READ:
    MOV     r9, BANK_2_START
    MOV     r5, 0x1
    SBBO    r5, r4, 0, 4 
    MOV     r31.b0, 32 | (PRU0_ARM_SYSEVT - 16) // notify completion to host
    QBA     TRANSMIT
BANK2READ:
    MOV     r9, BANK_1_START
    MOV     r5, 0x2
    SBBO    r5, r4, 0, 4 
    MOV     r31.b0, 32 | (PRU0_ARM_SYSEVT - 16) // notify completion to host

TRANSMIT:
    CALL    SPICLK           // repeat call the SPICLK procedure until all 24-bits written/read

    MOV     r0, TIME_CLOCK     // time for clock high
WAIT1:    
    SUB     r0, r0, 1     // decrement the counter by 1 and loop (next line)
    QBNE    WAIT1, r0, 0     // check if the count is still low                 
    CALL    SPICLK           // repeat call the SPICLK procedure until all 24-bits written/read

    MOV     r0, TIME_CLOCK     // time for clock high
WAIT2:    
    SUB     r0, r0, 1     // decrement the counter by 1 and loop (next line)
    QBNE    WAIT2, r0, 0     // check if the count is still low                 
    QBA     WAIT_FOR_SYNC
END:
    HALT

SPICLK:
    MOV     r11, 0x00000010     // load bit counter
    CLR     r30.t5              // set CS pin low
CYCLEST:
    MOV     r0, TIME_CLOCK      // time for clock low -- assuming clock low before cycle
    CLR     r30.t3              // set the clock low
    //set output
    QBBC    CLEAROUT, r10.t0
    SET     r30.t2
    QBA     OUTEND
CLEAROUT:
    CLR     r30.t2
OUTEND:
    LSR     r10, r10, 1   // shift data to left
    SUB     r11, r11, 1     // decrement bit counter
CLKLOW:    
    SUB     r0, r0, 1     // decrement the counter by 1 and loop (next line)
    QBNE    CLKLOW, r0, 0     // check if the count is still low                 

    QBEQ    SPITRDONE, r11, 0  // The write state needs to be set right here -- bit 31 shifted left

    SET     r30.t3         // set the clock high
    MOV     r0, TIME_CLOCK     // time for clock high
CLKHIGH:    
    SUB     r0, r0, 1     // decrement the counter by 1 and loop (next line)
    QBNE    CLKHIGH, r0, 0     // check if the count is still low                 
    QBA     CYCLEST

SPITRDONE:
    MOV     r0, TIME_CLOCK     // time for clock high
CSLOW:
    SUB     r0, r0, 1     // decrement the counter by 1 and loop (next line)
    QBNE    CSLOW, r0, 0     // check if the count is still low                 
    SET     r30.t5              // set CS pin high
    CLR     r30.t2

    RET
