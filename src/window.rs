use std::process;

use cosmic::app::Core;

use cosmic::applet::menu_button;
use cosmic::iced::wayland::popup::{destroy_popup, get_popup};
use cosmic::iced::window::Id;
use cosmic::iced::{Command, Limits};

use cosmic::iced_futures::Subscription;
use cosmic::iced_runtime::core::window;
use cosmic::iced_style::application;
use cosmic::widget::{button, text, column, row};

use cosmic::{Element, Theme};

use crate::config::{Config, CONFIG_VERSION};
use cosmic::cosmic_config;

pub const APP_ID: &str = "com.prple.CosmicLogoMenu";

// TODO: Figure out how icons are handled and this probably needs its own function 
const ICON: &str = APP_ID;
pub struct Window {
    core: Core,
    config: Config,
    config_handler: Option<cosmic_config::Config>,
    popup: Option<Id>,
}

#[derive(Clone, Debug)]
enum PowerAction {
    Suspend,
    Restart,
    Shutdown,
}

#[derive(Clone, Debug)]
pub enum Message {
    Config(Config),
    TogglePopup,
    PopupClosed(Id),
    AboutMySystem,
    SoftwareCenter,
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub config_handler: Option<cosmic_config::Config>,
    pub config: Config,
}

impl cosmic::Application for Window {
    type Executor = cosmic::executor::Default;
    type Flags = Flags;
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(
        core: Core,
        flags: Self::Flags,
    ) -> (Self, cosmic::Command<cosmic::app::Message<Self::Message>>) {
        let window = Window {
            core,
            config: flags.config,
            config_handler: flags.config_handler,
            popup: None,
        };
        (window, Command::none())
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Self::Message) -> Command<cosmic::app::Message<Self::Message>> {
        macro_rules! config_set {
            ($name: ident, $value: expr) => {
                match &self.config_handler {
                    Some(config_handler) => {
                        match paste::paste! { self.config.[<set_ $name>](config_handler, $value) } {
                            Ok(_) => {}
                            Err(err) => {
                                eprintln!("failed to save config {:?}: {}", stringify!($name), err);
                            }
                        }
                    }
                    None => {
                        self.config.$name = $value;
                        eprintln!(
                            "failed to save config {:?}: no config handler",
                            stringify!($name),
                        );
                    }
                }
            };
        }

        match message {
            Message::Config(config) => {
                if config != self.config {
                    self.config = config
                }
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings =
                        self.core
                            .applet
                            .get_popup_settings(Id::MAIN, new_id, None, None, None);
                    popup_settings.positioner.size_limits = Limits::NONE
                        .min_width(100.0)
                        .max_height(100.0)
                        .min_height(400.0)
                        .max_width(500.0);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::AboutMySystem => {
                let _ = process::Command::new("cosmic-settings")
                .arg("about")
                .spawn();
            }
            Message::SoftwareCenter => {
                let _ = process::Command::new("cosmic-store").spawn();
            }
        }
        Command::none()
    }

    // I think is what shows up in the panel?
    fn view(&self) -> Element<Self::Message> {
        self.core
            .applet
            .icon_button(ICON)
            .on_press(Message::TogglePopup)
            .into()
    }

    // This is where the content goes when clicked
    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        let content = menu_button(text("About my system").size(14))
        .on_press(Message::AboutMySystem);
    
        self.core.applet.popup_container(content).into()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        struct ConfigSubscription;

        let config = cosmic_config::config_subscription(
            std::any::TypeId::of::<ConfigSubscription>(),
            Self::APP_ID.into(),
            CONFIG_VERSION,
        )
        .map(|update| {
            if !update.errors.is_empty() {
                eprintln!(
                    "errors loading config {:?}: {:?}",
                    update.keys, update.errors
                );
            }
            Message::Config(update.config)
        });

        Subscription::batch(vec![config])
    }

    fn style(&self) -> Option<<Theme as application::StyleSheet>::Style> {
        Some(cosmic::applet::style())
    }
}