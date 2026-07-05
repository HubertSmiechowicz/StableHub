use super::{
    HealthEventDate, HealthEventId, HealthEventStatus, HealthEventTitle, HealthEventType,
    HorseReference,
};

#[derive(Debug, Clone)]
pub struct HealthEvent {
    id: HealthEventId,
    horse_id: HorseReference,
    event_type: HealthEventType,
    occurred_on: HealthEventDate,
    title: HealthEventTitle,
    status: HealthEventStatus,
}

impl HealthEvent {
    pub fn create(
        id: HealthEventId,
        horse_id: HorseReference,
        event_type: HealthEventType,
        occurred_on: HealthEventDate,
        title: HealthEventTitle,
    ) -> Self {
        Self {
            id,
            horse_id,
            event_type,
            occurred_on,
            title,
            status: HealthEventStatus::Active,
        }
    }

    pub fn from_existing(
        id: HealthEventId,
        horse_id: HorseReference,
        event_type: HealthEventType,
        occurred_on: HealthEventDate,
        title: HealthEventTitle,
        status: HealthEventStatus,
    ) -> Self {
        Self {
            id,
            horse_id,
            event_type,
            occurred_on,
            title,
            status,
        }
    }

    pub fn update(
        &mut self,
        horse_id: HorseReference,
        event_type: HealthEventType,
        occurred_on: HealthEventDate,
        title: HealthEventTitle,
    ) {
        self.horse_id = horse_id;
        self.event_type = event_type;
        self.occurred_on = occurred_on;
        self.title = title;
    }

    pub fn archive(&mut self) {
        self.status = HealthEventStatus::Archived;
    }

    pub fn id(&self) -> &HealthEventId {
        &self.id
    }

    pub fn horse_id(&self) -> &HorseReference {
        &self.horse_id
    }

    pub fn event_type(&self) -> HealthEventType {
        self.event_type
    }

    pub fn occurred_on(&self) -> &HealthEventDate {
        &self.occurred_on
    }

    pub fn title(&self) -> &HealthEventTitle {
        &self.title
    }

    pub fn status(&self) -> HealthEventStatus {
        self.status
    }
}
