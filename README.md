# Conny

Use OpenVPN from GUI.

A GUI application for managing OpenVPN profiles and active connections.

## Building the project

To compile and run the application locally (for development purposes) use
project's Nix `devShell`. This way you would get all the necessary dependencies
and avoid having to recompile the whole application from scratch.

```shell
nix develop
./dev.sh
```

You can also run `./dev.sh` outside of `nix develop` shell, but you then have to
manually insure all the dependencies are installed.

## Goals

### Technical

- [ ] Successful VPN initialization
- [ ] Save profiles
- [ ] Work in background
- [ ] Working 2FA
- [ ] Ability to add custom console options
- [ ] Ability to view logs
- [ ] Color messages of different log levels in logs

### Best practices

- [ ] Full compliance with GNOME Circle guidelines
- [ ] Full compliance with GNOME HIG
- [ ] Infrastructure for adding translations

### Meta

- [ ] Package with Nix
- [ ] Package with Flatpak
- [ ] Apply for GNOME Circle
- [ ] Publish to Flathub

## License

This project is under the [GPL-3.0-only] license.

[GPL-3.0-only]: https://opensource.org/license/gpl-3-0

## Code Of Conduct

This project follows the [GNOME Code of Conduct](https://conduct.gnome.org/).
