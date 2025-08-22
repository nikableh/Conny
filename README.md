# GTK4/Adwaita/Rust/Flatpak/Nix Template

A template for developing stunning desktop applications with...

- **GTK4 and Adwaita** beauty
- **Flatpak and Nix** reproducibility
- **Rust** safety and developer experience

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
flatpak-builder --user --force-clean flatpak_app build-aux/moe.nikableh.Conny.Devel.json
flatpak-builder --run flatpak_app build-aux/moe.nikableh.Conny.Devel.json conny
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
