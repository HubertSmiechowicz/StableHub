<script setup lang="ts">
import { reactive, watch } from "vue";
import { CalendarDays, HeartPulse } from "@lucide/vue";
import type { CalendarEntryDetails, CreateCalendarEntryRequest } from "../types/calendar";
import { calendarEntryTypeOptions } from "../utils/calendarLabels";
import DatePickerField from "../../../shared/components/DatePickerField.vue";
import TimePickerField from "../../../shared/components/TimePickerField.vue";

const props = withDefaults(
  defineProps<{
    entry?: CalendarEntryDetails | null;
    initialDate?: string | null;
    title?: string;
    submitLabel?: string;
  }>(),
  {
    entry: null,
    initialDate: null,
    title: "Nowy wpis terminarza",
    submitLabel: "Zapisz wpis"
  }
);

const emit = defineEmits<{
  submit: [request: CreateCalendarEntryRequest & { status?: "planned" | "done" }];
  cancel: [];
  createHealthEvent: [date: string | null];
}>();

const form = reactive({
  title: "",
  scheduled_on: "",
  scheduled_time: "",
  entry_type: "task",
  description: "",
  status: "planned" as "planned" | "done"
});

watch(
  () => [props.entry, props.initialDate] as const,
  ([entry, initialDate]) => {
    form.title = entry?.title ?? "";
    form.scheduled_on = entry?.scheduled_on ?? initialDate ?? "";
    form.scheduled_time = entry?.scheduled_time ?? "";
    form.entry_type = entry?.entry_type ?? "task";
    form.description = entry?.description ?? "";
    form.status = entry?.status === "done" ? "done" : "planned";
  },
  { immediate: true }
);

function submitForm() {
  emit("submit", {
    title: form.title,
    scheduled_on: form.scheduled_on,
    scheduled_time: form.scheduled_time || null,
    entry_type: form.entry_type,
    description: form.description || null,
    status: form.status
  });
}
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <h2>{{ title }}</h2>
      <CalendarDays :size="20" aria-hidden="true" />
    </div>

    <form class="entity-form" @submit.prevent="submitForm">
      <label>
        <span>Tytuł</span>
        <input v-model="form.title" type="text" maxlength="120" required />
      </label>
      <label>
        <span>Typ</span>
        <select v-model="form.entry_type">
          <option v-for="type in calendarEntryTypeOptions" :key="type.value" :value="type.value">
            {{ type.label }}
          </option>
        </select>
      </label>
      <DatePickerField v-model="form.scheduled_on" label="Data" required />
      <TimePickerField v-model="form.scheduled_time" label="Godzina" />
      <label v-if="entry">
        <span>Status</span>
        <select v-model="form.status">
          <option value="planned">Zaplanowane</option>
          <option value="done">Wykonane</option>
        </select>
      </label>
      <label class="full-row">
        <span>Opis</span>
        <textarea v-model="form.description" rows="4" />
      </label>
      <div class="form-actions split-actions">
        <div>
          <button class="secondary-action" type="submit">{{ submitLabel }}</button>
          <button class="ghost-action" type="button" @click="emit('cancel')">Anuluj</button>
        </div>
        <button
          v-if="!entry"
          class="ghost-action"
          type="button"
          @click="emit('createHealthEvent', form.scheduled_on || initialDate)"
        >
          <HeartPulse :size="17" aria-hidden="true" />
          Dodaj zdarzenie zdrowotne
        </button>
      </div>
    </form>
  </section>
</template>
