use iced::{Align, Column, Container, Element, Font, Length, Sandbox,
    Settings, Text, };

use iced_aw::{Tabs, TabLabel};

mod login;
use login::{LoginMessage, LoginTab};

mod ferris;
use ferris::{FerrisMessage, FerrisTab};

mod counter;
use counter::{CounterMessage, CounterTab};

mod settings;
use settings::{TabBarPosition, SettingsMessage, SettingsTab};

mod theme;

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

const ICON_FONT: Font = iced::Font::External{
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

enum Icon {
    User,
    Heart,
    Calc,
    CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::User => '\u{E800}',
            Icon::Heart => '\u{E801}',
            Icon::Calc => '\u{F1EC}',
            Icon::CogAlt => '\u{E802}',
        }
    }
}

fn main() -> iced::Result {
    TabBarExample::run(Settings::default())
}

struct TabBarExample {
    active_tab: usize,
    login_tab: LoginTab,
    ferris_tab: FerrisTab,
    counter_tab: CounterTab,
    settings_tab: SettingsTab,
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(usize),
    LoginMessage(LoginMessage),
    FerrisMessage(FerrisMessage),
    CounterMessage(CounterMessage),
    SettingsMessage(SettingsMessage),
}

impl Sandbox for TabBarExample {
    type Message = Message;

    fn new() -> Self {
        TabBarExample {
            active_tab: 0,
            login_tab: LoginTab::new(),
            ferris_tab: FerrisTab::new(),
            counter_tab: CounterTab::new(),
            settings_tab: SettingsTab::new(),
        }
    }

    fn title(&self) -> String {
        String::from("TabBar Example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TabSelected(selected) => {
                self.active_tab = selected
            },
            Message::LoginMessage(message) => {
                self.login_tab.update(message)
            },
            Message::FerrisMessage(message) => {
                self.ferris_tab.update(message)
            },
            Message::CounterMessage(message) => {
                self.counter_tab.update(message)
            },
            Message::SettingsMessage(message) => {
                self.settings_tab.update(message)
            },
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let position = self.settings_tab.settings().tab_bar_position
            .unwrap_or_default();
        let theme = self.settings_tab.settings().tab_bar_theme
            .unwrap_or_default();

        Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.login_tab.tab_label(), self.login_tab.view())
            .push(self.ferris_tab.tab_label(), self.ferris_tab.view())
            .push(self.counter_tab.tab_label(), self.counter_tab.view())
            .push(self.settings_tab.tab_label(), self.settings_tab.view())
            .tab_bar_style(theme)
            .icon_font(ICON_FONT)
            .tab_bar_position(match position {
                TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
            })
            .into()
    }
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&mut self) -> Element<'_, Self::Message> {
        let column = Column::new()
        .spacing(20)
        .push(
            Text::new(self.title())
                .size(HEADER_SIZE)
        )
        .push(self.content());
    
        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&mut self) -> Element<'_, Self::Message>;
}