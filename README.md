<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/cosmic-tasks/main/res/icons/hicolor/256x256/apps/com.system76.CosmicTasks.svg" width="150" />
  <h1>COSMIC Tasks</h1>
  
  <h3>A simple task management application for the COSMIC desktop.</h3>

  ![window-light.png](https://raw.githubusercontent.com/edfloreshz/cosmic-tasks/main/res/screenshots/window-light.png#gh-light-mode-only)
  ![window-dark.png](https://raw.githubusercontent.com/edfloreshz/cosmic-tasks/main/res/screenshots/window-dark.png#gh-dark-mode-only)
</div>

# Installation
```
git clone https://github.com/edfloreshz/cosmic-tasks.git
cd cosmic-tasks
sudo just install
```

# Build
```
git clone https://github.com/edfloreshz/cosmic-tasks.git
cd cosmic-tasks
cargo build
```

## Dependencies
- [libcosmic](https://github.com/pop-os/libcosmic?tab=readme-ov-file#building)
- sqlite3

Ubuntu
```
sudo apt install libsqlite3-dev
```

Fedora
```
sudo apt install sqlite-devel
```

# Copyright and licensing

Copyright 2024 © Eduardo Flores

Done is released under the terms of the [GPL-3.0](https://github.com/edfloreshz/cosmic-tasks/blob/main/LICENSE)
