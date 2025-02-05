# chronolink
### A tool for bypassing time-based features of Family Link parental controls

## How to use
> [!NOTE]
> YOU NEED A COMPUTER AND DEVELOPER OPTIONS ENABLED FOR THIS TO WORK. DEVELOPER OPTIONS CAN BE ENABLED ON THE PARENT APP.
1. Download the appropriate release for your CPU architecture from the [releases](https://github.com/rifting/chronolink/releases/tag/v0.1.0) (it likely IS aarch64).
2. Ensure you have adb installed on your machine
3. Connect your phone via USB
4. `adb push chronolink /data/local/tmp`
5. `adb shell chmod +x /data/local/tmp/chronolink`
6. `adb shell /data/local/tmp/chronolink`
7. Scroll the options with the J & K keys, and hit enter on your chosen time.
8. Enjoy bypassing downtime!

## FAQ
### Will this work with developer options off / through a terminal emulator?
No. 
### Why do you have a file with every single time zone on Android? Is that not bloat?
It DEFINITELY is. I just didn't have the time to sort through all the unique time zones (since some have 30 and 15 minute offsets). If somebody would like to PR it to remove the non duplicate timezones I would gladly merge. This is just a real quick lazy implementation.

## How do I compile?
Compiled on Ubuntu 24.04.1 LTS. This will assume you are using a debian-based distro, but this should work on most major distributions. The binder_rust crate relies on on the nix crate which is not supported on Windows right now.

First things first, make sure you have Rust and the android NDK ready. You can follow [this guide](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html) for a walkthrough of how to set up both of those.

`sudo apt install -y build-essential`

`git clone https://github.com/rifting/chronolink.git`

`cd chronolink`

`cargo build --release --target=<YOUR ANDROID TARGET>`

Find your chronolink binary in `target/<YOUR ANDROID TARGET>/release/chronolink`
