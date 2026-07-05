export type HealthEventSummary = {
  id: string;
  horse_id: string;
  horse_name: string | null;
  event_type: string;
  occurred_on: string;
  occurred_time: string | null;
  title: string;
  cost: number | null;
  status: string;
};

export type HealthEventDetails = HealthEventSummary & {
  notes: string | null;
  created_at: string;
  updated_at: string;
  archived_at: string | null;
};

export type CreateHealthEventRequest = {
  horse_id: string;
  event_type: string;
  occurred_on: string;
  occurred_time: string | null;
  title: string;
  notes: string | null;
  cost: number | null;
};

export type UpdateHealthEventRequest = CreateHealthEventRequest & {
  id: string;
};

export type HealthListRequest = {
  search: string | null;
  horse_id: string | null;
  event_type: string | null;
  sort_by: "occurred_on" | "title" | "type" | "horse" | "created_at";
  sort_direction: "asc" | "desc";
};
