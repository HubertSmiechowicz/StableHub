<script setup lang="ts">
import { reactive, watch } from "vue";
import { HeartPulse } from "@lucide/vue";
import type { HorseSummary } from "../../horses/types/horse";
import type { CreateHealthEventRequest, HealthEventDetails } from "../types/health";
import { healthEventTypeOptions } from "../utils/healthLabels";
import DatePickerField from "../../../shared/components/DatePickerField.vue";
import TimePickerField from "../../../shared/components/TimePickerField.vue";

const props = withDefaults(
  defineProps<{
    event?: HealthEventDetails | null;
    horses: HorseSummary[];
    initialDate?: string | null;
    title?: string;
    submitLabel?: string;
  }>(),
  {
    event: null,
    initialDate: null,
    title: "Nowe zdarzenie zdrowotne",
    submitLabel: "Zapisz zdarzenie"
  }
);

const emit = defineEmits<{
  submit: [request: CreateHealthEventRequest];
  cancel: [];
}>();

const form = reactive({
  horse_id: "",
  event_type: "checkup",
  occurred_on: "",
  occurred_time: "",
  title: "",
  notes: "",
  cost: null as number | null
});

watch(
  () => [props.event, props.initialDate] as const,
  ([event, initialDate]) => {
    form.horse_id = event?.horse_id ?? props.horses[0]?.id ?? "";
    form.event_type = event?.event_type ?? "checkup";
    form.occurred_on = event?.occurred_on ?? initialDate ?? "";
    form.occurred_time = event?.occurred_time ?? "";
    form.title = event?.title ?? "";
    form.notes = event?.notes ?? "";
    form.cost = event?.cost ?? null;
  },
  { immediate: true }
);

watch(
  () => props.horses,
  (horses) => {
    if (!form.horse_id && horses.length > 0) {
      form.horse_id = horses[0].id;
    }
  },
  { immediate: true }
);

function submitForm() {
  emit("submit", {
    horse_id: form.horse_id,
    event_type: form.event_type,
    occurred_on: form.occurred_on,
    occurred_time: form.occurred_time || null,
    title: form.title,
    notes: form.notes || null,
    cost: form.cost === null ? null : Number(form.cost)
  });
}
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <h2>{{ title }}</h2>
      <HeartPulse :size="20" aria-hidden="true" />
    </div>

    <form class="entity-form" @submit.prevent="submitForm">
      <label>
        <span>Koń</span>
        <select v-model="form.horse_id" required>
          <option value="" disabled>Wybierz konia</option>
          <option v-for="horse in horses" :key="horse.id" :value="horse.id">
            {{ horse.name }}
          </option>
        </select>
      </label>
      <label>
        <span>Typ</span>
        <select v-model="form.event_type">
          <option
            v-for="eventType in healthEventTypeOptions"
            :key="eventType.value"
            :value="eventType.value"
          >
            {{ eventType.label }}
          </option>
        </select>
      </label>
      <DatePickerField v-model="form.occurred_on" label="Data" required />
      <TimePickerField v-model="form.occurred_time" label="Godzina" />
      <label>
        <span>Tytuł</span>
        <input v-model="form.title" type="text" maxlength="120" required />
      </label>
      <label>
        <span>Koszt</span>
        <input v-model.number="form.cost" type="number" min="0" step="0.01" />
      </label>
      <label class="full-row">
        <span>Notatki</span>
        <textarea v-model="form.notes" rows="4" />
      </label>
      <div class="form-actions">
        <button class="secondary-action" type="submit">{{ submitLabel }}</button>
        <button class="ghost-action" type="button" @click="emit('cancel')">Anuluj</button>
      </div>
    </form>
  </section>
</template>
