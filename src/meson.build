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

cargo_env = ['CARGO_HOME=' + meson.project_build_root() / 'cargo-home']

global_conf_env = environment()
global_conf_env.set('APP_ID', app_id)
global_conf_env.set('APP_NAME', app_name)
global_conf_env.set('PKGDATADIR', pkgdatadir)
global_conf_env.set('PROFILE', profile)
global_conf_env.set('VERSION', version + version_suffix)
global_conf_env.set('GETTEXT_PACKAGE', gettext_package)
global_conf_env.set('LOCALEDIR', localedir)

custom_target(
  'cargo-build',
  output: meson.project_name(),
  build_always_stale: true,
  build_by_default: true,
  console: true,
  depends: resources,
  env: global_conf_env,
  install: true,
  install_dir: bindir,
  command: [
    'env',
    cargo_env,
    cargo,
    'build',
    cargo_options,
    '&&',
    'cp',
    'src' / rust_target / meson.project_name(),
    '@OUTPUT@',
  ],
)
