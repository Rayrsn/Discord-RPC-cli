<div align="center">
  
# `🛠️ Discord RPC cli client`
## **A Discord RPC cli client written purely in Rust **
 <p align="center">
  <img width=70% height=70% src="https://github.com/Rayrsn/Discord-Custom-RPC/raw/main/images/preview_win.png?raw=true">
</p>

[![Rust](https://github.com/Rayrsn/Discord-RPC-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/Rayrsn/Discord-RPC-cli/actions/workflows/rust.yml)

</div>

## Features
* **Typical RPC Features**
  - State
  - Details
  - Displaying large image and it's custom text
  - Displaying small image and it's custom text
  - Buttons
    - Can put 2 buttons

* **More Advanced Features**
  - Setting custom times (Unix format)
    - Can set start time
    - Can set end time
  - Creating a party
    - Can set party size
    - Can set party ID
  - Creating a match
    - Can set match ID
    - Can set join ID
    - Can set spectate ID

* **Other Featurs**
  - Show the time from when the program was run
  - Set a time for the program to exit after reaching it
  - Disable colored output (Enabled by default.)

## Building
### For installing on Arch you can use the AUR ([discord-rpc-cli-git](https://aur.archlinux.org/packages/discord-rpc-cli-git))
### Simply run
#### *Make sure you have rust installed*
```bash
git clone https://github.com/Rayrsn/Discord-RPC-cli && cd Discord-RPC-cli && cargo b --release
```
* **The executable will be located in target/release/Discord-RPC-cli**

## Usage
### The simplest way to use the program is as follows
```bash
./Discord-RPC-cli -c <Application ID goes here> 
```
### You can also run the program with the `-h` argument to print out the help message

## How do i get the client id and the other stuff?
1. Go to [Discord's Developer Portal](https://discord.com/developers/applications/).
2. Make a New Application and name it whatever you want. (this is also the name of your RPC)

![DevPortal](https://github.com/Rayrsn/Discord-Custom-RPC/raw/main/images/newapp.png?raw=true)

3. Then head to the Rich Presence tab.

![RichPresence](https://github.com/Rayrsn/Discord-Custom-RPC/raw/main/images/richpresence.png?raw=true)

4. Scroll down to the "Rich Presence Assets" section

5. Here you can upload any image you want. (it has to be 512x512, there are many online tools that resize images for you)
Also remember you can't rename the image once you hit the "Save Changes" button. (it has to be deleted and reuploaded)

![Uploading](https://github.com/Rayrsn/Discord-Custom-RPC/raw/main/images/Uploading.png?raw=true)

6. Then head back to the "General Information" tab and copy the Application ID. (this is the same as Client ID)

![AppID](https://github.com/Rayrsn/Discord-Custom-RPC/raw/main/images/appid.png?raw=true)

## Acknowledgments and FAQ
* Why can't I click on my own button? 
You just can't. (Discord Limitation)

* Parts of the readme uses the `opensource-template` by EmbarkStudios: [opensource-template](https://github.com/EmbarkStudios/opensource-template)

## Questions
### If yall have any questions or just wanna talk, add me on [Discord](https://rayr.ml/LinkInBio) or use my username Rayr#6709 (this might change so it's better to just use the link)
