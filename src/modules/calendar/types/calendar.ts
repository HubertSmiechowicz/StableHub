export type CalendarItemSummary = {
  id: string;
  source_module: "calendar" | "health" | string;
  source_id: string;
  title: string;
  scheduled_on: string;
  scheduled_time: string | null;
  item_type: string;
  description: string | null;
  status: string;
  related_label: string | null;
};

export type CalendarEntryDetails = {
  id: string;
  title: string;
  scheduled_on: string;
  scheduled_time: string | null;
  entry_type: string;
  description: string | null;
  status: string;
  created_at: string;
  updated_at: string;
  archived_at: string | null;
};

export type CreateCalendarEntryRequest = {
  title: string;
  scheduled_on: string;
  scheduled_time: string | null;
  entry_type: string;
  description: string | null;
};

export type UpdateCalendarEntryRequest = CreateCalendarEntryRequest & {
  id: string;
  status: "planned" | "done";
};

export type CalendarListRequest = {
  search: string | null;
  source_module: string | null;
  item_type: string | null;
  date_from: string | null;
  date_to: string | null;
  sort_by: "scheduled_on" | "title" | "type" | "source";
  sort_direction: "asc" | "desc";
};
