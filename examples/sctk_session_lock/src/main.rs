use iced::wayland::session_lock;
use iced::{
    event::wayland::{Event as WaylandEvent, OutputEvent, SessionLockEvent},
    subscription,
    wayland::InitialSurface,
    widget::{column, container, text},
    window, Application, Color, Command, Element, Length, Subscription, Theme,
};
use iced_runtime::window::Id as SurfaceId;

fn main() {
    let mut settings = iced::Settings::default();
    settings.initial_surface = InitialSurface::None;
    Locker::run(settings).unwrap();
}

#[derive(Debug, Clone, Default)]
struct Locker {
    max_surface_id: u128,
}

#[derive(Debug, Clone)]
pub enum Message {
    WaylandEvent(WaylandEvent),
    Ignore,
}

impl Locker {
    fn next_surface_id(&mut self) -> SurfaceId {
        self.max_surface_id += 1;
        SurfaceId(self.max_surface_id)
    }
}

impl Application for Locker {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Locker, Command<Self::Message>) {
        (
            Locker {
                ..Locker::default()
            },
            session_lock::lock(),
        )
    }

    fn title(&self) -> String {
        String::from("Locker")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::WaylandEvent(evt) => {
                match evt {
                    // TODO handle creation/removal after lock
                    WaylandEvent::Output(evt, output) => match evt {
                        OutputEvent::Created(_) => {
                            return session_lock::get_lock_surface(
                                self.next_surface_id(),
                                output,
                            );
                        }
                        OutputEvent::Removed => {}
                        _ => {}
                    },
                    WaylandEvent::SessionLock(evt) => match evt {
                        SessionLockEvent::Locked => {}
                        _ => {} // TODO
                    },
                    _ => {}
                }
            }
            Message::Ignore => {}
        }
        Command::none()
    }

    fn view(&self, id: window::Id) -> Element<Self::Message> {
        text("Lock").into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::subscription::events_with(|evt, _| {
            if let iced::Event::PlatformSpecific(
                iced::event::PlatformSpecific::Wayland(evt),
            ) = evt
            {
                Some(Message::WaylandEvent(evt))
            } else {
                None
            }
        })
    }

    fn close_requested(&self, _id: window::Id) -> Self::Message {
        Message::Ignore
    }
}
