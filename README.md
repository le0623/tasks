<div align="center">
  <br>
  <img src="res/icons/hicolor/scalable/apps/dev.le0623.Tasks.svg" width="150" />
  <h1>Tasks</h1>

  <p>A simple task management application for the COSMIC™ desktop</p>

  ![window-light.png](https://raw.githubusercontent.com/le0623/tasks/main/res/screenshots/window-light.png#gh-light-mode-only)
  ![window-dark.png](https://raw.githubusercontent.com/le0623/tasks/main/res/screenshots/window-dark.png#gh-dark-mode-only)

  <a href='https://flathub.org/apps/dev.le0623.Tasks'>
    <img width='200' alt='Get it on Flathub' src='https://flathub.org/api/badge?locale=en'/>
  </a>
</div>

# Installation
```
git clone https://github.com/le0623/tasks.git
cd tasks
sudo just install
```

# Build
```
git clone https://github.com/le0623/tasks.git
cd tasks
cargo build
```

# Flatpak
To build the cargo sources for the Flatpak manifest:

```
python3 ./flatpak/flatpak-cargo-generator.py ./Cargo.lock -o ./flatpak/cargo-sources.json
appstreamcli validate --pedantic --explain res/dev.le0623.Tasks.metainfo.xml
```

## Dependencies
- [libcosmic](https://github.com/pop-os/libcosmic?tab=readme-ov-file#building)

# Copyright and licensing

Copyright 2024 © Eduardo Flores

Tasks is released under the terms of the [GPL-3.0](https://github.com/le0623/tasks/blob/main/LICENSE)
