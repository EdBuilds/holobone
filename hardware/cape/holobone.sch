EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L holobone:ST1S50 U6
U 1 1 5E8DF3DF
P 8400 5000
F 0 "U6" H 8000 5300 50  0000 L CNN
F 1 "ST1S50" H 8500 5300 50  0000 L CNN
F 2 "Package_DFN_QFN:DFN-10-1EP_3x3mm_P0.5mm_EP1.65x2.38mm" H 8550 5550 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/1947270.pdf" H 8550 5550 50  0001 C CNN
F 4 "https://uk.farnell.com/stmicroelectronics/st1s50pur/dc-dc-conv-sync-buck-500khz-vfdfn/dp/2460739?st=ST1S50" H 8400 5000 50  0001 C CNN "Store"
	1    8400 5000
	1    0    0    -1  
$EndComp
$Comp
L power:+15V #PWR037
U 1 1 5E8E3EAF
P 6350 4700
F 0 "#PWR037" H 6350 4550 50  0001 C CNN
F 1 "+15V" H 6365 4873 50  0000 C CNN
F 2 "" H 6350 4700 50  0001 C CNN
F 3 "" H 6350 4700 50  0001 C CNN
	1    6350 4700
	-1   0    0    -1  
$EndComp
$Comp
L power:GNDD #PWR046
U 1 1 5E8E4CF4
P 8450 5650
F 0 "#PWR046" H 8450 5400 50  0001 C CNN
F 1 "GNDD" H 8454 5495 50  0000 C CNN
F 2 "" H 8450 5650 50  0001 C CNN
F 3 "" H 8450 5650 50  0001 C CNN
	1    8450 5650
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR050
U 1 1 5E8E590D
P 10300 4600
F 0 "#PWR050" H 10300 4450 50  0001 C CNN
F 1 "+5V" H 10315 4773 50  0000 C CNN
F 2 "" H 10300 4600 50  0001 C CNN
F 3 "" H 10300 4600 50  0001 C CNN
	1    10300 4600
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR038
U 1 1 5E8E70BC
P 6350 5750
F 0 "#PWR038" H 6350 5500 50  0001 C CNN
F 1 "GND" H 6355 5577 50  0000 C CNN
F 2 "" H 6350 5750 50  0001 C CNN
F 3 "" H 6350 5750 50  0001 C CNN
	1    6350 5750
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R20
U 1 1 5E8E8319
P 9300 4850
F 0 "R20" H 9359 4896 50  0000 L CNN
F 1 "147K" H 9359 4805 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 9230 4850 50  0001 C CNN
F 3 "http://industrial.panasonic.com/cdbs/www-data/pdf/RDO0000/AOA0000C331.pdf" H 9300 4850 50  0001 C CNN
F 4 "https://uk.farnell.com/panasonic/erjp06f1473v/res-147k-1-0-5w-0805-thick-film/dp/2326712" H 9300 4850 50  0001 C CNN "Store"
	1    9300 4850
	1    0    0    -1  
$EndComp
$Comp
L Device:L L1
U 1 1 5E8EB39C
P 9150 4700
F 0 "L1" V 9340 4700 50  0000 C CNN
F 1 "47 uH" V 9249 4700 50  0000 C CNN
F 2 "Inductor_SMD:L_12x12mm_H8mm" H 9150 4700 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2138022.pdf" H 9150 4700 50  0001 C CNN
F 4 "https://uk.farnell.com/epcos/b82477g4473m000/inductor-47uh-2-5a-20-smd/dp/7430400" V 9150 4700 50  0001 C CNN "Store"
	1    9150 4700
	0    -1   -1   0   
$EndComp
Wire Wire Line
	9000 4700 9000 4800
Wire Wire Line
	9000 4800 8900 4800
Wire Wire Line
	8900 5000 9300 5000
Wire Wire Line
	9300 4700 9600 4700
$Comp
L power:GNDD #PWR049
U 1 1 5E904419
P 9600 5400
F 0 "#PWR049" H 9600 5150 50  0001 C CNN
F 1 "GNDD" H 9604 5245 50  0000 C CNN
F 2 "" H 9600 5400 50  0001 C CNN
F 3 "" H 9600 5400 50  0001 C CNN
	1    9600 5400
	1    0    0    -1  
$EndComp
Connection ~ 9600 4700
$Comp
L Device:C_Small C16
U 1 1 5E91983C
P 8600 4500
F 0 "C16" V 8371 4500 50  0000 C CNN
F 1 "0.1uF" V 8462 4500 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 8638 4350 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2802608.pdf" H 8600 4500 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/cga4j2x7r2a104k125aa/cap-0-1-f-100v-10-x7r-0805/dp/2210962" H 8600 4500 50  0001 C CNN "Store"
	1    8600 4500
	0    1    1    0   
$EndComp
Wire Wire Line
	8450 4650 8450 4500
Wire Wire Line
	9000 4500 9000 4700
Connection ~ 9000 4700
Wire Wire Line
	9300 5600 8550 5600
$Comp
L Device:R_Small R14
U 1 1 5E92FB9C
P 8350 4050
F 0 "R14" H 8409 4096 50  0000 L CNN
F 1 "16.9K" H 8409 4005 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 8280 4050 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 8350 4050 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080516k9fkea/res-16k9-1-0-125w-0805-thick-film/dp/1652926" H 8350 4050 50  0001 C CNN "Store"
	1    8350 4050
	1    0    0    -1  
$EndComp
Wire Wire Line
	9300 4750 9300 4700
Connection ~ 9300 4700
Wire Wire Line
	9300 4950 9300 5000
Connection ~ 9300 5000
Wire Wire Line
	9600 4700 9600 4900
$Comp
L Device:C_Small C15
U 1 1 5E953B68
P 7800 5500
F 0 "C15" H 7708 5454 50  0000 R CNN
F 1 "1000 pF" H 7708 5545 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 7838 5350 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2068140.pdf" H 7800 5500 50  0001 C CNN
F 4 "https://uk.farnell.com/avx/08055a102jat2a/cap-1000pf-50v-5-c0g-np0-0805/dp/317457" H 7800 5500 50  0001 C CNN "Store"
	1    7800 5500
	-1   0    0    1   
$EndComp
$Comp
L Device:R_Small R7
U 1 1 5E959E61
P 7800 5300
F 0 "R7" H 7859 5346 50  0000 L CNN
F 1 "10K" H 7859 5255 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 7730 5300 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 7800 5300 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080510k0fkea/res-10k-1-0-125w-0805-thick-film/dp/1469856" H 7800 5300 50  0001 C CNN "Store"
	1    7800 5300
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C14
U 1 1 5E95ED38
P 7150 5300
F 0 "C14" H 7058 5254 50  0000 R CNN
F 1 "22nF" H 7058 5345 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 7188 5150 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2237835.pdf" H 7150 5300 50  0001 C CNN
F 4 "https://uk.farnell.com/avx/08055c223kat2a/cap-0-022-f-50v-10-x7r-0805/dp/578198?st=22nF" H 7150 5300 50  0001 C CNN "Store"
	1    7150 5300
	-1   0    0    1   
$EndComp
$Comp
L Device:C_Small C12
U 1 1 5E95F76E
P 6750 5300
F 0 "C12" H 6658 5254 50  0000 R CNN
F 1 "3.3uF" H 6658 5345 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 6788 5150 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2792994.pdf" H 6750 5300 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/c2012x7r1e335k125ab/cap-3-3-f-25v-10-x7r-0805/dp/2346942" H 6750 5300 50  0001 C CNN "Store"
	1    6750 5300
	-1   0    0    1   
$EndComp
Wire Wire Line
	7900 5100 7800 5100
Wire Wire Line
	7800 5100 7800 5200
Wire Wire Line
	7150 5000 7150 5200
Wire Wire Line
	6750 4900 6750 5200
Wire Wire Line
	7150 5400 7150 5600
Wire Wire Line
	6750 5400 6750 5600
$Comp
L Device:C_Small C10
U 1 1 5E97726E
P 6350 5300
F 0 "C10" H 6258 5254 50  0000 R CNN
F 1 "10uF" H 6258 5345 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 6388 5150 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2792994.pdf" H 6350 5300 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/c2012x5r1e106k125ab/cap-10-f-25v-10-x5r-0805/dp/2346927" H 6350 5300 50  0001 C CNN "Store"
	1    6350 5300
	-1   0    0    1   
$EndComp
Wire Wire Line
	6350 4800 6350 5200
Wire Wire Line
	6350 4700 6350 4800
Connection ~ 6350 4800
$Comp
L Device:C_Small C17
U 1 1 5E925891
P 9600 5000
F 0 "C17" V 9371 5000 50  0000 C CNN
F 1 "2.2uF" V 9462 5000 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 9638 4850 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2792994.pdf" H 9600 5000 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/c2012x5r1h225k125ab/cap-2-2-f-50v-10-x5r-0805/dp/2346931" H 9600 5000 50  0001 C CNN "Store"
	1    9600 5000
	-1   0    0    1   
$EndComp
Wire Wire Line
	9600 5400 9600 5100
$Comp
L Device:R_Small R6
U 1 1 5E941492
P 7500 5300
F 0 "R6" H 7559 5346 50  0000 L CNN
F 1 "4.7M" H 7559 5255 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 7430 5300 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 7500 5300 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw08054m70fkea/res-4m7-1-0-125w-0805-thick-film/dp/2139201" H 7500 5300 50  0001 C CNN "Store"
	1    7500 5300
	1    0    0    -1  
$EndComp
Wire Wire Line
	7500 5200 7500 5000
Wire Wire Line
	7500 5400 7500 5600
Wire Wire Line
	7500 5600 7800 5600
Connection ~ 7800 5600
Wire Wire Line
	7500 5000 7900 5000
Wire Wire Line
	7800 5600 8450 5600
Wire Wire Line
	8350 5350 8350 5500
Wire Wire Line
	6350 5400 6350 5750
Connection ~ 6350 5750
Connection ~ 8450 5600
Wire Wire Line
	8450 5350 8450 5600
Wire Wire Line
	8450 5650 8450 5600
Wire Wire Line
	9300 5450 9300 5600
Wire Wire Line
	6350 4800 7900 4800
Wire Wire Line
	6750 4900 7900 4900
Wire Wire Line
	6750 5600 7150 5600
Connection ~ 7150 5600
Wire Wire Line
	7150 5600 7500 5600
Connection ~ 7500 5600
Wire Wire Line
	6350 5750 8350 5750
Wire Wire Line
	7150 5000 7500 5000
Connection ~ 7500 5000
$Comp
L Device:R_Small R17
U 1 1 5EA0CFFD
P 8850 4500
F 0 "R17" H 8909 4546 50  0000 L CNN
F 1 "10R" H 8909 4455 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 8780 4500 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf4.12" H 8850 4500 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080510r0fkea/res-10r-1-0-125w-0805-thick-film/dp/1469859" H 8850 4500 50  0001 C CNN "Store"
	1    8850 4500
	0    -1   -1   0   
$EndComp
Wire Wire Line
	8500 4500 8450 4500
Wire Wire Line
	8750 4500 8700 4500
Wire Wire Line
	9000 4500 8950 4500
$Comp
L power:+3.3V #PWR045
U 1 1 5E93208D
P 8350 3900
F 0 "#PWR045" H 8350 3750 50  0001 C CNN
F 1 "+3V3" H 8350 4040 50  0000 C CNN
F 2 "" H 8350 3900 60  0000 C CNN
F 3 "" H 8350 3900 60  0000 C CNN
	1    8350 3900
	1    0    0    -1  
$EndComp
Wire Wire Line
	8350 3950 8350 3900
Text GLabel 9300 4200 2    50   Input ~ 0
POWER_GOOD
Wire Wire Line
	8350 4150 8350 4200
Wire Wire Line
	9300 4200 8350 4200
Connection ~ 8350 4200
Wire Wire Line
	8350 4200 8350 4650
$Comp
L power:+3.3V #PWR035
U 1 1 5E9951D8
P 6400 1000
F 0 "#PWR035" H 6400 850 50  0001 C CNN
F 1 "+3V3" H 6400 1140 50  0000 C CNN
F 2 "" H 6400 1000 60  0000 C CNN
F 3 "" H 6400 1000 60  0000 C CNN
	1    6400 1000
	1    0    0    -1  
$EndComp
Wire Wire Line
	6400 1000 6400 1050
$Comp
L power:GNDD #PWR034
U 1 1 5EA305CB
P 5850 1350
F 0 "#PWR034" H 5850 1100 50  0001 C CNN
F 1 "GNDD" H 5854 1195 50  0000 C CNN
F 2 "" H 5850 1350 50  0001 C CNN
F 3 "" H 5850 1350 50  0001 C CNN
	1    5850 1350
	1    0    0    -1  
$EndComp
Wire Wire Line
	5850 1300 5850 1350
$Comp
L Device:C_Small C9
U 1 1 5E9AAB42
P 5850 1200
F 0 "C9" H 5758 1154 50  0000 R CNN
F 1 "0.1uF" H 5758 1245 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 5888 1050 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2802608.pdf" H 5850 1200 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/cga4j2x7r2a104k125aa/cap-0-1-f-100v-10-x7r-0805/dp/2210962" H 5850 1200 50  0001 C CNN "Store"
	1    5850 1200
	-1   0    0    1   
$EndComp
Wire Wire Line
	5850 1100 5850 1050
Wire Wire Line
	5850 1050 6400 1050
$Comp
L Amplifier_Operational:TL084 U3
U 1 1 5EA6DD56
P 8000 2750
F 0 "U3" H 8000 2950 50  0000 C CNN
F 1 "TL084" H 8000 3026 50  0001 C CNN
F 2 "Package_SO:TSSOP-14_4.4x5mm_P0.65mm" H 7950 2850 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/tl081.pdf" H 8050 2950 50  0001 C CNN
F 4 "https://uk.farnell.com/texas-instruments/tl084cpwr/ic-op-amp-3mhz-13v-us-tssop-14/dp/3005142?st=TL084" H 8000 2750 50  0001 C CNN "Store"
	1    8000 2750
	1    0    0    -1  
$EndComp
$Comp
L Amplifier_Operational:TL084 U4
U 2 1 5EA7A004
P 9150 2650
F 0 "U4" H 9150 2850 50  0000 C CNN
F 1 "TL084" H 9150 2926 50  0001 C CNN
F 2 "Package_SO:TSSOP-14_4.4x5mm_P0.65mm" H 9100 2750 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/tl081.pdf" H 9200 2850 50  0001 C CNN
F 4 "https://uk.farnell.com/texas-instruments/tl084cpwr/ic-op-amp-3mhz-13v-us-tssop-14/dp/3005142?st=TL084" H 9150 2650 50  0001 C CNN "Store"
	2    9150 2650
	1    0    0    -1  
$EndComp
$Comp
L Amplifier_Operational:TL084 U4
U 3 1 5EA80D11
P 10150 2550
F 0 "U4" H 10150 2750 50  0000 C CNN
F 1 "TL084" H 10150 2826 50  0001 C CNN
F 2 "Package_SO:TSSOP-14_4.4x5mm_P0.65mm" H 10100 2650 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/tl081.pdf" H 10200 2750 50  0001 C CNN
F 4 "https://uk.farnell.com/texas-instruments/tl084cpwr/ic-op-amp-3mhz-13v-us-tssop-14/dp/3005142?st=TL084" H 10150 2550 50  0001 C CNN "Store"
	3    10150 2550
	1    0    0    -1  
$EndComp
$Comp
L Amplifier_Operational:TL084 U3
U 2 1 5EBB7BC2
P 9150 1450
F 0 "U3" H 9150 1650 50  0000 C CNN
F 1 "TL084" H 9150 1726 50  0001 C CNN
F 2 "Package_SO:TSSOP-14_4.4x5mm_P0.65mm" H 9100 1550 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/tl081.pdf" H 9200 1650 50  0001 C CNN
F 4 "https://uk.farnell.com/texas-instruments/tl084cpwr/ic-op-amp-3mhz-13v-us-tssop-14/dp/3005142?st=TL084" H 9150 1450 50  0001 C CNN "Store"
	2    9150 1450
	1    0    0    -1  
$EndComp
$Comp
L Amplifier_Operational:TL084 U3
U 3 1 5EBB934E
P 10150 1350
F 0 "U3" H 10150 1550 50  0000 C CNN
F 1 "TL084" H 10150 1626 50  0001 C CNN
F 2 "Package_SO:TSSOP-14_4.4x5mm_P0.65mm" H 10100 1450 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/tl081.pdf" H 10200 1550 50  0001 C CNN
F 4 "https://uk.farnell.com/texas-instruments/tl084cpwr/ic-op-amp-3mhz-13v-us-tssop-14/dp/3005142?st=TL084" H 10150 1350 50  0001 C CNN "Store"
	3    10150 1350
	1    0    0    -1  
$EndComp
Wire Wire Line
	7700 2850 7700 3000
Wire Wire Line
	7700 3000 8300 3000
Wire Wire Line
	8300 3000 8300 2750
Wire Wire Line
	8750 2750 8850 2750
Wire Wire Line
	8850 2750 8850 2850
Connection ~ 8850 2750
Wire Wire Line
	8850 3150 8850 3050
Wire Wire Line
	8850 2550 8750 2550
Wire Wire Line
	9050 2200 8850 2200
Wire Wire Line
	8850 2200 8850 2550
Connection ~ 8850 2550
Wire Wire Line
	9250 2200 9450 2200
Wire Wire Line
	9450 2200 9450 2650
Wire Wire Line
	10000 2900 9850 2900
Wire Wire Line
	9850 2900 9850 2650
Wire Wire Line
	9850 2650 9750 2650
Connection ~ 9850 2650
Wire Wire Line
	9550 2650 9450 2650
Connection ~ 9450 2650
Wire Wire Line
	10450 2550 10450 2900
Wire Wire Line
	10450 2900 10200 2900
$Comp
L power:GND #PWR048
U 1 1 5EAEF2B5
P 9700 2300
F 0 "#PWR048" H 9700 2050 50  0001 C CNN
F 1 "GND" H 9705 2127 50  0000 C CNN
F 2 "" H 9700 2300 50  0001 C CNN
F 3 "" H 9700 2300 50  0001 C CNN
	1    9700 2300
	1    0    0    -1  
$EndComp
Wire Wire Line
	9700 2300 9700 2250
Wire Wire Line
	9700 2250 9850 2250
Wire Wire Line
	9850 2250 9850 2450
Wire Wire Line
	8300 2750 8550 2750
Connection ~ 8300 2750
Wire Wire Line
	8550 1550 8300 1550
Wire Wire Line
	9850 1450 9750 1450
Wire Wire Line
	9550 1450 9450 1450
Wire Wire Line
	10000 1700 9850 1700
Wire Wire Line
	9850 1700 9850 1450
Connection ~ 9850 1450
Wire Wire Line
	10200 1700 10450 1700
Wire Wire Line
	10450 1700 10450 1350
Wire Wire Line
	9450 1450 9450 950 
Wire Wire Line
	9450 950  9250 950 
Connection ~ 9450 1450
Wire Wire Line
	9050 950  8850 950 
Wire Wire Line
	8850 950  8850 1350
Wire Wire Line
	8750 1350 8850 1350
Connection ~ 8850 1350
Wire Wire Line
	8750 1550 8850 1550
$Comp
L power:GND #PWR043
U 1 1 5EA8C1EE
P 8850 1900
F 0 "#PWR043" H 8850 1650 50  0001 C CNN
F 1 "GND" H 8855 1727 50  0000 C CNN
F 2 "" H 8850 1900 50  0001 C CNN
F 3 "" H 8850 1900 50  0001 C CNN
	1    8850 1900
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR047
U 1 1 5EA8C890
P 9700 1100
F 0 "#PWR047" H 9700 850 50  0001 C CNN
F 1 "GND" H 9705 927 50  0000 C CNN
F 2 "" H 9700 1100 50  0001 C CNN
F 3 "" H 9700 1100 50  0001 C CNN
	1    9700 1100
	1    0    0    -1  
$EndComp
Wire Wire Line
	9700 1100 9700 1050
Wire Wire Line
	9700 1050 9850 1050
Wire Wire Line
	9850 1050 9850 1250
Connection ~ 8850 1550
Wire Wire Line
	8850 1850 8850 1900
Wire Wire Line
	8850 1550 8850 1650
Wire Wire Line
	8300 1550 8300 2750
Wire Wire Line
	8200 1350 8550 1350
$Comp
L Device:R_Small R12
U 1 1 5EC2BA48
P 8850 1750
F 0 "R12" H 8909 1796 50  0000 L CNN
F 1 "11.3K" H 8909 1705 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 8780 1750 50  0001 C CNN
F 3 "" H 8850 1750 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080511k3fkea/res-11k3-1-0-125w-0805-thick-film/dp/1652911?st=smd%20resisitor" H 8850 1750 50  0001 C CNN "Store"
	1    8850 1750
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R9
U 1 1 5EC2C37F
P 8650 1550
F 0 "R9" V 8709 1596 50  0000 L CNN
F 1 "16.2K" V 8709 1505 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 8580 1550 50  0001 C CNN
F 3 "" H 8650 1550 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080516k2fkea/res-16k2-1-0-125w-0805-thick-film/dp/1469878?st=smd%20resisitor" H 8650 1550 50  0001 C CNN "Store"
	1    8650 1550
	0    -1   -1   0   
$EndComp
$Comp
L Device:R_Small R15
U 1 1 5EC38E46
P 9150 950
F 0 "R15" V 9209 996 50  0000 L CNN
F 1 "18.7K" V 9209 905 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 9080 950 50  0001 C CNN
F 3 "" H 9150 950 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080518k7fkea/res-18k7-1-0-125w-0805-thick-film/dp/1652934?st=smd%20resisitor" H 9150 950 50  0001 C CNN "Store"
	1    9150 950 
	0    -1   -1   0   
$EndComp
$Comp
L Device:R_Small R8
U 1 1 5EC393E2
P 8650 1350
F 0 "R8" V 8709 1396 50  0000 L CNN
F 1 "3.9K" V 8709 1305 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 8580 1350 50  0001 C CNN
F 3 "" H 8650 1350 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw08053k90fkea/res-3k9-1-0-125w-0805-thick-film/dp/1652998?st=smd%20resisitor" H 8650 1350 50  0001 C CNN "Store"
	1    8650 1350
	0    -1   -1   0   
$EndComp
Text GLabel 5800 1950 0    50   Input ~ 0
CS_DAC
Text GLabel 5800 1750 0    50   Input ~ 0
MOSI
Text GLabel 5800 1850 0    50   Input ~ 0
SCLK
Text GLabel 5800 2050 0    50   Input ~ 0
LDAC
Wire Wire Line
	10700 2550 10450 2550
Connection ~ 10450 2550
Wire Wire Line
	10700 2750 10700 3000
Wire Wire Line
	10700 3000 9450 3000
Wire Wire Line
	9450 3000 9450 2650
$Comp
L power:GND #PWR044
U 1 1 5EA3260B
P 8850 3150
F 0 "#PWR044" H 8850 2900 50  0001 C CNN
F 1 "GND" H 8855 2977 50  0000 C CNN
F 2 "" H 8850 3150 50  0001 C CNN
F 3 "" H 8850 3150 50  0001 C CNN
	1    8850 3150
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR052
U 1 1 5EE8FBF7
P 10600 2750
F 0 "#PWR052" H 10600 2500 50  0001 C CNN
F 1 "GND" H 10605 2577 50  0000 C CNN
F 2 "" H 10600 2750 50  0001 C CNN
F 3 "" H 10600 2750 50  0001 C CNN
	1    10600 2750
	1    0    0    -1  
$EndComp
Wire Wire Line
	10700 2650 10600 2650
Wire Wire Line
	10600 2650 10600 2750
$Comp
L Connector_Generic:Conn_01x03 J3
U 1 1 5EEA1179
P 10900 1450
F 0 "J3" H 10850 1650 50  0000 L CNN
F 1 "Conn_01x03" H 10980 1401 50  0001 L CNN
F 2 "Connector_JST:JST_XH_S3B-XH-A-1_1x03_P2.50mm_Horizontal" H 10900 1450 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2057209.pdf" H 10900 1450 50  0001 C CNN
F 4 "https://uk.farnell.com/jst-japan-solderless-terminals/s3b-xh-a-lf-sn/header-right-angle-3way/dp/1516290?MER=sy-me-pd-mi-acce&st=JST%20XH%20S3B" H 10900 1450 50  0001 C CNN "Store"
	1    10900 1450
	1    0    0    -1  
$EndComp
Wire Wire Line
	10700 1350 10450 1350
Wire Wire Line
	10700 1550 10700 1800
Wire Wire Line
	10700 1800 9450 1800
$Comp
L power:GND #PWR051
U 1 1 5EEA1182
P 10600 1550
F 0 "#PWR051" H 10600 1300 50  0001 C CNN
F 1 "GND" H 10605 1377 50  0000 C CNN
F 2 "" H 10600 1550 50  0001 C CNN
F 3 "" H 10600 1550 50  0001 C CNN
	1    10600 1550
	1    0    0    -1  
$EndComp
Wire Wire Line
	10700 1450 10600 1450
Wire Wire Line
	10600 1450 10600 1550
Wire Wire Line
	9450 1800 9450 1450
$Comp
L Connector_Generic:Conn_01x03 J1
U 1 1 5EEE19F9
P 800 3300
F 0 "J1" H 800 3100 50  0000 C CNN
F 1 "Conn_01x03" H 718 3066 50  0001 C CNN
F 2 "Connector_JST:JST_XH_S3B-XH-A-1_1x03_P2.50mm_Horizontal" H 800 3300 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2057209.pdf" H 800 3300 50  0001 C CNN
F 4 "https://uk.farnell.com/jst-japan-solderless-terminals/s3b-xh-a-lf-sn/header-right-angle-3way/dp/1516290?MER=sy-me-pd-mi-acce&st=JST%20XH%20S3B" H 800 3300 50  0001 C CNN "Store"
	1    800  3300
	-1   0    0    1   
$EndComp
$Comp
L power:+15V #PWR03
U 1 1 5EEEF784
P 1100 3100
F 0 "#PWR03" H 1100 2950 50  0001 C CNN
F 1 "+15V" H 1100 3250 50  0000 C CNN
F 2 "" H 1100 3100 50  0001 C CNN
F 3 "" H 1100 3100 50  0001 C CNN
	1    1100 3100
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR06
U 1 1 5EEF36F5
P 1500 3500
F 0 "#PWR06" H 1500 3250 50  0001 C CNN
F 1 "GND" H 1500 3350 50  0000 C CNN
F 2 "" H 1500 3500 50  0001 C CNN
F 3 "" H 1500 3500 50  0001 C CNN
	1    1500 3500
	1    0    0    -1  
$EndComp
Connection ~ 800  1900
Wire Wire Line
	800  1800 800  1900
$Comp
L power:GND #PWR08
U 1 1 5E6B3E51
P 800 2150
F 0 "#PWR08" H 800 1900 50  0001 C CNN
F 1 "GND" H 805 1977 50  0000 C CNN
F 2 "" H 800 2150 50  0001 C CNN
F 3 "" H 800 2150 50  0001 C CNN
	1    800  2150
	1    0    0    -1  
$EndComp
Wire Wire Line
	1050 1900 800  1900
Wire Wire Line
	1050 1800 800  1800
$Comp
L Device:C C3
U 1 1 5E688508
P 2900 1150
F 0 "C3" H 3015 1196 50  0000 L CNN
F 1 "0.1uF" H 3015 1105 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 2938 1000 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2802608.pdf" H 2900 1150 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/cga4j2x7r2a104k125aa/cap-0-1-f-100v-10-x7r-0805/dp/2210962" H 2900 1150 50  0001 C CNN "Store"
	1    2900 1150
	1    0    0    -1  
$EndComp
$Comp
L power:+3.3V #PWR021
U 1 1 5E6A36A5
P 2550 950
F 0 "#PWR021" H 2550 800 50  0001 C CNN
F 1 "+3V3" H 2550 1090 50  0000 C CNN
F 2 "" H 2550 950 60  0000 C CNN
F 3 "" H 2550 950 60  0000 C CNN
	1    2550 950 
	1    0    0    -1  
$EndComp
$Comp
L Switch:SW_DIP_x02 SW1
U 1 1 5E6AFD31
P 1350 1900
F 0 "SW1" H 1350 2150 50  0000 C CNN
F 1 "SW_DIP_x02" H 1350 2176 50  0001 C CNN
F 2 "Package_DIP:DIP-4_W7.62mm_SMDSocket_SmallPads" H 1350 1900 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2918533.pdf" H 1350 1900 50  0001 C CNN
F 4 "https://uk.farnell.com/multicomp/mcdm-r-02-t/switch-2-way-spst-flush/dp/9472029?st=smd%20dip%202" H 1350 1900 50  0001 C CNN "Store"
	1    1350 1900
	1    0    0    -1  
$EndComp
Wire Wire Line
	2150 2000 2100 2000
Wire Wire Line
	1800 2000 1800 2050
Wire Wire Line
	1800 2050 800  2050
Wire Wire Line
	2150 1900 1900 1900
Wire Wire Line
	2150 1800 1700 1800
Wire Wire Line
	1700 1450 1700 1800
Connection ~ 1700 1800
Wire Wire Line
	1700 1800 1650 1800
Wire Wire Line
	1900 1450 1900 1900
Connection ~ 1900 1900
Wire Wire Line
	1900 1900 1650 1900
Wire Wire Line
	2100 1450 2100 2000
Connection ~ 2100 2000
Wire Wire Line
	2100 2000 1800 2000
$Comp
L power:+3.3V #PWR016
U 1 1 5E6E3759
P 1900 950
F 0 "#PWR016" H 1900 800 50  0001 C CNN
F 1 "+3V3" H 1900 1090 50  0000 C CNN
F 2 "" H 1900 950 60  0000 C CNN
F 3 "" H 1900 950 60  0000 C CNN
	1    1900 950 
	1    0    0    -1  
$EndComp
Wire Wire Line
	1900 950  1900 1050
Wire Wire Line
	2100 1150 2100 1050
Wire Wire Line
	2100 1050 1900 1050
Connection ~ 1900 1050
Wire Wire Line
	1900 1050 1900 1150
Wire Wire Line
	1700 1150 1700 1050
Wire Wire Line
	1700 1050 1900 1050
$Comp
L Device:R R4
U 1 1 5E70A53E
P 3350 1350
F 0 "R4" V 3420 1396 50  0000 C CNN
F 1 "5.6K" V 3420 1305 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 3280 1350 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 3350 1350 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw08055k60fkea/res-5k6-1-0-125w-0805-thick-film/dp/1469941" H 3350 1350 50  0001 C CNN "Store"
	1    3350 1350
	1    0    0    -1  
$EndComp
$Comp
L Device:R R5
U 1 1 5E70A544
P 3550 1350
F 0 "R5" V 3620 1396 50  0000 C CNN
F 1 "5.6K" V 3620 1305 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 3480 1350 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 3550 1350 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw08055k60fkea/res-5k6-1-0-125w-0805-thick-film/dp/1469941" H 3550 1350 50  0001 C CNN "Store"
	1    3550 1350
	1    0    0    -1  
$EndComp
Wire Wire Line
	3350 1500 3350 1800
Wire Wire Line
	3550 1500 3550 1900
$Comp
L power:+3.3V #PWR027
U 1 1 5E70A54C
P 3550 950
F 0 "#PWR027" H 3550 800 50  0001 C CNN
F 1 "+3V3" H 3550 1090 50  0000 C CNN
F 2 "" H 3550 950 60  0000 C CNN
F 3 "" H 3550 950 60  0000 C CNN
	1    3550 950 
	1    0    0    -1  
$EndComp
Connection ~ 3550 1100
Wire Wire Line
	3550 1100 3550 1200
Wire Wire Line
	3350 1200 3350 1100
Wire Wire Line
	3350 1100 3550 1100
Wire Wire Line
	2950 1800 3350 1800
Wire Wire Line
	2950 1900 3550 1900
$Comp
L Device:R R1
U 1 1 5E6BC6B8
P 1700 1300
F 0 "R1" V 1770 1346 50  0000 C CNN
F 1 "4.75K" V 1770 1255 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 1630 1300 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 1700 1300 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw08054k75fkea/res-4k75-1-0-125w-0805-thick-film/dp/1469924" H 1700 1300 50  0001 C CNN "Store"
	1    1700 1300
	1    0    0    -1  
$EndComp
$Comp
L Device:R R2
U 1 1 5E6BE187
P 1900 1300
F 0 "R2" V 1970 1346 50  0000 C CNN
F 1 "4.75K" V 1970 1255 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 1830 1300 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 1900 1300 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw08054k75fkea/res-4k75-1-0-125w-0805-thick-film/dp/1469924" H 1900 1300 50  0001 C CNN "Store"
	1    1900 1300
	1    0    0    -1  
$EndComp
$Comp
L Device:R R3
U 1 1 5E6BF0C0
P 2100 1300
F 0 "R3" V 2170 1346 50  0000 C CNN
F 1 "4.75K" V 2170 1255 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 2030 1300 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 2100 1300 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw08054k75fkea/res-4k75-1-0-125w-0805-thick-film/dp/1469924" H 2100 1300 50  0001 C CNN "Store"
	1    2100 1300
	1    0    0    -1  
$EndComp
Text GLabel 3700 1800 2    50   Input ~ 0
SDA
Text GLabel 3700 1900 2    50   Input ~ 0
SCL
Wire Wire Line
	3350 1800 3700 1800
Connection ~ 3350 1800
Wire Wire Line
	3550 1900 3700 1900
Connection ~ 3550 1900
$Comp
L power:GNDD #PWR022
U 1 1 5EA002D6
P 2550 2200
F 0 "#PWR022" H 2550 1950 50  0001 C CNN
F 1 "GNDD" H 2554 2045 50  0000 C CNN
F 2 "" H 2550 2200 50  0001 C CNN
F 3 "" H 2550 2200 50  0001 C CNN
	1    2550 2200
	1    0    0    -1  
$EndComp
$Comp
L power:GNDD #PWR023
U 1 1 5EA015CF
P 2900 1350
F 0 "#PWR023" H 2900 1100 50  0001 C CNN
F 1 "GNDD" H 2904 1195 50  0000 C CNN
F 2 "" H 2900 1350 50  0001 C CNN
F 3 "" H 2900 1350 50  0001 C CNN
	1    2900 1350
	1    0    0    -1  
$EndComp
Wire Wire Line
	800  1900 800  2050
Connection ~ 800  2050
Wire Wire Line
	800  2050 800  2150
Connection ~ 10450 1350
$Comp
L Connector_Generic:Conn_01x03 J4
U 1 1 5EE3DDD1
P 10900 2650
F 0 "J4" H 10850 2850 50  0000 L CNN
F 1 "Conn_01x03" H 10980 2601 50  0001 L CNN
F 2 "Connector_JST:JST_XH_S3B-XH-A-1_1x03_P2.50mm_Horizontal" H 10900 2650 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2057209.pdf" H 10900 2650 50  0001 C CNN
F 4 "https://uk.farnell.com/jst-japan-solderless-terminals/s3b-xh-a-lf-sn/header-right-angle-3way/dp/1516290?MER=sy-me-pd-mi-acce&st=JST%20XH%20S3B" H 10900 2650 50  0001 C CNN "Store"
	1    10900 2650
	1    0    0    -1  
$EndComp
Wire Wire Line
	2400 7400 2000 7400
Wire Wire Line
	2400 7200 2400 7400
Connection ~ 2400 7200
Connection ~ 2000 7400
Wire Wire Line
	4550 7200 4650 7200
Wire Wire Line
	4050 7200 3950 7200
Text GLabel 4650 7200 2    50   Input ~ 0
AUX1
Text GLabel 3950 7200 0    50   Input ~ 0
AUX0
Wire Wire Line
	1750 6400 1650 6400
Wire Wire Line
	1650 6500 1750 6500
Text GLabel 1650 6500 0    50   Input ~ 0
LDAC
Text GLabel 1650 6400 0    50   Input ~ 0
POWER_GOOD
Wire Wire Line
	2350 5900 2250 5900
Wire Wire Line
	1650 5900 1750 5900
Text GLabel 1650 5900 0    50   Input ~ 0
SCL
Text GLabel 2350 5900 2    50   Input ~ 0
SDA
Wire Wire Line
	2350 6000 2250 6000
Text GLabel 2350 6000 2    50   Input ~ 0
SCLK
Wire Wire Line
	1650 5800 1750 5800
Text GLabel 1650 5800 0    50   Input ~ 0
CS_DAC
Wire Wire Line
	2350 6400 2250 6400
Text GLabel 2350 6400 2    50   Input ~ 0
MOSI
Wire Wire Line
	2000 7400 1600 7400
Wire Wire Line
	1600 7200 1600 7100
$Comp
L Connector_Generic:Conn_02x23_Odd_Even P2
U 1 1 55DF7DE1
P 4250 6100
F 0 "P2" H 4250 7300 50  0000 C CNN
F 1 "BeagleBone_Black_Header" V 4300 6150 50  0000 C CNN
F 2 "Socket_BeagleBone_Black:Socket_BeagleBone_Black" H 4250 5250 60  0001 C CNN
F 3 "" H 4250 5250 60  0000 C CNN
	1    4250 6100
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_02x23_Odd_Even P1
U 1 1 55DF7DBA
P 1950 6100
F 0 "P1" H 1950 7300 50  0000 C CNN
F 1 "BeagleBone_Black_Header" V 2000 6150 50  0000 C CNN
F 2 "Socket_BeagleBone_Black:Socket_BeagleBone_Black" H 1950 5250 60  0001 C CNN
F 3 "" H 1950 5250 60  0000 C CNN
	1    1950 6100
	1    0    0    -1  
$EndComp
Wire Wire Line
	1750 5400 1650 5400
Text GLabel 1650 5400 0    50   Input ~ 0
PWR_BUT
Wire Wire Line
	2250 5400 2350 5400
Text GLabel 2350 5400 2    50   Input ~ 0
SYS_RESETN
Wire Wire Line
	2250 6600 2600 6600
$Comp
L pwr_BeagleBone:GNDA_ADC #PWR015
U 1 1 5589865D
P 2600 6600
F 0 "#PWR015" H 2600 6350 50  0001 C CNN
F 1 "GNDA_ADC" H 2600 6450 50  0000 C CNN
F 2 "" H 2600 6600 60  0000 C CNN
F 3 "" H 2600 6600 60  0000 C CNN
	1    2600 6600
	0    -1   -1   0   
$EndComp
Wire Wire Line
	2600 6500 2600 6400
Wire Wire Line
	2250 6500 2600 6500
$Comp
L pwr_BeagleBone:VDD_ADC #PWR014
U 1 1 558985D8
P 2600 6400
F 0 "#PWR014" H 2600 6250 50  0001 C CNN
F 1 "VDD_ADC" H 2600 6540 50  0000 C CNN
F 2 "" H 2600 6400 60  0000 C CNN
F 3 "" H 2600 6400 60  0000 C CNN
	1    2600 6400
	1    0    0    -1  
$EndComp
$Comp
L pwr_BeagleBone:SYS_5V #PWR02
U 1 1 558984AF
P 1150 4800
F 0 "#PWR02" H 1150 4650 50  0001 C CNN
F 1 "SYS_5V" H 1150 4940 50  0000 C CNN
F 2 "" H 1150 4800 60  0000 C CNN
F 3 "" H 1150 4800 60  0000 C CNN
	1    1150 4800
	1    0    0    -1  
$EndComp
$Comp
L pwr_BeagleBone:SYS_5V #PWR017
U 1 1 55898497
P 2850 4800
F 0 "#PWR017" H 2850 4650 50  0001 C CNN
F 1 "SYS_5V" H 2850 4940 50  0000 C CNN
F 2 "" H 2850 4800 60  0000 C CNN
F 3 "" H 2850 4800 60  0000 C CNN
	1    2850 4800
	1    0    0    -1  
$EndComp
Wire Wire Line
	1150 5300 1150 4800
Wire Wire Line
	1750 5300 1150 5300
Wire Wire Line
	1400 5200 1400 4800
Wire Wire Line
	1750 5200 1400 5200
Wire Wire Line
	1600 5100 1600 4800
Wire Wire Line
	1750 5100 1600 5100
$Comp
L power:+5V #PWR05
U 1 1 55897EF8
P 1400 4800
F 0 "#PWR05" H 1400 4650 50  0001 C CNN
F 1 "+5V" H 1400 4940 50  0000 C CNN
F 2 "" H 1400 4800 60  0000 C CNN
F 3 "" H 1400 4800 60  0000 C CNN
	1    1400 4800
	1    0    0    -1  
$EndComp
$Comp
L power:+3.3V #PWR07
U 1 1 55897EE7
P 1600 4800
F 0 "#PWR07" H 1600 4650 50  0001 C CNN
F 1 "+3V3" H 1600 4940 50  0000 C CNN
F 2 "" H 1600 4800 60  0000 C CNN
F 3 "" H 1600 4800 60  0000 C CNN
	1    1600 4800
	1    0    0    -1  
$EndComp
Wire Wire Line
	2850 5300 2250 5300
Wire Wire Line
	2850 4800 2850 5300
Wire Wire Line
	2600 5200 2600 4800
Wire Wire Line
	2250 5200 2600 5200
Wire Wire Line
	2400 5100 2400 4800
Wire Wire Line
	2250 5100 2400 5100
$Comp
L power:+5V #PWR013
U 1 1 55897A7F
P 2600 4800
F 0 "#PWR013" H 2600 4650 50  0001 C CNN
F 1 "+5V" H 2600 4940 50  0000 C CNN
F 2 "" H 2600 4800 60  0000 C CNN
F 3 "" H 2600 4800 60  0000 C CNN
	1    2600 4800
	1    0    0    -1  
$EndComp
$Comp
L power:+3.3V #PWR011
U 1 1 55897A67
P 2400 4800
F 0 "#PWR011" H 2400 4650 50  0001 C CNN
F 1 "+3V3" H 2400 4940 50  0000 C CNN
F 2 "" H 2400 4800 60  0000 C CNN
F 3 "" H 2400 4800 60  0000 C CNN
	1    2400 4800
	1    0    0    -1  
$EndComp
Wire Wire Line
	2000 7500 2000 7400
Wire Wire Line
	2250 7200 2400 7200
Connection ~ 1600 7200
Wire Wire Line
	1750 7200 1600 7200
Wire Wire Line
	1600 7100 1750 7100
Wire Wire Line
	1600 7400 1600 7200
Wire Wire Line
	2400 7100 2400 7200
Wire Wire Line
	2250 7100 2400 7100
$Comp
L power:GNDD #PWR010
U 1 1 5589785A
P 2000 7500
F 0 "#PWR010" H 2000 7250 50  0001 C CNN
F 1 "GNDD" H 2000 7350 50  0000 C CNN
F 2 "" H 2000 7500 60  0000 C CNN
F 3 "" H 2000 7500 60  0000 C CNN
	1    2000 7500
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C8
U 1 1 5F2101D8
P 5450 1200
F 0 "C8" H 5358 1154 50  0000 R CNN
F 1 "4.7uF" H 5358 1245 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 5488 1050 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2792994.pdf" H 5450 1200 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/c2012x5r1h475k125ab/cap-4-7-f-50v-10-x5r-0805/dp/2346932" H 5450 1200 50  0001 C CNN "Store"
	1    5450 1200
	-1   0    0    1   
$EndComp
$Comp
L power:GNDD #PWR031
U 1 1 5F210AA9
P 5450 1350
F 0 "#PWR031" H 5450 1100 50  0001 C CNN
F 1 "GNDD" H 5454 1195 50  0000 C CNN
F 2 "" H 5450 1350 50  0001 C CNN
F 3 "" H 5450 1350 50  0001 C CNN
	1    5450 1350
	1    0    0    -1  
$EndComp
Wire Wire Line
	5450 1050 5850 1050
Connection ~ 5850 1050
Wire Wire Line
	5450 3300 5450 3200
Wire Wire Line
	5450 3600 5450 3500
Connection ~ 5450 3600
Wire Wire Line
	5450 3700 5450 3600
Wire Wire Line
	5450 4000 5450 3900
Wire Wire Line
	5800 3600 5800 4100
Wire Wire Line
	5450 3600 5800 3600
Connection ~ 5200 4000
Wire Wire Line
	5450 4000 5200 4000
Connection ~ 5200 3200
Wire Wire Line
	5200 3200 5450 3200
Wire Wire Line
	5200 4000 5200 4100
Wire Wire Line
	5200 3100 5200 3200
Wire Wire Line
	5200 4000 5100 4000
$Comp
L power:GND #PWR032
U 1 1 5ED99871
P 5800 4100
F 0 "#PWR032" H 5800 3850 50  0001 C CNN
F 1 "GND" H 5805 3927 50  0000 C CNN
F 2 "" H 5800 4100 50  0001 C CNN
F 3 "" H 5800 4100 50  0001 C CNN
	1    5800 4100
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C7
U 1 1 5ED9986B
P 5450 3800
F 0 "C7" H 5358 3754 50  0000 R CNN
F 1 "0.1uF" H 5358 3845 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 5488 3650 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2802608.pdf" H 5450 3800 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/cga4j2x7r2a104k125aa/cap-0-1-f-100v-10-x7r-0805/dp/2210962" H 5450 3800 50  0001 C CNN "Store"
	1    5450 3800
	-1   0    0    1   
$EndComp
$Comp
L Device:C_Small C6
U 1 1 5ED99865
P 5450 3400
F 0 "C6" H 5358 3354 50  0000 R CNN
F 1 "0.1uF" H 5358 3445 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 5488 3250 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2802608.pdf" H 5450 3400 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/cga4j2x7r2a104k125aa/cap-0-1-f-100v-10-x7r-0805/dp/2210962" H 5450 3400 50  0001 C CNN "Store"
	1    5450 3400
	-1   0    0    1   
$EndComp
Wire Wire Line
	5100 3200 5200 3200
Wire Wire Line
	5100 3300 5100 3200
Wire Wire Line
	5100 4000 5100 3900
$Comp
L power:-15V #PWR030
U 1 1 5ED9985C
P 5200 4100
F 0 "#PWR030" H 5200 4200 50  0001 C CNN
F 1 "-15V" H 5215 4273 50  0000 C CNN
F 2 "" H 5200 4100 50  0001 C CNN
F 3 "" H 5200 4100 50  0001 C CNN
	1    5200 4100
	-1   0    0    1   
$EndComp
$Comp
L power:+15V #PWR029
U 1 1 5ED99856
P 5200 3100
F 0 "#PWR029" H 5200 2950 50  0001 C CNN
F 1 "+15V" H 5215 3273 50  0000 C CNN
F 2 "" H 5200 3100 50  0001 C CNN
F 3 "" H 5200 3100 50  0001 C CNN
	1    5200 3100
	1    0    0    -1  
$EndComp
Wire Wire Line
	4250 3600 4250 3700
Connection ~ 4250 3600
Wire Wire Line
	4600 3600 4250 3600
Wire Wire Line
	4600 4100 4600 3600
Wire Wire Line
	4250 3500 4250 3600
Connection ~ 4000 4000
Wire Wire Line
	4250 4000 4000 4000
Wire Wire Line
	4250 3900 4250 4000
Connection ~ 4000 3200
Wire Wire Line
	4250 3200 4250 3300
Wire Wire Line
	4000 3200 4250 3200
Wire Wire Line
	4000 4000 4000 4100
Wire Wire Line
	4000 3100 4000 3200
Wire Wire Line
	4000 4000 3900 4000
$Comp
L power:GND #PWR028
U 1 1 5EB8C549
P 4600 4100
F 0 "#PWR028" H 4600 3850 50  0001 C CNN
F 1 "GND" H 4605 3927 50  0000 C CNN
F 2 "" H 4600 4100 50  0001 C CNN
F 3 "" H 4600 4100 50  0001 C CNN
	1    4600 4100
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C5
U 1 1 5EB4DB7A
P 4250 3800
F 0 "C5" H 4158 3754 50  0000 R CNN
F 1 "0.1uF" H 4158 3845 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 4288 3650 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2802608.pdf" H 4250 3800 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/cga4j2x7r2a104k125aa/cap-0-1-f-100v-10-x7r-0805/dp/2210962" H 4250 3800 50  0001 C CNN "Store"
	1    4250 3800
	-1   0    0    1   
$EndComp
$Comp
L Device:C_Small C4
U 1 1 5EB4A73D
P 4250 3400
F 0 "C4" H 4158 3354 50  0000 R CNN
F 1 "0.1uF" H 4158 3445 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 4288 3250 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2802608.pdf" H 4250 3400 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/cga4j2x7r2a104k125aa/cap-0-1-f-100v-10-x7r-0805/dp/2210962" H 4250 3400 50  0001 C CNN "Store"
	1    4250 3400
	-1   0    0    1   
$EndComp
$Comp
L Memory_EEPROM:CAT24C256 U2
U 1 1 5E66F845
P 2550 1900
F 0 "U2" H 2300 2150 50  0000 C CNN
F 1 "CAT24C256" H 2800 2150 50  0000 C CNN
F 2 "Package_SO:TSSOP-8_4.4x3mm_P0.65mm" H 2550 1900 50  0001 C CNN
F 3 "https://www.onsemi.cn/PowerSolutions/document/CAT24C256-D.PDF" H 2550 1900 50  0001 C CNN
F 4 "https://uk.farnell.com/on-semiconductor/cat24c256yi-gt3/eeprom-256kbit-i2c-1mhz-tssop/dp/2464468?st=CAT24C256" H 2550 1900 50  0001 C CNN "Store"
	1    2550 1900
	1    0    0    -1  
$EndComp
$Comp
L Amplifier_Operational:TL084 U4
U 5 1 5EBC5547
P 5200 3600
F 0 "U4" H 5250 3650 50  0000 C CNN
F 1 "TL084" H 5300 3550 50  0000 C CNN
F 2 "Package_SO:TSSOP-14_4.4x5mm_P0.65mm" H 5150 3700 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/tl081.pdf" H 5250 3800 50  0001 C CNN
F 4 "https://uk.farnell.com/texas-instruments/tl084cpwr/ic-op-amp-3mhz-13v-us-tssop-14/dp/3005142?st=TL084" H 5200 3600 50  0001 C CNN "Store"
	5    5200 3600
	1    0    0    -1  
$EndComp
Wire Wire Line
	3900 3200 4000 3200
Wire Wire Line
	3900 3300 3900 3200
Wire Wire Line
	3900 4000 3900 3900
$Comp
L power:-15V #PWR025
U 1 1 5EAE9A8A
P 4000 4100
F 0 "#PWR025" H 4000 4200 50  0001 C CNN
F 1 "-15V" H 4015 4273 50  0000 C CNN
F 2 "" H 4000 4100 50  0001 C CNN
F 3 "" H 4000 4100 50  0001 C CNN
	1    4000 4100
	-1   0    0    1   
$EndComp
$Comp
L power:+15V #PWR024
U 1 1 5EAE8496
P 4000 3100
F 0 "#PWR024" H 4000 2950 50  0001 C CNN
F 1 "+15V" H 4015 3273 50  0000 C CNN
F 2 "" H 4000 3100 50  0001 C CNN
F 3 "" H 4000 3100 50  0001 C CNN
	1    4000 3100
	1    0    0    -1  
$EndComp
$Comp
L Amplifier_Operational:TL084 U3
U 5 1 5EA83E31
P 4000 3600
F 0 "U3" H 3958 3646 50  0000 L CNN
F 1 "TL084" H 3958 3555 50  0000 L CNN
F 2 "Package_SO:TSSOP-14_4.4x5mm_P0.65mm" H 3950 3700 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/tl081.pdf" H 4050 3800 50  0001 C CNN
F 4 "https://uk.farnell.com/texas-instruments/tl084cpwr/ic-op-amp-3mhz-13v-us-tssop-14/dp/3005142?st=TL084" H 4000 3600 50  0001 C CNN "Store"
	5    4000 3600
	1    0    0    -1  
$EndComp
NoConn ~ 4550 5100
NoConn ~ 4550 5200
NoConn ~ 4550 5300
NoConn ~ 4550 5400
NoConn ~ 4550 5500
NoConn ~ 4550 5600
NoConn ~ 4550 5700
NoConn ~ 4550 5800
NoConn ~ 4550 5900
NoConn ~ 4550 6000
NoConn ~ 4550 6100
NoConn ~ 4550 6200
NoConn ~ 4550 6300
NoConn ~ 4550 6400
NoConn ~ 4550 6500
NoConn ~ 4550 6600
NoConn ~ 4550 6700
NoConn ~ 4550 6800
NoConn ~ 4550 6900
NoConn ~ 4550 7000
NoConn ~ 4550 7100
NoConn ~ 4050 7100
NoConn ~ 4050 7000
NoConn ~ 4050 6900
NoConn ~ 4050 6800
NoConn ~ 4050 6700
NoConn ~ 4050 6600
NoConn ~ 4050 6500
NoConn ~ 4050 6400
NoConn ~ 4050 6300
NoConn ~ 4050 6200
NoConn ~ 4050 6100
NoConn ~ 4050 6000
NoConn ~ 4050 5900
NoConn ~ 4050 5800
NoConn ~ 4050 5100
NoConn ~ 4050 5200
NoConn ~ 4050 5300
NoConn ~ 4050 5400
NoConn ~ 4050 5500
NoConn ~ 4050 5600
NoConn ~ 4050 5700
NoConn ~ 2250 5500
NoConn ~ 2250 5600
NoConn ~ 2250 5700
NoConn ~ 2250 5800
NoConn ~ 1750 5700
NoConn ~ 1750 5600
NoConn ~ 1750 5500
NoConn ~ 1750 6000
NoConn ~ 1750 6100
NoConn ~ 1750 6200
NoConn ~ 1750 6300
NoConn ~ 2250 6300
NoConn ~ 2250 6200
NoConn ~ 2250 6100
NoConn ~ 2250 6700
NoConn ~ 2250 6800
NoConn ~ 2250 6900
NoConn ~ 2250 7000
NoConn ~ 1750 7000
NoConn ~ 1750 6900
NoConn ~ 1750 6800
NoConn ~ 1750 6700
NoConn ~ 1750 6600
NoConn ~ 2950 2000
Wire Wire Line
	9300 5000 9300 5250
$Comp
L Device:R_Small R21
U 1 1 5E92E104
P 9300 5350
F 0 "R21" H 9359 5396 50  0000 L CNN
F 1 "28K" H 9359 5305 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 9230 5350 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 9300 5350 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080528k0fkea/res-28k-1-0-125w-0805-thick-film/dp/2138988" H 9300 5350 50  0001 C CNN "Store"
	1    9300 5350
	1    0    0    -1  
$EndComp
$Comp
L Connector:TestPoint TP5
U 1 1 5EB20D0F
P 2800 3100
F 0 "TP5" H 2858 3172 50  0000 L CNN
F 1 "TestPoint" H 2858 3127 50  0001 L CNN
F 2 "hex_test_points:hex_test_pad_1mm" H 3000 3100 50  0001 C CNN
F 3 "~" H 3000 3100 50  0001 C CNN
	1    2800 3100
	1    0    0    -1  
$EndComp
$Comp
L Connector:TestPoint TP7
U 1 1 5EB2ED40
P 8300 3000
F 0 "TP7" H 8242 3026 50  0000 R CNN
F 1 "TestPoint" H 8242 3117 50  0000 R CNN
F 2 "hex_test_points:hex_test_pad_1mm" H 8500 3000 50  0001 C CNN
F 3 "~" H 8500 3000 50  0001 C CNN
	1    8300 3000
	-1   0    0    1   
$EndComp
Connection ~ 8300 3000
$Comp
L Connector:TestPoint TP11
U 1 1 5EB48437
P 9600 4600
F 0 "TP11" H 9658 4672 50  0000 L CNN
F 1 "TestPoint" H 9658 4627 50  0001 L CNN
F 2 "hex_test_points:hex_test_pad_2mm" H 9800 4600 50  0001 C CNN
F 3 "~" H 9800 4600 50  0001 C CNN
	1    9600 4600
	1    0    0    -1  
$EndComp
Wire Wire Line
	9600 4700 10300 4700
Wire Wire Line
	9600 4600 9600 4700
Wire Wire Line
	10300 4600 10300 4700
$Comp
L Connector:TestPoint TP8
U 1 1 5EB9A5D0
P 8400 2350
F 0 "TP8" H 8458 2422 50  0000 L CNN
F 1 "TestPoint" H 8458 2377 50  0001 L CNN
F 2 "hex_test_points:hex_test_pad_1mm" H 8600 2350 50  0001 C CNN
F 3 "~" H 8600 2350 50  0001 C CNN
	1    8400 2350
	1    0    0    -1  
$EndComp
Wire Wire Line
	8400 2550 8550 2550
$Comp
L Connector:TestPoint TP6
U 1 1 5EBCE32B
P 8200 1200
F 0 "TP6" H 8258 1272 50  0000 L CNN
F 1 "TestPoint" H 8258 1227 50  0001 L CNN
F 2 "hex_test_points:hex_test_pad_1mm" H 8400 1200 50  0001 C CNN
F 3 "~" H 8400 1200 50  0001 C CNN
	1    8200 1200
	1    0    0    -1  
$EndComp
Wire Wire Line
	8200 1200 8200 1350
$Comp
L Connector:TestPoint TP10
U 1 1 5EBE99EF
P 9450 2100
F 0 "TP10" H 9508 2172 50  0000 L CNN
F 1 "TestPoint" H 9508 2127 50  0001 L CNN
F 2 "hex_test_points:hex_test_pad_1mm" H 9650 2100 50  0001 C CNN
F 3 "~" H 9650 2100 50  0001 C CNN
	1    9450 2100
	1    0    0    -1  
$EndComp
$Comp
L Connector:TestPoint TP9
U 1 1 5EBEA0B6
P 9450 850
F 0 "TP9" H 9508 922 50  0000 L CNN
F 1 "TestPoint" H 9508 877 50  0001 L CNN
F 2 "hex_test_points:hex_test_pad_1mm" H 9650 850 50  0001 C CNN
F 3 "~" H 9650 850 50  0001 C CNN
	1    9450 850 
	1    0    0    -1  
$EndComp
Wire Wire Line
	9450 850  9450 950 
Connection ~ 9450 950 
Wire Wire Line
	9450 2100 9450 2200
Connection ~ 9450 2200
$Comp
L Connector:TestPoint TP12
U 1 1 5EC163AF
P 10450 1250
F 0 "TP12" H 10508 1322 50  0000 L CNN
F 1 "TestPoint" H 10508 1277 50  0001 L CNN
F 2 "hex_test_points:hex_test_pad_1mm" H 10650 1250 50  0001 C CNN
F 3 "~" H 10650 1250 50  0001 C CNN
	1    10450 1250
	1    0    0    -1  
$EndComp
$Comp
L Connector:TestPoint TP13
U 1 1 5EC1B2D3
P 10450 2450
F 0 "TP13" H 10508 2522 50  0000 L CNN
F 1 "TestPoint" H 10508 2477 50  0001 L CNN
F 2 "hex_test_points:hex_test_pad_1mm" H 10650 2450 50  0001 C CNN
F 3 "~" H 10650 2450 50  0001 C CNN
	1    10450 2450
	1    0    0    -1  
$EndComp
Wire Wire Line
	10450 2450 10450 2550
Wire Wire Line
	10450 1250 10450 1350
Wire Wire Line
	5450 1050 5450 1100
Wire Wire Line
	5450 1300 5450 1350
$Comp
L Connector:TestPoint TP2
U 1 1 5EE9C48F
P 1300 3150
F 0 "TP2" H 1250 3350 50  0000 L CNN
F 1 "TestPoint" H 1358 3177 50  0001 L CNN
F 2 "hex_test_points:hex_test_pad_2mm" H 1500 3150 50  0001 C CNN
F 3 "~" H 1500 3150 50  0001 C CNN
	1    1300 3150
	1    0    0    -1  
$EndComp
$Comp
L Connector:TestPoint TP1
U 1 1 5EEA2777
P 1300 3450
F 0 "TP1" H 1350 3650 50  0000 R CNN
F 1 "TestPoint" H 1242 3567 50  0001 R CNN
F 2 "hex_test_points:hex_test_pad_2mm" H 1500 3450 50  0001 C CNN
F 3 "~" H 1500 3450 50  0001 C CNN
	1    1300 3450
	-1   0    0    1   
$EndComp
Wire Wire Line
	1000 3400 1100 3400
Connection ~ 1100 3400
$Comp
L Connector:TestPoint TP3
U 1 1 5EF38653
P 1500 3150
F 0 "TP3" H 1450 3350 50  0000 L CNN
F 1 "TestPoint" H 1442 3267 50  0001 R CNN
F 2 "hex_test_points:hex_test_pad_2mm" H 1700 3150 50  0001 C CNN
F 3 "~" H 1700 3150 50  0001 C CNN
	1    1500 3150
	1    0    0    -1  
$EndComp
Wire Wire Line
	2550 950  2550 1000
Wire Wire Line
	2900 1000 2550 1000
Connection ~ 2550 1000
Wire Wire Line
	2550 1000 2550 1600
Wire Wire Line
	2900 1350 2900 1300
Wire Wire Line
	3550 950  3550 1100
$Comp
L power:-15V #PWR04
U 1 1 5EEF1700
P 1100 3500
F 0 "#PWR04" H 1100 3600 50  0001 C CNN
F 1 "-15V" H 1100 3650 50  0000 C CNN
F 2 "" H 1100 3500 50  0001 C CNN
F 3 "" H 1100 3500 50  0001 C CNN
	1    1100 3500
	-1   0    0    1   
$EndComp
Wire Wire Line
	1000 3200 1100 3200
Wire Wire Line
	1100 3100 1100 3200
Connection ~ 1100 3200
Wire Wire Line
	1300 3150 1300 3200
Wire Wire Line
	1100 3200 1300 3200
Wire Wire Line
	1300 3400 1300 3450
Wire Wire Line
	1100 3400 1300 3400
Wire Wire Line
	1100 3400 1100 3500
Wire Wire Line
	1000 3300 1500 3300
Wire Wire Line
	1500 3300 1500 3500
Wire Wire Line
	1500 3150 1500 3300
Connection ~ 1500 3300
Wire Wire Line
	8550 5350 8550 5500
Wire Wire Line
	8550 5500 8350 5500
Connection ~ 8350 5500
Wire Wire Line
	8350 5500 8350 5750
Wire Wire Line
	8550 5500 8550 5600
Connection ~ 8550 5500
Connection ~ 8550 5600
Wire Wire Line
	8550 5600 8450 5600
Wire Wire Line
	8400 2350 8400 2550
$Comp
L Analog_DAC:MCP4922-EST U5
U 1 1 5EBE8392
P 6500 1950
F 0 "U5" H 7144 1996 50  0000 L CNN
F 1 "MCP4922-EST" H 7144 1905 50  0000 L CNN
F 2 "Package_SO:TSSOP-14_4.4x5mm_P0.65mm" H 6500 1950 50  0001 C CIN
F 3 "http://ww1.microchip.com/downloads/en/devicedoc/21897a.pdf" H 6500 1950 50  0001 C CNN
F 4 "https://uk.farnell.com/microchip/mcp4922-e-st/dac-12bit-dual-2ch-spi-14tssop/dp/1834917?st=MCP4922" H 6500 1950 50  0001 C CNN "Store"
	1    6500 1950
	1    0    0    -1  
$EndComp
Wire Wire Line
	5900 2050 5800 2050
Wire Wire Line
	5900 1950 5800 1950
Wire Wire Line
	5900 1850 5800 1850
Wire Wire Line
	5900 1750 5800 1750
$Comp
L power:GND #PWR019
U 1 1 5ED114CF
P 6400 2550
F 0 "#PWR019" H 6400 2300 50  0001 C CNN
F 1 "GND" H 6405 2377 50  0000 C CNN
F 2 "" H 6400 2550 50  0001 C CNN
F 3 "" H 6400 2550 50  0001 C CNN
	1    6400 2550
	1    0    0    -1  
$EndComp
Wire Wire Line
	6400 2550 6400 2450
$Comp
L Device:R_Small R11
U 1 1 5ED68949
P 8650 2550
F 0 "R11" V 8709 2596 50  0000 L CNN
F 1 "3.9K" V 8709 2505 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 8580 2550 50  0001 C CNN
F 3 "" H 8650 2550 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw08053k90fkea/res-3k9-1-0-125w-0805-thick-film/dp/1652998?st=smd%20resisitor" H 8650 2550 50  0001 C CNN "Store"
	1    8650 2550
	0    -1   -1   0   
$EndComp
$Comp
L Device:R_Small R13
U 1 1 5ED68FD2
P 8650 2750
F 0 "R13" V 8709 2796 50  0000 L CNN
F 1 "16.2K" V 8709 2705 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 8580 2750 50  0001 C CNN
F 3 "" H 8650 2750 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080516k2fkea/res-16k2-1-0-125w-0805-thick-film/dp/1469878?st=smd%20resisitor" H 8650 2750 50  0001 C CNN "Store"
	1    8650 2750
	0    -1   -1   0   
$EndComp
$Comp
L Device:R_Small R16
U 1 1 5ED695D1
P 8850 2950
F 0 "R16" H 8909 2996 50  0000 L CNN
F 1 "11.3K" H 8909 2905 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 8780 2950 50  0001 C CNN
F 3 "" H 8850 2950 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080511k3fkea/res-11k3-1-0-125w-0805-thick-film/dp/1652911?st=smd%20resisitor" H 8850 2950 50  0001 C CNN "Store"
	1    8850 2950
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R18
U 1 1 5ED69C6A
P 9150 2200
F 0 "R18" V 9209 2246 50  0000 L CNN
F 1 "18.7K" V 9209 2155 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 9080 2200 50  0001 C CNN
F 3 "" H 9150 2200 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080518k7fkea/res-18k7-1-0-125w-0805-thick-film/dp/1652934?st=smd%20resisitor" H 9150 2200 50  0001 C CNN "Store"
	1    9150 2200
	0    -1   -1   0   
$EndComp
$Comp
L Device:R_Small R19
U 1 1 5ED6A3CB
P 9650 1450
F 0 "R19" V 9709 1496 50  0000 L CNN
F 1 "16.2K" V 9709 1405 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 9580 1450 50  0001 C CNN
F 3 "" H 9650 1450 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080516k2fkea/res-16k2-1-0-125w-0805-thick-film/dp/1469878?st=smd%20resisitor" H 9650 1450 50  0001 C CNN "Store"
	1    9650 1450
	0    -1   -1   0   
$EndComp
$Comp
L Device:R_Small R23
U 1 1 5ED6AB24
P 10100 1700
F 0 "R23" V 10159 1746 50  0000 L CNN
F 1 "16.2K" V 10159 1655 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 10030 1700 50  0001 C CNN
F 3 "" H 10100 1700 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080516k2fkea/res-16k2-1-0-125w-0805-thick-film/dp/1469878?st=smd%20resisitor" H 10100 1700 50  0001 C CNN "Store"
	1    10100 1700
	0    -1   -1   0   
$EndComp
$Comp
L Device:R_Small R22
U 1 1 5ED6B183
P 9650 2650
F 0 "R22" V 9709 2696 50  0000 L CNN
F 1 "16.2K" V 9709 2605 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 9580 2650 50  0001 C CNN
F 3 "" H 9650 2650 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080516k2fkea/res-16k2-1-0-125w-0805-thick-film/dp/1469878?st=smd%20resisitor" H 9650 2650 50  0001 C CNN "Store"
	1    9650 2650
	0    -1   -1   0   
$EndComp
$Comp
L Device:R_Small R24
U 1 1 5ED6B7B6
P 10100 2900
F 0 "R24" V 10159 2946 50  0000 L CNN
F 1 "16.2K" V 10159 2855 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 10030 2900 50  0001 C CNN
F 3 "" H 10100 2900 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw080516k2fkea/res-16k2-1-0-125w-0805-thick-film/dp/1469878?st=smd%20resisitor" H 10100 2900 50  0001 C CNN "Store"
	1    10100 2900
	0    -1   -1   0   
$EndComp
Wire Wire Line
	8400 2550 8150 2550
Wire Wire Line
	8150 2550 8150 2150
Wire Wire Line
	8150 2150 7100 2150
Connection ~ 8400 2550
Wire Wire Line
	8200 1350 8150 1350
Wire Wire Line
	8150 1350 8150 1750
Wire Wire Line
	8150 1750 7100 1750
Connection ~ 8200 1350
Wire Wire Line
	6400 1450 6400 1050
Connection ~ 6400 1050
$Comp
L Reference_Voltage:LM4040DBZ-2.0 U1
U 1 1 5EFCDFC7
P 2400 3450
F 0 "U1" V 2446 3362 50  0000 R CNN
F 1 "LM4040DBZ-2.0" V 2355 3362 50  0000 R CNN
F 2 "Package_TO_SOT_SMD:SOT-23" H 2400 3250 50  0001 C CIN
F 3 "http://www.ti.com/lit/ds/symlink/lm4040-n.pdf" H 2400 3450 50  0001 C CIN
F 4 "https://uk.farnell.com/texas-instruments/lm4040d20idbztg4/voltage-ref-shunt-2-048v-sot-23/dp/3124434" V 2400 3450 50  0001 C CNN "Store"
	1    2400 3450
	0    -1   -1   0   
$EndComp
$Comp
L power:+3.3V #PWR09
U 1 1 5EFD7F74
P 2400 2800
F 0 "#PWR09" H 2400 2650 50  0001 C CNN
F 1 "+3V3" H 2400 2940 50  0000 C CNN
F 2 "" H 2400 2800 60  0000 C CNN
F 3 "" H 2400 2800 60  0000 C CNN
	1    2400 2800
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R10
U 1 1 5F00A948
P 2400 3000
F 0 "R10" H 2459 3046 50  0000 L CNN
F 1 "2K" H 2459 2955 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" V 2330 3000 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2310790.pdf" H 2400 3000 50  0001 C CNN
F 4 "https://uk.farnell.com/vishay/crcw08052k00fkea/res-2k-1-0-125w-0805-thick-film/dp/1469884" H 2400 3000 50  0001 C CNN "Store"
	1    2400 3000
	1    0    0    -1  
$EndComp
Wire Wire Line
	2400 2900 2400 2850
Wire Wire Line
	2400 3100 2400 3200
$Comp
L power:GND #PWR012
U 1 1 5F06879C
P 2400 3700
F 0 "#PWR012" H 2400 3450 50  0001 C CNN
F 1 "GND" H 2405 3527 50  0000 C CNN
F 2 "" H 2400 3700 50  0001 C CNN
F 3 "" H 2400 3700 50  0001 C CNN
	1    2400 3700
	1    0    0    -1  
$EndComp
Wire Wire Line
	2400 3700 2400 3650
Text GLabel 3350 3200 2    50   Input ~ 0
VREF
Connection ~ 2400 3200
Wire Wire Line
	2400 3200 2400 3300
$Comp
L Device:C_Small C1
U 1 1 5F0C5864
P 2000 3450
F 0 "C1" H 1908 3404 50  0000 R CNN
F 1 "0.1uF" H 1908 3495 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 2038 3300 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2802608.pdf" H 2000 3450 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/cga4j2x7r2a104k125aa/cap-0-1-f-100v-10-x7r-0805/dp/2210962" H 2000 3450 50  0001 C CNN "Store"
	1    2000 3450
	-1   0    0    1   
$EndComp
$Comp
L Device:C_Small C2
U 1 1 5F0C8221
P 3200 3450
F 0 "C2" H 3108 3404 50  0000 R CNN
F 1 "0.1uF" H 3108 3495 50  0000 R CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 3238 3300 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2802608.pdf" H 3200 3450 50  0001 C CNN
F 4 "https://uk.farnell.com/tdk/cga4j2x7r2a104k125aa/cap-0-1-f-100v-10-x7r-0805/dp/2210962" H 3200 3450 50  0001 C CNN "Store"
	1    3200 3450
	-1   0    0    1   
$EndComp
Wire Wire Line
	2400 2850 2000 2850
Wire Wire Line
	2000 2850 2000 3350
Connection ~ 2400 2850
Wire Wire Line
	2400 2850 2400 2800
Wire Wire Line
	3200 3200 3200 3350
Wire Wire Line
	2400 3200 2800 3200
Wire Wire Line
	3350 3200 3200 3200
Connection ~ 3200 3200
Wire Wire Line
	2400 3650 2000 3650
Wire Wire Line
	2000 3650 2000 3550
Connection ~ 2400 3650
Wire Wire Line
	2400 3650 2400 3600
Wire Wire Line
	3200 3550 3200 3650
Wire Wire Line
	3200 3650 2400 3650
Text GLabel 7600 2650 0    50   Input ~ 0
VREF
Wire Wire Line
	2800 3100 2800 3200
Connection ~ 2800 3200
Wire Wire Line
	2800 3200 3200 3200
Wire Wire Line
	7600 2650 7700 2650
Text GLabel 6700 1350 2    50   Input ~ 0
VREF
Text GLabel 6700 2550 2    50   Input ~ 0
VREF
Wire Wire Line
	6700 2550 6600 2550
Wire Wire Line
	6600 2550 6600 2450
Wire Wire Line
	6700 1350 6600 1350
Wire Wire Line
	6600 1350 6600 1450
$Comp
L power:+3.3V #PWR0101
U 1 1 5ECDD8BF
P 5250 2050
F 0 "#PWR0101" H 5250 1900 50  0001 C CNN
F 1 "+3V3" H 5250 2190 50  0000 C CNN
F 2 "" H 5250 2050 60  0000 C CNN
F 3 "" H 5250 2050 60  0000 C CNN
	1    5250 2050
	1    0    0    -1  
$EndComp
Wire Wire Line
	5900 2150 5250 2150
Wire Wire Line
	5250 2150 5250 2050
NoConn ~ 4550 5000
NoConn ~ 4050 5000
NoConn ~ 1750 5000
NoConn ~ 2250 5000
$EndSCHEMATC
