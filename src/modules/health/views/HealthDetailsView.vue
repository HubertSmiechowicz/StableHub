<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { archiveHealthEvent, getHealthEventDetails } from "../api/healthApi";
import HealthEventDetailsCard from "../components/HealthEventDetailsCard.vue";
import type { HealthEventDetails } from "../types/health";
import { formatError } from "../../../shared/errors";

const props = defineProps<{
  eventId: string;
}>();

const emit = defineEmits<{
  back: [];
  edit: [id: string];
}>();

const event = ref<HealthEventDetails | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function load() {
  isLoading.value = true;
  error.value = null;

  try {
    event.value = await getHealthEventDetails(props.eventId);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać zdarzenia zdrowotnego.");
  } finally {
    isLoading.value = false;
  }
}

async function archiveCurrentEvent() {
  if (!confirm("Czy na pewno usunąć to zdarzenie zdrowotne?")) {
    return;
  }

  error.value = null;

  try {
    await archiveHealthEvent(props.eventId);
    emit("back");
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się usunąć zdarzenia zdrowotnego.");
  }
}

onMounted(load);
watch(() => props.eventId, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł zdrowia</p>
      <h2>Szczegóły</h2>
    </div>
    <button class="ghost-action" type="button" @click="emit('back')">Wróć do listy</button>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie zdarzenia zdrowotnego.</p>
  </section>
  <HealthEventDetailsCard
    v-else-if="event"
    :event="event"
    @edit="emit('edit', event.id)"
    @archive="archiveCurrentEvent"
  />
  <section v-else class="panel empty-state compact">
    <h2>Nie znaleziono zdarzenia</h2>
    <p>Wróć do listy i wybierz zdarzenie ponownie.</p>
  </section>
</template>
