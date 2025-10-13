# USB Thermal Printer Setup for Fulcitt POS AppImage

This document explains how to configure USB thermal printer access when running Fulcitt POS as an AppImage on Linux systems.

## Overview

Fulcitt POS uses ESC/POS thermal printers via USB. AppImages run in a sandboxed environment but can access system USB devices with proper permissions.

## Prerequisites

- USB thermal printer (ESC/POS compatible)
- Linux system with USB support
- Administrative access for initial setup

## USB Device Permission Setup

### 1. Identify Your Printer

Connect your thermal printer and identify its USB vendor and product IDs:

```bash
# List USB devices to find your printer
lsusb

# Look for your printer in the output, e.g.:
# Bus 001 Device 005: ID 0416:5011 Winbond Electronics Corp. Virtual Com Port
#                      ^^^^:^^^^
#                      VID :PID
```

### 2. Create udev Rules

Create a udev rule to allow access to your specific printer:

```bash
# Create udev rule file (replace VID:PID with your printer's values)
sudo nano /etc/udev/rules.d/99-thermal-printer.rules
```

Add this content (replace `0416` and `5011` with your printer's VID and PID):

```
# Thermal printer udev rule
SUBSYSTEM=="usb", ATTRS{idVendor}=="0416", ATTRS{idProduct}=="5011", MODE="0666", GROUP="lp", TAG+="uaccess"
```

### 3. Add User to LP Group

```bash
# Add your user to the lp (line printer) group
sudo usermod -a -G lp $USER

# Verify group membership
groups $USER
```

### 4. Reload udev Rules

```bash
# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Unplug and reconnect your printer
```

### 5. Verify Permissions

```bash
# Check printer device permissions
ls -l /dev/usb/lp*

# Should show something like:
# crw-rw-rw- 1 root lp 180, 0 Dec 12 10:30 /dev/usb/lp0
```

## Alternative: Generic USB Printer Rule

If you want to allow access to all USB printers (less secure but more convenient):

```bash
# Generic rule for all USB printers
echo 'SUBSYSTEM=="usb", ATTR{bInterfaceClass}=="07", MODE="0666", GROUP="lp", TAG+="uaccess"' | sudo tee /etc/udev/rules.d/99-usb-printers.rules
```

## Testing Printer Access

### 1. Test with Raw Commands

```bash
# Test basic connectivity (replace with your device)
echo -e "\x1B\x40Hello World\x0A\x1D\x56\x41\x10" > /dev/usb/lp0
```

### 2. Test in Fulcitt POS

1. Launch the AppImage
2. Go to Settings → Printer Configuration
3. Click "Scan USB Devices"
4. Select your printer from the list
5. Use "Test Print" to verify functionality

## Troubleshooting

### Permission Denied Errors

If you see "Permission denied" when accessing the printer:

1. Verify the user is in the `lp` group: `groups $USER`
2. Log out and log back in (group changes require session restart)
3. Check udev rule syntax: `sudo udevadm test /sys/bus/usb/devices/[device]`
4. Verify device path: `ls -l /dev/usb/lp*`

### Device Not Found

If the printer isn't detected:

1. Check USB connection: `lsusb`
2. Verify kernel modules: `lsmod | grep usblp`
3. Check dmesg for USB events: `dmesg | tail -20`

### CUPS Conflicts

If CUPS has claimed the printer:

1. Stop CUPS temporarily: `sudo systemctl stop cups`
2. Test Fulcitt POS
3. Restart CUPS: `sudo systemctl start cups`

## Security Considerations

The udev rules above grant broad access to USB printers. For production environments:

1. Use specific VID:PID combinations rather than generic rules
2. Consider creating a dedicated user group for POS operations
3. Restrict physical access to USB ports
4. Regularly audit USB device access logs

## AppImage-Specific Notes

- AppImages can access system USB devices through `/dev/usb/` paths
- No additional sandboxing bypass is required for USB printer access
- The escpos and rusb libraries are bundled within the AppImage
- Network printers would require additional firewall configuration

## Support

For printer-specific issues:
1. Check manufacturer documentation for ESC/POS compatibility
2. Test with other ESC/POS software to isolate hardware issues
3. Verify thermal paper specifications match printer requirements

For Fulcitt POS issues:
1. Check application logs for detailed error messages
2. Verify printer configuration in application settings
3. Test with different USB ports or cables