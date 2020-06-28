# rusty-automation
Privacy focused home automation software written in Rust.
## rocket-server
A webserver based on the Rocket rust framework. Currently only has one webhook.
### /torrent
Accepts a POST request with a JSON body that includes the following keys:
#### filename
The local filename to save to downloaded file to
#### url
The url to download the file from

## basic_node
Some basic code for running a Rocket server on a Raspberry Pi and setting GPIO outputs using rppal.
### /on and /off
Both urls accept a basic GET request and set a GPIO pin (currently 26) to either high or low
