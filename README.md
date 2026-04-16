## Welcome to my Chip-8 emulator! There are many like it, but this one is mine :) 
*Written in Rust.*

This Chip-8 emulator is a complete/near complete representation of the original COSMAC VIP CHIP-8. It runs from the command line with a few options for configuration.

<img width="705" height="478" alt="image" src="https://github.com/user-attachments/assets/f33f4b7c-b98e-47f9-9a90-d6e0ba6a5c2f" />

# Features
*ch8 <path\to\ROM>*

***Configuration Options:***
|||
|--|--|
|*--frequency*|Set clock frequency in Hz. 1-1500. *(Default: 600)*  |
|*--scale*|Specify scaling for the 64x32 emulation window *(Default: 10)*|
|*--volume*|Set program volume 0-100 *(Default: 50)*|
|*--vsync*|Enable vertical sync *(Default: disabled)*|
|*--vy*|Addresses an ambiguous program instruction. May help if program isn't behaving correctly. *(Default: enabled)*|

# Prerequisites
**Required tools:**
 - Rust (obviously!)
 - CMake (any version above 4.0 seems to cause problems building SDL2, but workarounds exist.)
 - C Compiler (to compile SDL2) - MSVC, MinGW, etc.

# Build Instructions

    git clone https://github.com/BTubbs200/Another-CHIP-8-Emulator.git
    cd main
    cargo build --release
***Troubleshooting SDL2 and CMake 4.x.x***

If you are on CMake 4.x.x you will probably encounter a build error with SDL2 stating that compatibility with CMake versions older than 3.5 has been removed. This is because SDL2 build scripts utilize legacy CMake policies that 4.x.x no longer supports. You can attempt to bypass this by setting a temporary environment variable before building:

*PowerShell*

    $env:CMAKE_POLICY_VERSION_MINIMUM = "3.5"
    cargo build
If Rust Analyzer isn't playing nicely, you can try adding this to your IDE's *settings.json:*

    "rust-analyzer.cargo.extraEnv": { 
	    "CMAKE_POLICY_VERSION_MINIMUM": "3.5"
     }

