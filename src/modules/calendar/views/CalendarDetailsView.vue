<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { archiveCalendarEntry, getCalendarEntryDetails } from "../api/calendarApi";
import CalendarEntryDetailsCard from "../components/CalendarEntryDetailsCard.vue";
import type { CalendarEntryDetails } from "../types/calendar";
import { formatError } from "../../../shared/errors";

const props = defineProps<{
  entryId: string;
}>();

const emit = defineEmits<{
  back: [];
  edit: [id: string];
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

async function archiveCurrentEntry() {
  if (!confirm("Czy na pewno usunąć ten wpis kalendarza?")) {
    return;
  }

  error.value = null;

  try {
    await archiveCalendarEntry(props.entryId);
    emit("back");
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się usunąć wpisu kalendarza.");
  }
}

onMounted(load);
watch(() => props.entryId, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł kalendarza</p>
      <h2>Szczegóły</h2>
    </div>
    <button class="ghost-action" type="button" @click="emit('back')">Wróć do listy</button>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie wpisu kalendarza.</p>
  </section>
  <CalendarEntryDetailsCard
    v-else-if="entry"
    :entry="entry"
    @edit="emit('edit', entry.id)"
    @archive="archiveCurrentEntry"
  />
  <section v-else class="panel empty-state compact">
    <h2>Nie znaleziono wpisu</h2>
    <p>Wróć do listy i wybierz wpis ponownie.</p>
  </section>
</template>
