<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { listHorses } from "../../horses/api/horseApi";
import type { HorseSummary } from "../../horses/types/horse";
import { getHealthEventDetails, updateHealthEvent } from "../api/healthApi";
import HealthEventForm from "../components/HealthEventForm.vue";
import type { CreateHealthEventRequest, HealthEventDetails } from "../types/health";
import { formatError } from "../../../shared/errors";

const props = defineProps<{
  eventId: string;
}>();

const emit = defineEmits<{
  saved: [id: string];
  cancel: [];
}>();

const event = ref<HealthEventDetails | null>(null);
const horses = ref<HorseSummary[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function load() {
  isLoading.value = true;
  error.value = null;

  try {
    const [eventResult, horsesResult] = await Promise.all([
      getHealthEventDetails(props.eventId),
      listHorses({ search: null, sort_by: "name", sort_direction: "asc" })
    ]);
    event.value = eventResult;
    horses.value = horsesResult;
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać zdarzenia zdrowotnego.");
  } finally {
    isLoading.value = false;
  }
}

async function submit(request: CreateHealthEventRequest) {
  error.value = null;

  try {
    const updated = await updateHealthEvent({ ...request, id: props.eventId });
    emit("saved", updated.id);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się zaktualizować zdarzenia zdrowotnego.");
  }
}

onMounted(load);
watch(() => props.eventId, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł zdrowia</p>
      <h2>Edytuj zdarzenie</h2>
    </div>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie zdarzenia zdrowotnego.</p>
  </section>
  <HealthEventForm
    v-else-if="event"
    :event="event"
    :horses="horses"
    title="Edycja zdarzenia zdrowotnego"
    submit-label="Zapisz zmiany"
    @submit="submit"
    @cancel="emit('cancel')"
  />
</template>
