// SPDX-License-Identifier: GPL-3.0-only

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};

use crate::application::^APP_NAME^Application;
use crate::config::{APP_ID, PROFILE};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/^TOP_LEVEL_DOMAIN^/^DOMAIN_NAME^/^APP_NAME^/ui/window.ui")]
    pub struct ^APP_NAME^Window {
        pub settings: gio::Settings,
    }

    impl Default for ^APP_NAME^Window {
        fn default() -> Self {
            Self {
                settings: gio::Settings::new(APP_ID),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ^APP_NAME^Window {
        const NAME: &'static str = "^APP_NAME^Window";
        type Type = super::^APP_NAME^Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ^APP_NAME^Window {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();
        }
    }

    impl WidgetImpl for ^APP_NAME^Window {}

    impl WindowImpl for ^APP_NAME^Window {
        // Save window state on delete event
        fn close_request(&self) -> glib::Propagation {
            if let Err(err) = self.obj().save_window_size() {
                tracing::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for ^APP_NAME^Window {}

    impl AdwApplicationWindowImpl for ^APP_NAME^Window {}
}

glib::wrapper! {
    pub struct ^APP_NAME^Window(ObjectSubclass<imp::^APP_NAME^Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup,
                    gtk::Root, gtk::Native, gtk::ShortcutManager,
                    gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ^APP_NAME^Window {
    pub fn new(app: &^APP_NAME^Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();

        imp.settings.set_int("window-width", width)?;
        imp.settings.set_int("window-height", height)?;

        imp.settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let imp = self.imp();

        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");
        let is_maximized = imp.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}
