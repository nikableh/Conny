// SPDX-License-Identifier: GPL-3.0-only

// These variables should be set to their real values in meson.build file.
// 
// Don't forget to edit the .cargo/config.toml file if you want to add a new
// variable here. This would remove the "not defined at compile time" error by
// giving your variable a default value.

pub const APP_ID: &str = env!("APP_ID");
pub const APP_NAME: &str = env!("APP_NAME");
pub const GETTEXT_PACKAGE: &str = env!("GETTEXT_PACKAGE");
pub const LOCALEDIR: &str = env!("LOCALEDIR");
pub const PKGDATADIR: &str = env!("PKGDATADIR");
pub const PROFILE: &str = env!("PROFILE");
pub const RESOURCES_FILE: &str = env!("RESOURCES_FILE");
pub const VERSION: &str = env!("VERSION");
