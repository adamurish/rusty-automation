# rusty-automation
Privacy focused home automation software written in Rust.

## basic_node
Some basic code for running a Rocket server on a Raspberry Pi and setting GPIO outputs using rppal.
### /on and /off
Both urls accept a basic GET request and set a GPIO pin (currently 26) to either high or low
