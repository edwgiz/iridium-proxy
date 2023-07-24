Generate SSL certificate 

```shell
openssl genrsa -traditional -out key.pem 4096
openssl req -new -x509 -key key.pem -out cert.pem -config openssl.cnf -days 99999

```

```text
root# dmesg
Booting Linux on physical CPU 0x0
Linux version 4.1.15 (umc@build-s02.umctec.ru) (gcc version 6.3.1 20170109 (Linaro GCC 6.3-2017.02) ) #1 SMP PREEMPT Wed Apr 25 13:38:59 +05 2018
CPU: ARMv7 Processor [412fc09a] revision 10 (ARMv7), cr=10c53c7d
CPU: PIPT / VIPT nonaliasing data cache, VIPT aliasing instruction cache
Machine model: UMC C3
Reserved memory: created CMA memory pool at 0x1c000000, size 320 MiB
Reserved memory: initialized node linux,cma, compatible id shared-dma-pool
Memory policy: Data cache writeback
On node 0 totalpages: 131072
free_area_init_node: node 0, pgdat 80a5a3c0, node_mem_map 8bb57000
  Normal zone: 1024 pages used for memmap
  Normal zone: 0 pages reserved
  Normal zone: 131072 pages, LIFO batch:31
CPU: All CPU(s) started in SVC mode.
PERCPU: Embedded 12 pages/cpu @8bb22000 s16908 r8192 d24052 u49152
pcpu-alloc: s16908 r8192 d24052 u49152 alloc=12*4096
pcpu-alloc: [0] 0 [0] 1 
Built 1 zonelists in Zone order, mobility grouping on.  Total pages: 130048
Kernel command line: console=ttymxc0,115200 root=/dev/mmcblk2p1 rootwait ro
PID hash table entries: 2048 (order: 1, 8192 bytes)
Dentry cache hash table entries: 65536 (order: 6, 262144 bytes)
Inode-cache hash table entries: 32768 (order: 5, 131072 bytes)
Memory: 180168K/524288K available (7458K kernel code, 333K rwdata, 2468K rodata, 312K init, 431K bss, 16440K reserved, 327680K cma-reserved, 0K highmem)
Virtual kernel memory layout:
    vector  : 0xffff0000 - 0xffff1000   (   4 kB)
    fixmap  : 0xffc00000 - 0xfff00000   (3072 kB)
    vmalloc : 0xa0800000 - 0xff000000   (1512 MB)
    lowmem  : 0x80000000 - 0xa0000000   ( 512 MB)
    pkmap   : 0x7fe00000 - 0x80000000   (   2 MB)
    modules : 0x7f000000 - 0x7fe00000   (  14 MB)
      .text : 0x80008000 - 0x809b9bb0   (9927 kB)
      .init : 0x809ba000 - 0x80a08000   ( 312 kB)
      .data : 0x80a08000 - 0x80a5b580   ( 334 kB)
       .bss : 0x80a5e000 - 0x80ac9e10   ( 432 kB)
SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=2, Nodes=1
Preemptible hierarchical RCU implementation.
	Additional per-CPU info printed with stalls.
	RCU restricting CPUs from NR_CPUS=4 to nr_cpu_ids=2.
RCU: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=2
NR_IRQS:16 nr_irqs:16 16
L2C-310 erratum 769419 enabled
L2C-310 enabling early BRESP for Cortex-A9
L2C-310 full line of zeros enabled for Cortex-A9
L2C-310 ID prefetch enabled, offset 16 lines
L2C-310 dynamic clock gating enabled, standby mode enabled
L2C-310 cache controller enabled, 16 ways, 512 kB
L2C-310: CACHE_ID 0x410000c8, AUX_CTRL 0x76050001
mxc_clocksource_init 3000000
Switching to timer-based delay loop, resolution 333ns
sched_clock: 32 bits at 3000kHz, resolution 333ns, wraps every 715827882841ns
clocksource mxc_timer1: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 637086815595 ns
Console: colour dummy device 80x30
Calibrating delay loop (skipped), value calculated using timer frequency.. 6.00 BogoMIPS (lpj=30000)
pid_max: default: 32768 minimum: 301
Mount-cache hash table entries: 1024 (order: 0, 4096 bytes)
Mountpoint-cache hash table entries: 1024 (order: 0, 4096 bytes)
CPU: Testing write buffer coherency: ok
CPU0: thread -1, cpu 0, socket 0, mpidr 80000000
Setting up static identity map for 0x10008280 - 0x100082d8
Brought up 1 CPUs
SMP: Total of 1 processors activated (6.00 BogoMIPS).
CPU: All CPU(s) started in SVC mode.
devtmpfs: initialized
VFP support v0.3: implementor 41 architecture 3 part 30 variant 9 rev 4
clocksource jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 19112604462750000 ns
pinctrl core: initialized pinctrl subsystem
NET: Registered protocol family 16
DMA: preallocated 256 KiB pool for atomic coherent allocations
cpuidle: using governor ladder
cpuidle: using governor menu
CPU identified as i.MX6DL, silicon rev 1.3
hw-breakpoint: found 5 (+1 reserved) breakpoint and 1 watchpoint registers.
hw-breakpoint: maximum watchpoint size is 4 bytes.
imx6dl-pinctrl 20e0000.iomuxc: initialized IMX pinctrl driver
mxs-dma 110000.dma-apbh: initialized
vgaarb: loaded
SCSI subsystem initialized
libata version 3.00 loaded.
usbcore: registered new interface driver usbfs
usbcore: registered new interface driver hub
usbcore: registered new device driver usb
2000000.aips-bus:usbphy_nop1 supply vcc not found, using dummy regulator
2000000.aips-bus:usbphy_nop2 supply vcc not found, using dummy regulator
i2c i2c-1: IMX I2C adapter registered
i2c i2c-1: can't use DMA
i2c i2c-2: IMX I2C adapter registered
i2c i2c-2: can't use DMA
Linux video capture interface: v2.00
pps_core: LinuxPPS API ver. 1 registered
pps_core: Software ver. 5.3.6 - Copyright 2005-2007 Rodolfo Giometti <giometti@linux.it>
PTP clock support registered
imx-ipuv3 2400000.ipu: IPU DMFC NORMAL mode: 1(0~1), 5B(4,5), 5F(6,7)
MIPI CSI2 driver module loaded
Advanced Linux Sound Architecture Driver Initialized.
Bluetooth: Core ver 2.20
NET: Registered protocol family 31
Bluetooth: HCI device and connection manager initialized
Bluetooth: HCI socket layer initialized
Bluetooth: L2CAP socket layer initialized
Bluetooth: SCO socket layer initialized
Switched to clocksource mxc_timer1
NET: Registered protocol family 2
TCP established hash table entries: 4096 (order: 2, 16384 bytes)
TCP bind hash table entries: 4096 (order: 3, 32768 bytes)
TCP: Hash tables configured (established 4096 bind 4096)
UDP hash table entries: 256 (order: 1, 8192 bytes)
UDP-Lite hash table entries: 256 (order: 1, 8192 bytes)
NET: Registered protocol family 1
RPC: Registered named UNIX socket transport module.
RPC: Registered udp transport module.
RPC: Registered tcp transport module.
RPC: Registered tcp NFSv4.1 backchannel transport module.
PCI: CLS 0 bytes, default 64
CPU PMU: Failed to parse /soc/pmu/interrupt-affinity[0]
hw perfevents: enabled with armv7_cortex_a9 PMU driver, 7 counters available
Bus freq driver module loaded
futex hash table entries: 512 (order: 3, 32768 bytes)
VFS: Disk quotas dquot_6.6.0
VFS: Dquot-cache hash table entries: 1024 (order 0, 4096 bytes)
NFS: Registering the id_resolver key type
Key type id_resolver registered
Key type id_legacy registered
fuse init (API version 7.23)
io scheduler noop registered
io scheduler deadline registered
io scheduler cfq registered (default)
imx-weim 21b8000.weim: Driver registered.
MIPI DSI driver module loaded
imx-sdma 20ec000.sdma: no iram assigned, using external mem
imx-sdma 20ec000.sdma: no event needs to be remapped
imx-sdma 20ec000.sdma: loaded firmware 3.3
imx-sdma 20ec000.sdma: initialized
2020000.serial: ttymxc0 at MMIO 0x2020000 (irq = 24, base_baud = 5000000) is a IMX
console [ttymxc0] enabled
imx-uart 21e8000.serial: ttymxc1 rs485 enabled, gpio 120
21e8000.serial: ttymxc1 at MMIO 0x21e8000 (irq = 294, base_baud = 5000000) is a IMX
imx-uart 21ec000.serial: ttymxc2 rs485 enabled, gpio 121
21ec000.serial: ttymxc2 at MMIO 0x21ec000 (irq = 295, base_baud = 5000000) is a IMX
imx-uart 21f0000.serial: ttymxc3 rs485 enabled, gpio 122
21f0000.serial: ttymxc3 at MMIO 0x21f0000 (irq = 296, base_baud = 5000000) is a IMX
imx-uart 21f4000.serial: ttymxc4 rs485 enabled, gpio 123
21f4000.serial: ttymxc4 at MMIO 0x21f4000 (irq = 297, base_baud = 5000000) is a IMX
[drm] Initialized drm 1.1.0 20060810
[drm] Initialized vivante 1.0.0 20120216 on minor 0
brd: module loaded
loop: module loaded
spi_imx 2008000.ecspi: probed
spi_imx 2014000.ecspi: probed
CAN device driver interface
2090000.flexcan supply xceiver not found, using dummy regulator
flexcan 2090000.flexcan: device registered (reg_base=a09d8000, irq=29)
2094000.flexcan supply xceiver not found, using dummy regulator
flexcan 2094000.flexcan: device registered (reg_base=a09e0000, irq=30)
2188000.ethernet supply phy not found, using dummy regulator
pps pps0: new PPS source ptp0
libphy: fec_enet_mii_bus: probed
fec 2188000.ethernet eth0: registered PHC device 0
ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
ehci-pci: EHCI PCI platform driver
ehci-mxc: Freescale On-Chip EHCI Host driver
usbcore: registered new interface driver usb-storage
usbcore: registered new interface driver usb_ehset_test
2184800.usbmisc supply vbus-wakeup not found, using dummy regulator
2184000.usb supply vbus not found, using dummy regulator
2184200.usb supply vbus not found, using dummy regulator
ci_hdrc ci_hdrc.1: EHCI Host Controller
ci_hdrc ci_hdrc.1: new USB bus registered, assigned bus number 1
ci_hdrc ci_hdrc.1: USB 2.0 started, EHCI 1.00
hub 1-0:1.0: USB hub found
hub 1-0:1.0: 1 port detected
mousedev: PS/2 mouse device common for all mice
rtc-m41t80 1-0068: rtc core: registered m41t65 as rtc0
snvs_rtc 20cc000.snvs:snvs-rtc-lp: rtc core: registered 20cc000.snvs:snvs-r as rtc1
i2c /dev entries driver
IR NEC protocol handler initialized
IR RC5(x/sz) protocol handler initialized
IR RC6 protocol handler initialized
IR JVC protocol handler initialized
IR Sony protocol handler initialized
IR SANYO protocol handler initialized
IR Sharp protocol handler initialized
IR MCE Keyboard/mouse protocol handler initialized
IR XMP protocol handler initialized
Driver for 1-wire Dallas network protocol.
watchdog: imx2+ watchdog: cannot register miscdev on minor=130 (err=-16).
watchdog: imx2+ watchdog: a legacy watchdog module is probably present.
imx2-wdt 20bc000.wdog: timeout 60 sec (nowayout=0)
Bluetooth: HCI UART driver ver 2.3
Bluetooth: HCI UART protocol H4 registered
Bluetooth: HCI UART protocol BCSP registered
Bluetooth: HCI UART protocol ATH3K registered
usbcore: registered new interface driver bcm203x
usbcore: registered new interface driver btusb
usbcore: registered new interface driver ath3k
sdhci: Secure Digital Host Controller Interface driver
sdhci: Copyright(c) Pierre Ossman
sdhci-pltfm: SDHCI platform and OF driver helper
/soc/aips-bus@02100000/usdhc@02190000: voltage-ranges unspecified
sdhci-esdhc-imx 2190000.usdhc: Got CD GPIO
sdhci-esdhc-imx 2190000.usdhc: No vqmmc regulator found
mmc0: SDHCI controller on 2190000.usdhc [2190000.usdhc] using ADMA
/soc/aips-bus@02100000/usdhc@02198000: voltage-ranges unspecified
sdhci-esdhc-imx 2198000.usdhc: No vqmmc regulator found
mmc2: SDHCI controller on 2198000.usdhc [2198000.usdhc] using ADMA
mxc_vpu 2040000.vpu_fsl: VPU initialized
mxc_vdoa 21e4000.vdoa: i.MX Video Data Order Adapter(VDOA) driver probed
galcore: clk_get vg clock failed, disable vg!
Galcore version 5.0.11.41671
usb 1-1: new high-speed USB device number 2 using ci_hdrc
mmc2: MAN_BKOPS_EN bit is not set
mmc2: new DDR MMC card at address 0001
mmcblk2: mmc2:0001 4YMD3R 3.64 GiB 
 mmcblk2: p1 p2 p3 p4
hub 1-1:1.0: USB hub found
hub 1-1:1.0: 4 ports detected
caam 2100000.caam: Entropy delay = 3200
caam 2100000.caam: Instantiated RNG4 SH0
caam 2100000.caam: Instantiated RNG4 SH1
caam 2100000.caam: device ID = 0x0a16010000000100 (Era -524)
caam 2100000.caam: job rings = 2, qi = 0
caam algorithms registered in /proc/crypto
caam_jr 2101000.jr0: registering rng-caam
platform caam_sm: blkkey_ex: 4 keystore units available
platform caam_sm: 64-bit clear key:
platform caam_sm: [0000] 00 01 02 03 04 0f 06 07
platform caam_sm: 64-bit black key:
platform caam_sm: [0000] 2a 54 4e f0 80 36 9c 50
platform caam_sm: [0008] 4e b5 3b 84 7d 88 d1 31
platform caam_sm: 128-bit clear key:
platform caam_sm: [0000] 00 01 02 03 04 0f 06 07
platform caam_sm: [0008] 08 09 0a 0b 0c 0d 0e 0f
platform caam_sm: 128-bit black key:
platform caam_sm: [0000] 7e 41 7f ff bb 03 0b 2a
platform caam_sm: [0008] b5 3c 60 ef ab 93 57 d3
platform caam_sm: 192-bit clear key:
platform caam_sm: [0000] 00 01 02 03 04 0f 06 07
platform caam_sm: [0008] 08 09 0a 0b 0c 0d 0e 0f
platform caam_sm: [0016] 10 11 12 13 14 15 16 17
platform caam_sm: 192-bit black key:
platform caam_sm: [0000] a7 2a 35 47 c0 03 09 f6
platform caam_sm: [0008] c4 c7 13 83 ef f1 89 fa
platform caam_sm: [0016] 5b 48 d1 7b 72 6f fa e1
platform caam_sm: [0024] 2f 5d 1f 74 59 2d a7 d6
platform caam_sm: 256-bit clear key:
platform caam_sm: [0000] 00 01 02 03 04 0f 06 07
platform caam_sm: [0008] 08 09 0a 0b 0c 0d 0e 0f
platform caam_sm: [0016] 10 11 12 13 14 15 16 17
platform caam_sm: [0024] 18 19 1a 1b 1c 1d 1e 1f
platform caam_sm: 256-bit black key:
platform caam_sm: [0000] 44 b7 89 e5 90 19 61 2a
platform caam_sm: [0008] e6 50 0a f8 01 e7 78 c3
platform caam_sm: [0016] 58 63 fa f0 6b ff 42 2b
platform caam_sm: [0024] f4 a2 58 99 06 82 e6 c5
platform caam_sm: 64-bit unwritten blob:
platform caam_sm: [0000] 00 00 00 00 00 00 00 00
platform caam_sm: [0008] 00 00 00 00 00 00 00 00
platform caam_sm: [0016] 00 00 00 00 00 00 00 00
platform caam_sm: [0024] 00 00 00 00 00 00 00 00
platform caam_sm: [0032] 00 00 00 00 00 00 00 00
platform caam_sm: [0040] 00 00 00 00 00 00 00 00
platform caam_sm: [0048] 00 00 00 00 00 00 00 00
platform caam_sm: [0056] 00 00 00 00 00 00 00 00
platform caam_sm: [0064] 00 00 00 00 00 00 00 00
platform caam_sm: [0072] 00 00 00 00 00 00 00 00
platform caam_sm: [0080] 00 00 00 00 00 00 00 00
platform caam_sm: [0088] 00 00 00 00 00 00 00 00
platform caam_sm: 128-bit unwritten blob:
platform caam_sm: [0000] 00 00 00 00 00 00 00 00
platform caam_sm: [0008] 00 00 00 00 00 00 00 00
platform caam_sm: [0016] 00 00 00 00 00 00 00 00
platform caam_sm: [0024] 00 00 00 00 00 00 00 00
platform caam_sm: [0032] 00 00 00 00 00 00 00 00
platform caam_sm: [0040] 00 00 00 00 00 00 00 00
platform caam_sm: [0048] 00 00 00 00 00 00 00 00
platform caam_sm: [0056] 00 00 00 00 00 00 00 00
platform caam_sm: [0064] 00 00 00 00 00 00 00 00
platform caam_sm: [0072] 00 00 00 00 00 00 00 00
platform caam_sm: [0080] 00 00 00 00 00 00 00 00
platform caam_sm: [0088] 00 00 00 00 00 00 00 00
platform caam_sm: 196-bit unwritten blob:
platform caam_sm: [0000] 00 00 00 00 00 00 00 00
platform caam_sm: [0008] 00 00 00 00 00 00 00 00
platform caam_sm: [0016] 00 00 00 00 00 00 00 00
platform caam_sm: [0024] 00 00 00 00 00 00 00 00
platform caam_sm: [0032] 00 00 00 00 00 00 00 00
platform caam_sm: [0040] 00 00 00 00 00 00 00 00
platform caam_sm: [0048] 00 00 00 00 00 00 00 00
platform caam_sm: [0056] 00 00 00 00 00 00 00 00
platform caam_sm: [0064] 00 00 00 00 00 00 00 00
platform caam_sm: [0072] 00 00 00 00 00 00 00 00
platform caam_sm: [0080] 00 00 00 00 00 00 00 00
platform caam_sm: [0088] 00 00 00 00 00 00 00 00
platform caam_sm: 256-bit unwritten blob:
platform caam_sm: [0000] 00 00 00 00 00 00 00 00
platform caam_sm: [0008] 00 00 00 00 00 00 00 00
platform caam_sm: [0016] 00 00 00 00 00 00 00 00
platform caam_sm: [0024] 00 00 00 00 00 00 00 00
platform caam_sm: [0032] 00 00 00 00 00 00 00 00
platform caam_sm: [0040] 00 00 00 00 00 00 00 00
platform caam_sm: [0048] 00 00 00 00 00 00 00 00
platform caam_sm: [0056] 00 00 00 00 00 00 00 00
platform caam_sm: [0064] 00 00 00 00 00 00 00 00
platform caam_sm: [0072] 00 00 00 00 00 00 00 00
platform caam_sm: [0080] 00 00 00 00 00 00 00 00
platform caam_sm: [0088] 00 00 00 00 00 00 00 00
platform caam_sm: 64-bit black key in blob:
platform caam_sm: [0000] c7 c9 4a d9 f2 35 79 9f
platform caam_sm: [0008] 5b e0 3a f8 ed 9e e7 f0
platform caam_sm: [0016] ce 05 09 73 47 2d 3e 56
platform caam_sm: [0024] c9 e6 a0 fe d0 15 4e c3
platform caam_sm: [0032] f0 88 73 67 0e 27 dd da
platform caam_sm: [0040] 70 84 d4 83 fe d8 f2 e0
platform caam_sm: [0048] 1b 17 68 18 0c 21 65 83
platform caam_sm: [0056] 00 00 00 00 00 00 00 00
platform caam_sm: [0064] 00 00 00 00 00 00 00 00
platform caam_sm: [0072] 00 00 00 00 00 00 00 00
platform caam_sm: [0080] 00 00 00 00 00 00 00 00
platform caam_sm: [0088] 00 00 00 00 00 00 00 00
platform caam_sm: 128-bit black key in blob:
platform caam_sm: [0000] df 33 29 4d 8b 67 f6 e1
platform caam_sm: [0008] 56 69 4e f4 54 7c f4 4c
platform caam_sm: [0016] 4a 50 81 35 53 72 8b 99
platform caam_sm: [0024] 82 5f 22 c8 c7 96 9a ac
platform caam_sm: [0032] 65 47 59 c5 91 a6 5c f8
platform caam_sm: [0040] 7c 66 aa 70 ad 00 2d 6d
platform caam_sm: [0048] 47 31 2e f9 9b f8 e8 93
platform caam_sm: [0056] 5c 50 5a 94 8a 6d 6a a3
platform caam_sm: [0064] 00 00 00 00 00 00 00 00
platform caam_sm: [0072] 00 00 00 00 00 00 00 00
platform caam_sm: [0080] 00 00 00 00 00 00 00 00
platform caam_sm: [0088] 00 00 00 00 00 00 00 00
platform caam_sm: 192-bit black key in blob:
platform caam_sm: [0000] b6 2c b4 91 33 2b 87 70
platform caam_sm: [0008] 56 75 21 71 8f 49 56 89
platform caam_sm: [0016] 1a c2 f5 71 9c 4d c8 b2
platform caam_sm: [0024] f9 f6 22 78 18 61 90 a8
platform caam_sm: [0032] 19 c9 73 00 86 44 8f ce
platform caam_sm: [0040] 16 06 0b 98 66 fa bd 9d
platform caam_sm: [0048] 39 2d ab aa 6f 50 42 58
platform caam_sm: [0056] e9 18 cd ea aa 9d a3 c6
platform caam_sm: [0064] 50 4f e5 cd b2 9c 21 4b
platform caam_sm: [0072] 00 00 00 00 00 00 00 00
platform caam_sm: [0080] 00 00 00 00 00 00 00 00
platform caam_sm: [0088] 00 00 00 00 00 00 00 00
platform caam_sm: 256-bit black key in blob:
platform caam_sm: [0000] 74 ee b5 cb a6 b8 f2 75
platform caam_sm: [0008] 43 51 6e 70 90 10 4a 1e
platform caam_sm: [0016] 4d 28 1e d6 56 bd 38 6a
platform caam_sm: [0024] 77 87 ec 9c 23 4c 0d 17
platform caam_sm: [0032] ac 1f e1 eb f5 22 79 71
platform caam_sm: [0040] 17 f3 bf 59 ef 20 22 3e
platform caam_sm: [0048] 57 9a dc ec e4 52 22 d2
platform caam_sm: [0056] 80 ac fe ee 77 79 c9 e0
platform caam_sm: [0064] f2 13 82 21 5c d8 35 0a
platform caam_sm: [0072] e7 8a 67 b2 9e 4d 63 42
platform caam_sm: [0080] 00 00 00 00 00 00 00 00
platform caam_sm: [0088] 00 00 00 00 00 00 00 00
platform caam_sm: restored 64-bit black key:
platform caam_sm: [0000] c1 3b b1 8d 27 90 5f de
platform caam_sm: [0008] 63 7e e3 ce fd f8 e4 d2
platform caam_sm: restored 128-bit black key:
platform caam_sm: [0000] 7e 41 7f ff bb 03 0b 2a
platform caam_sm: [0008] b5 3c 60 ef ab 93 57 d3
platform caam_sm: restored 192-bit black key:
platform caam_sm: [0000] a7 2a 35 47 c0 03 09 f6
platform caam_sm: [0008] c4 c7 13 83 ef f1 89 fa
platform caam_sm: [0016] 6e 39 4e 47 ad 5c f1 28
platform caam_sm: [0024] 08 d1 2a 6e 53 05 66 dd
platform caam_sm: restored 256-bit black key:
platform caam_sm: [0000] 44 b7 89 e5 90 19 61 2a
platform caam_sm: [0008] e6 50 0a f8 01 e7 78 c3
platform caam_sm: [0016] 58 63 fa f0 6b ff 42 2b
platform caam_sm: [0024] f4 a2 58 99 06 82 e6 c5
snvs-secvio 20cc000.caam-snvs: can't get snvs clock
snvs-secvio 20cc000.caam-snvs: violation handlers armed - non-secure state
usbcore: registered new interface driver usbhid
usbhid: USB HID core driver
fsl-asrc 2034000.asrc: driver registered
NET: Registered protocol family 26
ip_tables: (C) 2000-2006 Netfilter Core Team
NET: Registered protocol family 10
sit: IPv6 over IPv4 tunneling driver
NET: Registered protocol family 17
can: controller area network core (rev 20120528 abi 9)
NET: Registered protocol family 29
can: raw protocol (rev 20120528)
can: broadcast manager protocol (rev 20120528 t)
can: netlink gateway (rev 20130117) max_hops=1
Bluetooth: RFCOMM TTY layer initialized
Bluetooth: RFCOMM socket layer initialized
Bluetooth: RFCOMM ver 1.11
Bluetooth: BNEP (Ethernet Emulation) ver 1.3
Bluetooth: BNEP filters: protocol multicast
Bluetooth: BNEP socket layer initialized
Bluetooth: HIDP (Human Interface Emulation) ver 1.2
Bluetooth: HIDP socket layer initialized
8021q: 802.1Q VLAN Support v1.8
Key type dns_resolver registered
rtc-m41t80 1-0068: setting system clock to 2023-07-07 13:53:35 UTC (1688738015)
ALSA device list:
  No soundcards found.
EXT4-fs (mmcblk2p1): mounting ext3 file system using the ext4 subsystem
EXT4-fs (mmcblk2p1): mounted filesystem with ordered data mode. Opts: (null)
VFS: Mounted root (ext3 filesystem) readonly on device 179:1.
devtmpfs: mounted
Freeing unused kernel memory: 312K (809ba000 - 80a08000)
EXT4-fs (mmcblk2p4): recovery complete
EXT4-fs (mmcblk2p4): mounted filesystem with ordered data mode. Opts: (null)
devpts: called with bogus options
EXT4-fs (mmcblk2p3): recovery complete
EXT4-fs (mmcblk2p3): mounted filesystem with ordered data mode. Opts: (null)
udevd[192]: starting version 3.2.2
random: udevd urandom read with 24 bits of entropy available
udevd[193]: starting eudev-3.2.2
fec 2188000.ethernet eth0: Freescale FEC PHY driver [SMSC LAN8710/LAN8720] (mii_bus:phy_addr=2188000.ethernet:00, irq=-1)
IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
flexcan 2090000.flexcan can0: bit-timing not yet defined
flexcan 2094000.flexcan can1: bit-timing not yet defined
fec 2188000.ethernet eth0: Link is Up - 100Mbps/Full - flow control off
IPv6: ADDRCONF(NETDEV_CHANGE): eth0: link becomes ready
random: nonblocking pool is initialized
```

```text
lib/libc.so.6
GNU C Library (GNU libc) stable release version 2.23, by Roland McGrath et al.
Copyright (C) 2016 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.
There is NO warranty; not even for MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.
Compiled by GNU CC version 6.3.1 20170109.
Available extensions:
crypt add-on version 2.1 by Michael Glad and others
GNU Libidn by Simon Josefsson
Native POSIX Threads Library by Ulrich Drepper et al
BIND-8.2.3-T5B
libc ABIs: UNIQUE
For bug reporting instructions, please see:
<http://www.gnu.org/software/libc/bugs.html>.
```