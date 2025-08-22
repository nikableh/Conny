// SPDX-License-Identifier: GPL-3.0-only

use gettextrs::gettext;
use tracing::{debug, info};

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gdk, gio, glib};

use crate::config::{APP_ID, APP_NAME, PKGDATADIR, VERSION};
use crate::window::^APP_NAME^Window;

mod imp {
    use super::*;
    use glib::WeakRef;
    use std::cell::OnceCell;

    #[derive(Debug, Default)]
    pub struct ^APP_NAME^Application {
        pub window: OnceCell<WeakRef<^APP_NAME^Window>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ^APP_NAME^Application {
        const NAME: &'static str = "^APP_NAME^Application";
        type Type = super::^APP_NAME^Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for ^APP_NAME^Application {}

    impl ApplicationImpl for ^APP_NAME^Application {
        fn activate(&self) {
            debug!("Application::activate");
            self.parent_activate();
            let app = self.obj();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = ^APP_NAME^Window::new(&app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self) {
            debug!("Application::startup");
            self.parent_startup();
            let app = self.obj();

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for ^APP_NAME^Application {}

    impl AdwApplicationImpl for ^APP_NAME^Application {}
}

glib::wrapper! {
    pub struct ^APP_NAME^Application(ObjectSubclass<imp::^APP_NAME^Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl ^APP_NAME^Application {
    fn main_window(&self) -> ^APP_NAME^Window {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| {
                // This is needed to trigger the delete event and saving the window state
                app.main_window().close();
                app.quit();
            })
            .build();

        // About
        let action_about = gio::ActionEntry::builder("about")
            .activate(|app: &Self, _, _| {
                app.show_about_dialog();
            })
            .build();
        self.add_action_entries([action_quit, action_about]);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
        self.set_accels_for_action("window.close", &["<Control>w"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/^TOP_LEVEL_DOMAIN^/^DOMAIN_NAME^/^APP_NAME^/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn authors() -> Vec<&'static str> {
        // Authors are defined in Cargo.toml
        env!("CARGO_PKG_AUTHORS").split(":").collect()
    }

    fn show_about_dialog(&self) {
        let dialog = adw::AboutDialog::builder()
            .application_name(APP_NAME)
            .application_icon(APP_ID)
            .version(VERSION)
            .license_type(gtk::License::Gpl30Only)
            .developers(Self::authors())
            .website("^URL^")
            .issue_url("^URL_ISSUES^")
            .translator_credits(gettext("translator-credits"))
            .copyright("© 2025 ^AUTHOR_NAME^")
            .build();

        dialog.present(self.active_window().as_ref());
    }

    pub fn run(&self) -> glib::ExitCode {
        info!("{} ({})", APP_NAME, APP_ID);
        info!("Version: {}", VERSION,);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self)
    }
}

impl Default for ^APP_NAME^Application {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property("resource-base-path", "/^TOP_LEVEL_DOMAIN^/^DOMAIN_NAME^/^APP_NAME^/")
            .build()
    }
}
