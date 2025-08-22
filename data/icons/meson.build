if profile != ''
  app_icon = 'app.@0@.svg'.format(profile)
else
  app_icon = 'app.svg'
endif

install_data(
  app_icon,
  install_dir: iconsdir / 'hicolor' / 'scalable' / 'apps',
  rename: '@0@.svg'.format(app_id)
)

install_data(
  'app-symbolic.svg',
  install_dir: iconsdir / 'hicolor' / 'symbolic' / 'apps',
  rename: '@0@-symbolic.svg'.format(app_id)
)
