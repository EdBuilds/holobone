GPIO1 .set 0x4804C000
CLEARDATAOUT_OFFSET .set 0x190
SETDATAOUT_OFFSET .set   0x194
USR0_GPIO1_21  .set   1 << 21
USR1_GPIO1_22  .set   1 << 22
USR2_GPIO1_23  .set   1 << 23
USR3_GPIO1_24  .set   1 << 24

; Constant registers
CPRUCFG .set c4
CPRUDRAM .set c24


    .global _c_int00

_c_int00:
    	lbco &r0, CPRUCFG, 4, 4   ; read SYSCFG
    	clr  r0, r0.t4               ; clear SYSCFG[STANDBY_INIT]
    	sbco &r0, CPRUCFG, 4, 4   ; enable OCP master port

    	clr	r30, r30.t8        ; turn user LED 1 off
	ldi32	r1, 0x00010000	 ; load the base address into r1
	lbbo	&r2, r1, 4, 4	 ; the clock delay is now loaded into r2. 4 bytes.
	ldi32	r4, 0x00010000   ; Going to use PRU shared memory to share the state change
    	ldi32  r6, GPIO1 + SETDATAOUT_OFFSET
    	ldi32  r7, GPIO1 + CLEARDATAOUT_OFFSET
    	ldi32  r8, USR1_GPIO1_22
	qba	ENDOFLOOP	 ; ldi32e to comparison -- avoids duplicating code
 
MAINLOOP:
    	clr	r30, r30.t8        ; turn user LED 1 off
	clr	r5, r5, 0
	sbbo    &r5, r4, 0, 4	 ; store the clock state in PRU shared memory
	and	r0, r2, r2       ; load the delay r2 into temp r0 (50% duty cycle)
	add	r0, r0, 1	 ; balance duty cycle by looping 1 extra time on low
 
DELAYOFF:
	sub	r0, r0, 1	 ; decrement the counter by 1 and loop (next line)
	qbne	DELAYOFF, r0, 0  ; loop until the delay has expired (i.e., equals 0)
				 ; next instruction order used to keep 50% duty cycle
	and	r0, r2, r2       ; re-load the delay r2 into temporary r0
    	set	r30, r30.t8           ; turn user LED 1 on
	sbbo    &r5, r4, 0, 4	 ; store the clock state in PRU shared memory
 
DELAYON:
	sub	r0, r0, 1	 ; decrement the counter by 1 and loop (next line)
	qbne	DELAYON, r0, 0   ; loop until the delay has expired (equals 0)
 
ENDOFLOOP:                       ; is the clock running? 
	lbbo	&r3, r1, 0, 4	 ; loaded the state into r3 -- is running? 4 bytes total
	qbbs	RESETCLK, r3, 1	 ; If r3 bit 1 is high then reload the clock period
	qbbs	MAINLOOP, r3, 0  ; If r3 bit 0 is high then the clock is running
	qba	ENDOFLOOP        ; otherwise loop without toggling the clock -- i.e. clock off
 
RESETCLK:			 ; clear the r3.t1 bit and write back to memory
				 ; indicating that the clock frequency has been updated
	clr	r3, r3.t1	 ; i.e., clear the reload clock flag
	sbbo	&r3, r1, 4, 4    ; write that value back into memory
	qba	_c_int00         ; go back to the start of the program
 
END:				 ; program will not exit due to the QBA on the line above
	halt                     ; halt the pru program -- never reached!
