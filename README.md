![LMTK_l-sync](https://user-images.githubusercontent.com/85266594/161609849-03ae1c3b-5cdc-4e45-97a6-adec11676a62.png)
---
# L-Sync or, Lura-Sync
A content creator centered synchronisation program, not to be confused with Nvidia's G-Sync

## What is this used for?
This program is made to help with Getting your audio devices and video devices set up and synced by producing a noise and white flash for a quarter of a second

## Build instructions (Linux)
Open up your terminal and paste (or type)
```bash
git clone https://github.com/Luramoth/l-sync
cd l-sync
```
Then here you can choose to use Cargo or my own personal build script

### prerequisates:
you should have [rustup](https://www.rust-lang.org/tools/install) installed in order to have the whole thing work.

### using the custom build script:
```bash
bash build.sh --linux --release #this builds the program for Linux in release mode
```
use `--linux` to compile for linux,
use `--windows` to compile for windows,
use `--all` to compile for both,
append `--release` to compile any of those in release mode

![agplv3-155x51](https://user-images.githubusercontent.com/85266594/161496819-22de1d6d-6a8e-4a65-865a-86af5901c834.png)
