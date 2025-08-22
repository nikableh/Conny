# GTK4/Adwaita/Rust/Flatpak/Nix Template

A template for developing stunning desktop applications with...

- **GTK4 and Adwaita** beauty
- **Flatpak and Nix** reproducibility
- **Rust** safety and developer experience

## Using this template

_For now_ this template has a bunch of strings that must be set manually in
different places (both file names and strings). To make this job easier, I
replaced them with these placeholders:

- `^AUTHOR_NAME^` (e.g. `Nika`)
- `^AUTHOR_EMAIL^` (e.g. `nika@nikableh.moe`)
- `^TOP_LEVEL_DOMAIN^` (e.g. `moe` or `io`)
- `^DOMAIN_NAME^` (e.g. `nikableh` or `github.username`)
- `^APP_NAME^` (e.g. `AwesomeApplication`)
- `^URL^` (e.g. `https://github.com/nikableh/AwesomeApplication`)
- `^URL_ISSUES^` (e.g. `https://github.com/nikableh/AwesomeApplication/issues`)
- `^EXECUTABLE^` (e.g. `awesome-application`)
- `^SHORT_DESCRIPTION^` (e.g. `Short description of the application`)
- `^LONG_DESCRIPTION^` (e.g. `Long description of the application`)

## Building from source

### Nix (preferred)

To compile and run the application locally (for development purposes) use Nix
and its development shell. This way you get all the necessary dependencies
and avoid having to recompile the whole application from scratch.

```shell
nix develop
./dev.sh
```

You can also run `./dev.sh` without `nix develop` shell, but you then have to
manually ensure the correctness of all dependencies.

Alternatively, you can run application with a single command. But this would do
a full clean rebuild for any change in the source code.

```shell
nix run
```

### Flatpak

Flatpak would rebuild the project from scratch each time, which is not very
pleasant in situations when you need iterate rapidly.

> [!IMPORTANT]  
> On NixOS you have to enable Flatpak service in your `configuration.nix` file,
> for this to work.
>
> ```nix
> services.flatpak.enable = true;
> ```

```shell
flatpak install --user org.gnome.Sdk//48 org.gnome.Platform//48 org.freedesktop.Sdk.Extension.rust-stable//24.08 org.freedesktop.Sdk.Extension.llvm18//24.08
flatpak-builder --user --force-clean flatpak_app build-aux/^TOP_LEVEL_DOMAIN^.^DOMAIN_NAME^.^APP_NAME^.Devel.json
flatpak-builder --run flatpak_app build-aux/^TOP_LEVEL_DOMAIN^.^DOMAIN_NAME^.^APP_NAME^.Devel.json ^EXECUTABLE^
```

## Best practices

This is a list of resources you better familiarize yourself with, if you want
your application to be perfect.

- Ensure compliance with [GNOME Circle guidelines]
- Ensure compliance with [GNOME Human Interface Guidelines]

[GNOME Circle guidelines]: https://gitlab.gnome.org/Teams/Releng/AppOrganization/-/blob/main/AppCriteria.md
[GNOME Human Interface Guidelines]: https://developer.gnome.org/hig/

## License

This project is under the [GPL-3.0-only] license.

[GPL-3.0-only]: https://opensource.org/license/gpl-3-0

## Code Of Conduct

This project follows the [GNOME Code of Conduct](https://conduct.gnome.org/).
