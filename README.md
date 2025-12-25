# Suzuwave Analysis 💫

`Suzuwave` (or `Suzuha Signal Analysis`) is an opensource lightweight analysis tool for various (I hope so) types of signal files. In this environment you've able to perform a set of analysis operations such as

- Fast Fourier Transform
- Short Fourier Transform
- Haan Wavelet Transform
- Filtering

## Usage

The application provides a TUI for signal analysis. You can use the vim-like command system to control it. You can switch between different application modes, which gives you access to different functions.

![general usage](https://imgur.com/LqUxs5h)

### States

There are three states in the application:

- **Static Mode**: In this mode, you can control current chart view by moving chart and zooming it. Widgets except the chart view widget will be disabled.
- **Input Mode**: In this mode, you can use command console to input and editing commands in vim-like style. The full list of commands can be found in [this](README.md#commands) section.
- **Error Mode**: Submode of the Input Mode. Applied in case of error occurred while processing commands.
- **Explorer Mode**: In this mode, you can use chart explorer. Chart explorer allows you to iterate through opened chart views.

To switch between modes, use the following keys:

- `I` or `i`: Switch to Input Mode
- `E` or `e`: Switch to Explorer Mode
- `esc`: Return to Static Mode

### Commands

![commands](https://imgur.com/sBQjmox)

There are many commands available in the application. We can divide them into several categories:

#### General commands
| Command | Description |
| --- | --- |
| ce | Show or hide chart explorer |
| of | Open file |
| a | Show application's `about` information |
| h | Show commands table |
| q | Quit application |

#### Chart management commands
| Command | Description |
| --- | --- |
| zi | Enlarge chart |
| zo | Shrink chart |
| ml | Move chart left |
| mr | Move chart right |
| cwv | Close current chart view |
| swv | Move to the another chart view |

#### Chart transformation commands
| Command | Description |
| --- | --- |
| fft | Perform Fast Fourier Transform |
| sft | Perform Short-Time Fourier Transforms |
| hwt | Perform Haar Wavelet Transform |
| flp | Apply LowPass Filter |
| fhp | Apply HighPass Filter |
| fbp | Apply BandPass Filter |
| fbs | Apply BandStop Filter |

## Files
In current version application support loading binary files in BSUIR Vibric format. There are the specification.

Description of the file format for the signal:

1. File signature - **TMB1**: `4 bytes`, `text`
2. Number of channels: `4 bytes`, `integer` (number of channels through which the signal was received)
3. Sample size per channel: `4 bytes`, `integer` (number of discrete points per time interval of data reception (data block) **N**)
4. Number of spectral lines: `4 bytes`, `integer` (less than or equal to **N/2**)
5. Cutoff frequency: `4 bytes`, `integer` (specified cutoff frequency of the low-pass filter during data reception)
6. Frequency resolution: `4 bytes`, `real number` (frequency step between spectral lines during analysis, **Hz**)
7. Data block reception time: `4 bytes`, `real number` (time during which the data block was received, reciprocal of the frequency resolution)
8. Total data reception time: `4 bytes`, `integer` (reception time of the entire implementation in seconds)
9. Number of blocks received (specified by the user): `4 bytes`, `integer` (as specified by the user during data reception)
10. Data size: `4 bytes`, `integer` (number of discrete samples in the data file)
11. Number of blocks received (by the system): `4 bytes`, `integer` (actual number of blocks received)
12. Maximum value of received data: `4 bytes`, `real number` (maximum signal value)
13. Minimum value of received data: `4 bytes`, `real number` (minimum signal value)

This is followed by data in 4-byte format, a real number for one discrete signal value.

---
Enjoy your day with suzu:3

![suzuha](https://imgur.com/a/8IfKAVu)
