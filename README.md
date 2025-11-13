# Fulcitt POS

## Printer Setup

### Linux USB Printer Setup

To use USB printers on Linux, you need to set up udev rules to grant permission to access the USB device:

1. Create a udev rules file:
   ```bash
   sudo nano /etc/udev/rules.d/99-escpos-printer.rules
   ```

2. Add a rule for your printer (replace vendor/product IDs with your printer's):
   ```
   # ESC/POS Printer
   SUBSYSTEM=="usb", ATTR{idVendor}=="0fe6", ATTR{idProduct}=="811e", MODE="0666"
   ```

3. Reload udev rules:
   ```bash
   sudo udevadm control --reload-rules
   sudo udevadm trigger
   ```

4. Unplug and replug the printer

**Finding your printer's IDs**: Run `lsusb` to see all USB devices and find your printer's vendor and product IDs.

### Windows USB Printer Setup

Windows requires driver installation for direct USB access:

1. Download Zadig from https://zadig.akeo.ie/
2. Run as Administrator
3. Go to **Options → List All Devices**
4. Select your ESC/POS printer
5. Select **WinUSB** as the target driver
6. Click **Replace Driver**

**Note**: This is a one-time setup per machine. The printer will only work with this application after installing WinUSB.

### Network Printer Support (Future)

Network printer support can be added later for cross-platform compatibility without driver issues. ESC/POS printers typically default to `192.168.1.x` IP addresses, which may require network configuration to match your local network subnet. Network printing avoids USB permission and driver complications on both Linux and Windows.

## TODO

Features

- [X] Improve invoice layout
- [X] Allow printing selection
- [X] Clear sales history
- [X] Italian localization
- [X] Show all invoices and reprint
- [X] Export sales

- [W] Improve error messages
- [ ] Gestire disconnessione della stampante
- [ ] Integrate biome
- [ ] Linux (AppImage) automated build
- [ ] Windows automated build
- [ ] Product deposit (pay/refund mechanic)
- [ ] Voiding and refunding support

Known bugs

- [X] On startup the printer is not configured, even though trying to print works
- [X] Date times are not local in: tickets, reprint tickets and reports page
- [ ] Build requires `NO_STRIP=true` on Linux see [tauri/issues/8929](https://github.com/tauri-apps/tauri/issues/8929)