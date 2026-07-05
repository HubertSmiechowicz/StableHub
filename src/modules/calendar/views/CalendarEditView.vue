<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { getCalendarEntryDetails, updateCalendarEntry } from "../api/calendarApi";
import CalendarEntryForm from "../components/CalendarEntryForm.vue";
import type {
  CalendarEntryDetails,
  CreateCalendarEntryRequest,
  UpdateCalendarEntryRequest
} from "../types/calendar";
import { formatError } from "../../../shared/errors";

const props = defineProps<{
  entryId: string;
}>();

const emit = defineEmits<{
  saved: [id: string];
  cancel: [];
}>();

const entry = ref<CalendarEntryDetails | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function load() {
  isLoading.value = true;
  error.value = null;

  try {
    entry.value = await getCalendarEntryDetails(props.entryId);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać wpisu kalendarza.");
  } finally {
    isLoading.value = false;
  }
}

async function submit(request: CreateCalendarEntryRequest & { status?: "planned" | "done" }) {
  error.value = null;

  try {
    const payload: UpdateCalendarEntryRequest = {
      ...request,
      id: props.entryId,
      status: request.status ?? "planned"
    };
    const updated = await updateCalendarEntry(payload);
    emit("saved", updated.id);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się zaktualizować wpisu kalendarza.");
  }
}

onMounted(load);
watch(() => props.entryId, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł kalendarza</p>
      <h2>Edytuj wpis</h2>
    </div>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie wpisu kalendarza.</p>
  </section>
  <CalendarEntryForm
    v-else-if="entry"
    :entry="entry"
    title="Edycja wpisu terminarza"
    submit-label="Zapisz zmiany"
    @submit="submit"
    @cancel="emit('cancel')"
  />
</template>
