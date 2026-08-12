# STrack

A bare metal hardware profiler for embedded Rust (Embassy) on ARM Cortex-M. Measures real-time CPU cycle consumption and SRAM stack usage directly from hardware registers — no OS, no external tools, no overhead.

Built and tested on the STM32F446RE (Nucleo-F446RE).

---

## How It Works

**CPU Cycles — DWT CYCCNT**
The DWT unit's 32-bit cycle counter increments on every clock cycle. STrack samples the delta between polls using wrapping arithmetic to handle counter overflow correctly, and derives empirical CPU frequency from cycles-per-second.

**SRAM — Main Stack Pointer (MSP)**
The MSP register points to the current top of the stack. Since Cortex-M stacks grow downward, stack usage is:
```
sram_used = SRAM_TOP - MSP
```

Both values are polled every second and reported over RTT via `defmt`.

---

## Sample Output

```
[INFO] freq: 16 MHz | sram_used: 588 bytes | cycles: 16025120 
[INFO] freq: 16 MHz | sram_used: 588 bytes | cycles: 16025136 
```

---

## Hardware

| | |
|---|---|
| Board | STM32 Nucleo-F446RE |
| MCU | STM32F446RE (Cortex-M4, 180MHz max) |
| SRAM | 128KB |
| Debug | ST-Link v2 (on-board) |

Portable to any STM32 Cortex-M4 target with minor changes to `SRAM_TOP` and `memory.x`.

---

## Planned

- [ ] Per-task profiling for multi-task Embassy applications
- [ ] Stack high-water mark via stack painting
- [ ] Struct-based crate API for drop-in use across projects
- [ ] Real-time visualization via Python host-side script

---

## Background

Built from first principles — every design decision, from wrapping arithmetic for CYCCNT overflow to MSP-based stack measurement, was derived directly from the STM32F446RE reference manual and ARM Cortex-M4 architecture documentation. The goal is to observe what the hardware is actually doing, not what an abstraction layer reports.
