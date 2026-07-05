<script setup lang="ts">
import { ref } from "vue";
import { createCalendarEntry } from "../api/calendarApi";
import CalendarEntryForm from "../components/CalendarEntryForm.vue";
import type { CreateCalendarEntryRequest } from "../types/calendar";
import { formatError } from "../../../shared/errors";

const props = defineProps<{
  initialDate: string | null;
}>();

const emit = defineEmits<{
  created: [id: string];
  cancel: [];
  createHealthEvent: [date: string | null];
}>();

const error = ref<string | null>(null);

async function submit(request: CreateCalendarEntryRequest) {
  error.value = null;

  try {
    const entry = await createCalendarEntry(request);
    emit("created", entry.id);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się zapisać wpisu kalendarza.");
  }
}
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł kalendarza</p>
      <h2>Dodaj wpis</h2>
    </div>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <CalendarEntryForm
    :initial-date="props.initialDate"
    @submit="submit"
    @cancel="emit('cancel')"
    @create-health-event="emit('createHealthEvent', $event)"
  />
</template>
