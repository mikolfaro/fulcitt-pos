# Fulcitt POS

## TODO

Features

[X] - Improve invoice layout
[X] - Allow printing selection
[X] - Clear sales history
[X] - Italian localization
[X] - Show all invoices and reprint
[X] - Export sales
[W] - Improve error messages
[X] - Make error messages disappear after few seconds
[ ] - Gestire disconnessione della stampante
[X] - Migrare a UsbDriver

Known bugs

[X] - On startup the printer is not configured, even though trying to print works
[X] - Date times are not local in: tickets, reprint tickets and reports page
[ ] - Build requires `NO_STRIP=true` on Linux see [tauri/issues/8929](https://github.com/tauri-apps/tauri/issues/8929)