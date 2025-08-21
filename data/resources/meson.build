# Resources
gresource_conf = configuration_data()
gresource_conf.set('base-id-slashed', base_id_slashed)
resources = gnome.compile_resources(
  'resources',
  configure_file(
    input: 'resources.gresource.xml.in',
    output: '@BASENAME@',
    configuration: gresource_conf,
  ),
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)
