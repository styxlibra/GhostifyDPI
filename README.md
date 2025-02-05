# Ghostify DPI

Graphical user interface for GoodbyeDPI

## Links

Recommended:
- https://github.com/ValdikSS/GoodbyeDPI
- https://github.com/bol-van/zapret
- https://github.com/xvzc/SpoofDPI
- https://github.com/hufrea/byedpi

Recommended UI:
- https://github.com/SpoofDPIApp/SpoofDPI-App
- https://github.com/dovecoteescapee/ByeDPIAndroid

Alternatives (not tested):
- https://github.com/Storik4pro/goodbyeDPI-UI
- https://github.com/axeinstd/DPIshikanoker
- https://github.com/Virenbar/GDPIControl
- https://github.com/r3pr3ss10n/SpoofDPI-Platform

## Plan

- [ ] Features
  - [ ] Support of zapret
  - [ ] Automatic detection of the best mode
  - [ ] IP block diagnostics
  - [ ] DNS block diagnostics
  - [ ] DNS-over-HTTPS
  - [ ] Check if Kyber is enabled
- [ ] User interface
  - [ ] Unplated icon
  - [ ] Dark/light theme switching
  - [ ] Respect OS theme
  - [ ] Show alerts instead of panic
  - [ ] Show alerts on goodbyedpi.exe errors
  - [ ] Internationalization
  - [ ] Multilanguage installer
  - [ ] Custom images for installer
  - [ ] Websites list editor (group domains per website)
  - [ ] Detect taskkill and clear tray
  - [ ] Tray menu item to open settings
  - [ ] Get rid of white background on launch
  - [ ] Introduce error state (red button)
  - [ ] Get rid of blinking on start/stop (switch to PTY processes)
- [ ] Settings menu
  - [ ] Save settings into a file
  - [ ] Dark/light theme switching
  - [ ] Respect OS theme
  - [ ] Launch on system start
  - [ ] Activate on launch
  - [ ] Domains selection
- [ ] Infrastructure
  - [ ] Log everything into a file
  - [ ] Launch on system start
  - [ ] Auto-update
  - [ ] Docker build
  - [ ] Generate icon containing all required sizes
  - [ ] Use in-memory buffer for icon generation instead of a temp file (fix image-rs bug)
  - [ ] Use tauri events instead of channels
  - [ ] Uninstall raw service on auto-start
  - [ ] Kill processes and stop services only when needed (to avoid logs pollution)
  - [ ] Skip blacklist option if the file cannot be created
  - [ ] Reduce memory footprint in release mode (launch only one process)
- [ ] Testing
  - [ ] Directories instead of files (cannot read and write)
  - [ ] Read-only files
  - [ ] Absence of goodbyedpi.exe
  - [ ] Absence of WinDivert
  - [ ] External kill of goodbyedpi.exe
  - [ ] External kill of the app
- [ ] Documentation
  - [ ] About link from app to GitHub
  - [ ] References for used Apache works
  - [ ] Screenshots
  - [ ] Alternatives
  - [ ] How to launch
  - [ ] How to build
  - [ ] How to develop
  - [ ] How it works
  - [ ] Plans
  - [ ] VirusTotal
  - [ ] Translation of docs
