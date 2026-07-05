use super::{
    CalendarEntryDate, CalendarEntryId, CalendarEntryStatus, CalendarEntryTitle, CalendarEntryType,
};

#[derive(Debug, Clone)]
pub struct CalendarEntry {
    id: CalendarEntryId,
    title: CalendarEntryTitle,
    scheduled_on: CalendarEntryDate,
    entry_type: CalendarEntryType,
    status: CalendarEntryStatus,
}

impl CalendarEntry {
    pub fn create(
        id: CalendarEntryId,
        title: CalendarEntryTitle,
        scheduled_on: CalendarEntryDate,
        entry_type: CalendarEntryType,
    ) -> Self {
        Self {
            id,
            title,
            scheduled_on,
            entry_type,
            status: CalendarEntryStatus::Planned,
        }
    }

    pub fn from_existing(
        id: CalendarEntryId,
        title: CalendarEntryTitle,
        scheduled_on: CalendarEntryDate,
        entry_type: CalendarEntryType,
        status: CalendarEntryStatus,
    ) -> Self {
        Self {
            id,
            title,
            scheduled_on,
            entry_type,
            status,
        }
    }

    pub fn update(
        &mut self,
        title: CalendarEntryTitle,
        scheduled_on: CalendarEntryDate,
        entry_type: CalendarEntryType,
        status: CalendarEntryStatus,
    ) {
        self.title = title;
        self.scheduled_on = scheduled_on;
        self.entry_type = entry_type;
        self.status = status;
    }

    pub fn archive(&mut self) {
        self.status = CalendarEntryStatus::Archived;
    }

    pub fn id(&self) -> &CalendarEntryId {
        &self.id
    }

    pub fn title(&self) -> &CalendarEntryTitle {
        &self.title
    }

    pub fn scheduled_on(&self) -> &CalendarEntryDate {
        &self.scheduled_on
    }

    pub fn entry_type(&self) -> CalendarEntryType {
        self.entry_type
    }

    pub fn status(&self) -> CalendarEntryStatus {
        self.status
    }
}
