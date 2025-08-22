subdir('icons')
subdir('resources')

# Desktop file
desktop_conf = configuration_data()
desktop_conf.set('app-name', app_name)
desktop_conf.set('short-description', short_description)
desktop_conf.set('executable', meson.project_name())
desktop_conf.set('icon', app_id)
desktop_file = i18n.merge_file(
  type: 'desktop',
  input: configure_file(
    input: 'app.desktop.in.in',
    output: '@0@.desktop.in'.format(base_id_dotted),
    configuration: desktop_conf
  ),
  output: '@0@.desktop'.format(app_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'applications'
)
# Validate Desktop file
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file.full_path()
    ],
    depends: desktop_file,
  )
endif

# Appdata
appdata_conf = configuration_data()
appdata_conf.set('app-id', app_id)
appdata_conf.set('app-name', app_name)
appdata_conf.set('gettext-package', gettext_package)
appdata_conf.set('license', meson.project_license())
appdata_conf.set('short-description', short_description)
appdata_conf.set('description', description)
appdata_file = i18n.merge_file(
  input: configure_file(
    input: 'app.metainfo.xml.in.in',
    output: '@0@.metainfo.xml.in'.format(base_id_dotted),
    configuration: appdata_conf
  ),
  output: '@0@.metainfo.xml'.format(app_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'metainfo'
)

# Validate Appdata
if appstreamcli.found()
  test(
    'validate-appdata', appstreamcli,
    args: [
      'validate', '--no-net', '--pedantic', '--explain', appdata_file.full_path()
    ],
    depends: appdata_file,
  )
endif

# GSchema
gschema_conf = configuration_data()
gschema_conf.set('base-id-slashed', base_id_slashed)
gschema_conf.set('app-id', app_id)
gschema_conf.set('gettext-package', gettext_package)
configure_file(
  input: 'app.gschema.xml.in'.format(base_id_dotted),
  output: '@0@.gschema.xml'.format(app_id),
  configuration: gschema_conf,
  install: true,
  install_dir: datadir / 'glib-2.0' / 'schemas'
)

# Validate GSchema
test(
  'validate-gschema', glib_compile_schemas,
  args: [
    '--strict', '--dry-run', meson.current_build_dir()
  ],
)

# D-Bus service file
service_conf = configuration_data()
service_conf.set('app-id', app_id)
service_conf.set('bindir', bindir)
service_conf.set('executable', meson.project_name())
configure_file(
    input: 'app.service.in',
    output: '@0@.service'.format(app_id),
    configuration: service_conf,
    install: true,
    install_dir: datadir / 'dbus-1' / 'services'
)
