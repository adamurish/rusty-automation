# rusty-automation
Privacy focused home automation software written in Rust.
## rocket-server
A webserver based on the Rocket rust framework. Currently only has one webhook.
### /torrent
Accepts a POST request with a JSON body in the following format

{

  filename: String
  
  url: String
  
}
