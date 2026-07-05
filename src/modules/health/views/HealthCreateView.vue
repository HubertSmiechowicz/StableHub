<script setup lang="ts">
import { onMounted, ref } from "vue";
import { listHorses } from "../../horses/api/horseApi";
import type { HorseSummary } from "../../horses/types/horse";
import { createHealthEvent } from "../api/healthApi";
import HealthEventForm from "../components/HealthEventForm.vue";
import type { CreateHealthEventRequest } from "../types/health";
import { formatError } from "../../../shared/errors";

const emit = defineEmits<{
  created: [id: string];
  cancel: [];
}>();

const horses = ref<HorseSummary[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function loadHorses() {
  isLoading.value = true;
  error.value = null;

  try {
    horses.value = await listHorses({ search: null, sort_by: "name", sort_direction: "asc" });
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać listy koni.");
  } finally {
    isLoading.value = false;
  }
}

async function submit(request: CreateHealthEventRequest) {
  error.value = null;

  try {
    const event = await createHealthEvent(request);
    emit("created", event.id);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się zapisać zdarzenia zdrowotnego.");
  }
}

onMounted(loadHorses);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł zdrowia</p>
      <h2>Dodaj zdarzenie</h2>
    </div>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie listy koni.</p>
  </section>
  <HealthEventForm
    v-else
    :horses="horses"
    @submit="submit"
    @cancel="emit('cancel')"
  />
</template>
