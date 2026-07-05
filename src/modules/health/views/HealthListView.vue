<script setup lang="ts">
import { onMounted, reactive, ref, watch } from "vue";
import { Plus } from "@lucide/vue";
import { listHorses } from "../../horses/api/horseApi";
import type { HorseSummary } from "../../horses/types/horse";
import { listHealthEvents } from "../api/healthApi";
import HealthEventList from "../components/HealthEventList.vue";
import type { HealthEventSummary, HealthListRequest } from "../types/health";
import { healthEventTypeOptions } from "../utils/healthLabels";
import { formatError } from "../../../shared/errors";

const emit = defineEmits<{
  create: [];
  select: [id: string];
}>();

const events = ref<HealthEventSummary[]>([]);
const horses = ref<HorseSummary[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);
const filters = reactive<HealthListRequest>({
  search: null,
  horse_id: null,
  event_type: null,
  sort_by: "occurred_on",
  sort_direction: "desc"
});

let loadSequence = 0;

async function load() {
  const sequence = ++loadSequence;
  isLoading.value = true;
  error.value = null;

  try {
    const [eventsResult, horsesResult] = await Promise.all([
      listHealthEvents({
        ...filters,
        search: filters.search?.trim() || null,
        horse_id: filters.horse_id || null,
        event_type: filters.event_type || null
      }),
      listHorses({ search: null, sort_by: "name", sort_direction: "asc" })
    ]);

    if (sequence === loadSequence) {
      events.value = eventsResult;
      horses.value = horsesResult;
    }
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać listy zdarzeń zdrowotnych.");
  } finally {
    if (sequence === loadSequence) {
      isLoading.value = false;
    }
  }
}

onMounted(load);
watch(filters, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł zdrowia</p>
      <h2>Lista zdarzeń</h2>
    </div>
    <button class="secondary-action" type="button" @click="emit('create')">
      <Plus :size="17" aria-hidden="true" />
      Dodaj zdarzenie
    </button>
  </section>

  <section class="list-controls health-controls" aria-label="Filtrowanie listy zdrowia">
    <label>
      <span>Szukaj</span>
      <input v-model="filters.search" type="search" placeholder="Tytuł lub notatka" />
    </label>
    <label>
      <span>Koń</span>
      <select v-model="filters.horse_id">
        <option :value="null">Wszystkie</option>
        <option v-for="horse in horses" :key="horse.id" :value="horse.id">
          {{ horse.name }}
        </option>
      </select>
    </label>
    <label>
      <span>Typ</span>
      <select v-model="filters.event_type">
        <option :value="null">Wszystkie</option>
        <option
          v-for="eventType in healthEventTypeOptions"
          :key="eventType.value"
          :value="eventType.value"
        >
          {{ eventType.label }}
        </option>
      </select>
    </label>
    <label>
      <span>Sortuj</span>
      <select v-model="filters.sort_by">
        <option value="occurred_on">Data</option>
        <option value="title">Tytuł</option>
        <option value="type">Typ</option>
        <option value="horse">Koń</option>
        <option value="created_at">Data dodania</option>
      </select>
    </label>
    <label>
      <span>Kierunek</span>
      <select v-model="filters.sort_direction">
        <option value="desc">Malejąco</option>
        <option value="asc">Rosnąco</option>
      </select>
    </label>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>

  <HealthEventList :events="events" :is-loading="isLoading" @select="emit('select', $event)" />
</template>
