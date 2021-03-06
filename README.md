# druid-app-boilerplate

Druid boilerplate for Pinephone app development.
## Build

Install `cross`:
```bash
cargo install cross
```
Install `docker`:
https://docs.docker.com/engine/install/

Build the project using cross:
```bash
cross build --target=aarch64-unknown-linux-gnu --release
```

## Install
Send the project to the phone:

```bash
scp -r path/to/project {phone-username}@{phone-ip}:~/druid-app-boilerplate
```

Copy the release binary to `/usr/local/bin`:
```bash
sudo cp target/aarch64-unknown-linux-gnu/release/boilerplate /usr/local/bin
```

Install the desktop app with logo:

```bash
sudo cp extra/logo/boilerplate.svg /usr/share/pixmaps/Boilerplate.svg
sudo desktop-file-install extra/linux/Boilerplate.desktop
sudo update-desktop-database
```

## TODO:
Replace these build and install instructions with an easier flatpak/snap setup.
