export type HorseSummary = {
  id: string;
  name: string;
  sex: string | null;
  breed: string | null;
  status: string;
};

export type HorseDetails = HorseSummary & {
  date_of_birth: string | null;
  coat_color: string | null;
  identification_number: string | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
  archived_at: string | null;
};

export type CreateHorseRequest = {
  name: string;
  sex: string | null;
  breed: string | null;
  date_of_birth: string | null;
  coat_color: string | null;
  identification_number: string | null;
  notes: string | null;
};
