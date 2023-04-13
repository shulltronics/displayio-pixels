Rust Pixels Display for CircuitPython
=====================================
#### By Carsten Thue-Bludworth, 2023

`displayio_pixels` is a CircuitPython compatible displayio `Display` that targets a hardware-accelerated pixel buffer provided by the Rust crate `pixels`. The package is written in Rust and bindings to Python are produced with `pyo3`. This allows CircuitPython scripts to display a displayio GUI in a window on the host system.

### Compilation
The `maturin` tool is used to generate the `pyo3` python bindings from the Rust source code. Use `cargo install maturin` to install the tool, and then generate the bindings with `maturin develop`.
The `displayio_pixels` module provides the `PixelsDisplay` class used in the script.

### Usage
* Create and activate a Python virtual environment, and install the needed dependencies with `pip install -r requirements.txt`
* In a TTY, run an example with `python examples/<example>.py`

### Limitations and Improvements
* TODO - make it work, clean up/optimize, add CRT filter and other eye candy
* TODO - host on PyPi
