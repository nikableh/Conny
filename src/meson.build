cargo_options = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_options += ['--target-dir', meson.project_build_root() / 'src']

if get_option('profile') == 'release'
  cargo_options += ['--release']
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

cargo_build_env = environment()

cargo_build_env.set('APP_ID', app_id)
cargo_build_env.set('APP_NAME', app_name)
cargo_build_env.set('PKGDATADIR', pkgdatadir)
cargo_build_env.set('PROFILE', profile)
cargo_build_env.set('VERSION', version + version_suffix)
cargo_build_env.set('GETTEXT_PACKAGE', gettext_package)
cargo_build_env.set('LOCALEDIR', localedir)
cargo_build_env.set('RESOURCES_FILE', pkgdatadir / 'resources.gresource')

cargo_build_env.set('CARGO_HOME', meson.project_build_root() / 'cargo-home')
cargo_build_env.set('GSETTINGS_SCHEMA_DIR', meson.project_build_root() / 'data')

custom_target(
  'cargo-build',
  output: meson.project_name(),
  build_always_stale: true,
  build_by_default: true,
  console: true,
  depends: resources,
  env: cargo_build_env,
  install: true,
  install_dir: bindir,
  command: [
    'env',
    cargo,
    'build',
    cargo_options,
    '&&',
    'cp',
    'src' / rust_target / meson.project_name(),
    '@OUTPUT@',
  ],
)
