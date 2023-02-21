use std::time::Duration;

use chrono::{Local};
use gtk::prelude::*;
use relm4::prelude::*;

struct App {
    time: String,
}

#[derive(Debug)]
enum Msg {
    Shutdown,
    Reboot,
    Logout,
    Hibernate,
    Sleep,
    UpdateTime
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = String;
    type Input = Msg;
    type Output = ();

    view! {
        adw::Window {
            set_title: Some("System Tool"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
                set_valign: gtk::Align::Center,
                set_halign: gtk::Align::Center,

                gtk::Label {
                    #[watch]
                    set_label: model.time.as_str(),
                    set_margin_bottom: 10,
                    set_class_active: ("title-header", true),
                },

                gtk::Button {
                    set_label: "Shutdown",
                    connect_clicked => Msg::Shutdown,
                },

                gtk::Button {
                    set_label: "Reboot",
                    connect_clicked => Msg::Reboot,
                },

                gtk::Button {
                    set_label: "Logout",
                    connect_clicked => Msg::Logout,
                },

                gtk::Button {
                    set_label: "Hibernate",
                    connect_clicked => Msg::Hibernate,
                },

                gtk::Button {
                    set_label: "Sleep",
                    connect_clicked => Msg::Sleep,
                },
            }
        }
    }

    // Initialize the component.
    fn init(
        time: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App { time };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();
        
        // update the time once per second
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(1));
                sender.input(Msg::UpdateTime);
            }
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::Shutdown => {
                match system_shutdown::shutdown() {
                    Ok(_) => println!("Shutting down, bye!"),
                    Err(error) => eprintln!("Failed to shut down: {}", error),
                }
            }
            Msg::Reboot => {
                match system_shutdown::reboot() {
                    Ok(_) => println!("Rebooting, bye!"),
                    Err(error) => eprintln!("Failed to reboot: {}", error),
                }          
            }
            Msg::Logout => {
                match system_shutdown::logout() {
                    Ok(_) => println!("Logging out, bye!"),
                    Err(error) => eprintln!("Failed to logout: {}", error),
                }
            }
            Msg::Hibernate => {
                match system_shutdown::hibernate() {
                    Ok(_) => println!("Hibernating, bye!"),
                    Err(error) => eprintln!("Failed to hibernate: {}", error),
                }
            }
            Msg::Sleep => {
                match system_shutdown::sleep() {
                    Ok(_) => println!("Sleeping, bye!"),
                    Err(error) => eprintln!("Failed to sleep: {}", error),
                }
            }
            Msg::UpdateTime => {
                self.time = get_time();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("com.bnyro.system");
    relm4::set_global_css(include_str!("style.css"));
    app.run::<App>(String::from(get_time()));
}

fn get_time() -> String {
    let now = Local::now();
    let res = now.format("%d.%m.%Y %H:%M:%S");
    return res.to_string()
}