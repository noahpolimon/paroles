use mpris::Event;

pub struct EventState {
    current: Event,
    updated: bool,
}

impl EventState {
    pub fn new(event: Event) -> EventState {
        Self {
            current: event,
            updated: false,
        }
    }

    pub fn update(&mut self, event: Event) {
        self.current = event;
        self.updated = true;
    }

    pub fn mark_outdated(&mut self) {
        self.updated = false;
    }

    pub fn current_event(&self) -> &Event {
        &self.current
    }

    pub fn is_updated(&self) -> bool {
        self.updated
    }
}
