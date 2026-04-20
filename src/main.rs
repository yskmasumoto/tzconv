use chrono::offset::MappedLocalTime;
use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::{TZ_VARIANTS, Tz};
use iced::widget::{button, center, column, combo_box, container, row, text, text_input};
use iced::{clipboard, Element, Length, Task, Theme};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .window_size((460.0, 180.0))
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    UnixInputChanged(String),
    DatetimeInputChanged(String),
    TimezoneSelected(Tz),
    // combo_box::on_input requires Fn(String) -> Message; value unused
    #[allow(dead_code)]
    TimezoneInputChanged(String),
    ConvertToDatetime,
    ConvertToUnix,
    Now,
    CopyUnix,
    CopyDatetime,
}

struct App {
    unix_input: String,
    datetime_input: String,
    timezone: Tz,
    tz_state: combo_box::State<Tz>,
    status: String,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                unix_input: String::new(),
                datetime_input: String::new(),
                timezone: chrono_tz::Asia::Tokyo,
                tz_state: combo_box::State::new(TZ_VARIANTS.to_vec()),
                status: String::new(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("tzconv")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UnixInputChanged(v) => {
                self.unix_input = v;
                self.status.clear();
            }
            Message::DatetimeInputChanged(v) => {
                self.datetime_input = v;
                self.status.clear();
            }
            Message::TimezoneSelected(tz) => {
                self.timezone = tz;
                self.status.clear();
            }
            Message::TimezoneInputChanged(_) => {
                self.status.clear();
            }
            Message::ConvertToDatetime => {
                match self.unix_input.trim().parse::<i64>() {
                    Ok(ts) => match DateTime::from_timestamp(ts, 0) {
                        Some(utc) => {
                            let local = utc.with_timezone(&self.timezone);
                            self.datetime_input =
                                local.format("%Y-%m-%d %H:%M:%S").to_string();
                            self.status = "Converted: Unix -> Datetime".into();
                        }
                        None => self.status = "Error: timestamp out of range".into(),
                    },
                    Err(_) => self.status = "Error: invalid Unix timestamp".into(),
                }
            }
            Message::ConvertToUnix => {
                match NaiveDateTime::parse_from_str(
                    self.datetime_input.trim(),
                    "%Y-%m-%d %H:%M:%S",
                ) {
                    Ok(naive) => match naive.and_local_timezone(self.timezone) {
                        MappedLocalTime::Single(dt) => {
                            self.unix_input = dt.timestamp().to_string();
                            self.status = "Converted: Datetime -> Unix".into();
                        }
                        MappedLocalTime::Ambiguous(earliest, _) => {
                            self.unix_input = earliest.timestamp().to_string();
                            self.status =
                                "Warning: ambiguous time (DST overlap). Used earlier offset."
                                    .into();
                        }
                        MappedLocalTime::None => {
                            self.status =
                                "Error: this local time does not exist (DST gap)".into();
                        }
                    },
                    Err(_) => {
                        self.status = "Error: expected format yyyy-mm-dd HH:MM:SS".into()
                    }
                }
            }
            Message::Now => {
                let now = Utc::now();
                self.unix_input = now.timestamp().to_string();
                self.datetime_input = now
                    .with_timezone(&self.timezone)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();
                self.status = "Now (both updated)".into();
            }
            Message::CopyUnix => {
                let val = self.unix_input.clone();
                self.status = format!("Copied: {val}");
                return clipboard::write::<Message>(val);
            }
            Message::CopyDatetime => {
                let val = self.datetime_input.clone();
                self.status = format!("Copied: {val}");
                return clipboard::write::<Message>(val);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let unix_row = row![
            text("Unix").width(70),
            text_input("e.g. 1743321600", &self.unix_input)
                .on_input(Message::UnixInputChanged)
                .on_submit(Message::ConvertToDatetime)
                .width(Length::Fill),
            button(center(text("Now").size(14)))
                .on_press(Message::Now)
                .width(50)
                .height(30),
            button(center(text("Copy").size(14)))
                .on_press(Message::CopyUnix)
                .width(50)
                .height(30),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let datetime_row = row![
            text("Date").width(70),
            text_input("yyyy-mm-dd HH:MM:SS", &self.datetime_input)
                .on_input(Message::DatetimeInputChanged)
                .on_submit(Message::ConvertToUnix)
                .width(Length::Fill),
            button(center(text("Now").size(14)))
                .on_press(Message::Now)
                .width(50)
                .height(30),
            button(center(text("Copy").size(14)))
                .on_press(Message::CopyDatetime)
                .width(50)
                .height(30),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let tz_row = row![
            text("TZ").width(70),
            combo_box(&self.tz_state, "e.g. Asia/Tokyo", Some(&self.timezone), Message::TimezoneSelected)
                .on_input(Message::TimezoneInputChanged)
                .width(Length::Fill),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let status_bar = text(&self.status).size(14);

        let content = column![unix_row, datetime_row, tz_row, status_bar]
            .spacing(16)
            .padding(20)
            .width(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
