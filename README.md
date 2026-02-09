<img width="640" height="114" alt="image" src="https://github.com/user-attachments/assets/fb633986-4808-481e-8065-a561462bc0e5" />

### A tool for bypassing time-based features of Family Link parental controls

## How to use
> [!NOTE]
> YOU NEED A COMPUTER AND DEVELOPER OPTIONS ENABLED FOR THIS TO WORK. DEVELOPER OPTIONS CAN BE ENABLED ON THE PARENT APP.
1. Download the appropriate release for your CPU architecture from the [releases](https://github.com/rifting/chronolink/releases/tag/v2.0.0) (The one you need is probably chronolink-aarch64).
2. Ensure you have adb installed on your machine
3. Connect your phone via USB
4. `adb push chronolink /data/local/tmp`
5. `adb shell chmod +x /data/local/tmp/chronolink`
6. List time zones with `adb shell /data/local/tmp/chronolink list`
7. Once you've chosen a time zone, you can set device time to it as so:

   `adb shell /data/local/tmp/chronolink America/New_York`
   
   Replace America/New_York with the time zone you choose from the list. 
9. Enjoy bypassing downtime!

## FAQ
### Will this work with developer options off?
No. 

### Why do you have a file with every single time zone on Android? Is that not bloat?
Planning on removing it later lol

## How do I compile?
Compiled on Ubuntu 24.04.1 LTS. This will assume you are using a debian-based distro, but this should work on most major distributions. The binder_rust crate relies on on the nix crate which is not supported on Windows right now.

First things first, make sure you have Rust and the android NDK ready. You can follow [this guide](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html) for a walkthrough of how to set up both of those.

`sudo apt install -y build-essential`

`git clone https://github.com/rifting/chronolink.git`

`cd chronolink`

`cargo build --release --target=<YOUR ANDROID TARGET>`

Find your chronolink binary in `target/<YOUR ANDROID TARGET>/release/chronolink`
