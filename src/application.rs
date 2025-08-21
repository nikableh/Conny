use gettextrs::gettext;
use tracing::{debug, info};

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gdk, gio, glib};

use crate::config::{APP_ID, APP_NAME, PKGDATADIR, PROFILE, VERSION};
use crate::window::ConnyWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use std::cell::OnceCell;

    #[derive(Debug, Default)]
    pub struct ConnyApplication {
        pub window: OnceCell<WeakRef<ConnyWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ConnyApplication {
        const NAME: &'static str = "ConnyApplication";
        type Type = super::ConnyApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for ConnyApplication {}

    impl ApplicationImpl for ConnyApplication {
        fn activate(&self) {
            debug!("GtkApplication<ConnyApplication>::activate");
            self.parent_activate();
            let app = self.obj();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = ConnyWindow::new(&app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self) {
            debug!("GtkApplication<ConnyApplication>::startup");
            self.parent_startup();
            let app = self.obj();

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for ConnyApplication {}

    impl AdwApplicationImpl for ConnyApplication {}
}

glib::wrapper! {
    pub struct ConnyApplication(ObjectSubclass<imp::ConnyApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl ConnyApplication {
    fn main_window(&self) -> ConnyWindow {
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
        provider.load_from_resource("/moe/nikableh/Conny/style.css");
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
        let dialog = gtk::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            .license_type(gtk::License::Gpl30Only)
            .website("https://github.com/nikableh/Conny/")
            .version(VERSION)
            .transient_for(&self.main_window())
            .translator_credits(gettext("translator-credits"))
            .modal(true)
            .authors(Self::authors())
            .build();

        dialog.present();
    }

    pub fn run(&self) -> glib::ExitCode {
        info!("{} ({})", APP_NAME, APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self)
    }
}

impl Default for ConnyApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property("resource-base-path", "/moe/nikableh/Conny/")
            .build()
    }
}
