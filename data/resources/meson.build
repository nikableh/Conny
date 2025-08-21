# Resources
gresource_conf = configuration_data()
gresource_conf.set('top-level-domain', top_level_domain)
gresource_conf.set('domain-name', domain_name)
gresource_conf.set('app-name', app_name)
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
