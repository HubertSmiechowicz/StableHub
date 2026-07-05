<script setup lang="ts">
import { onMounted, reactive, ref, watch } from "vue";
import { Plus } from "@lucide/vue";
import { listHorses } from "../api/horseApi";
import HorseList from "../components/HorseList.vue";
import type { HorseListRequest, HorseSummary } from "../types/horse";
import { formatError } from "../../../shared/errors";

const emit = defineEmits<{
  create: [];
  select: [id: string];
}>();

const horses = ref<HorseSummary[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);
const filters = reactive<HorseListRequest>({
  search: null,
  sort_by: "name",
  sort_direction: "asc"
});

let loadSequence = 0;

async function load() {
  const sequence = ++loadSequence;
  isLoading.value = true;
  error.value = null;

  try {
    const request = {
      ...filters,
      search: filters.search?.trim() || null
    };
    const result = await listHorses(request);

    if (sequence === loadSequence) {
      horses.value = result;
    }
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać listy koni.");
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
      <p class="eyebrow">Moduł koni</p>
      <h2>Lista koni</h2>
    </div>
    <button class="secondary-action" type="button" @click="emit('create')">
      <Plus :size="17" aria-hidden="true" />
      Dodaj konia
    </button>
  </section>

  <section class="list-controls" aria-label="Filtrowanie listy koni">
    <label>
      <span>Szukaj</span>
      <input v-model="filters.search" type="search" placeholder="Imię lub rasa" />
    </label>
    <label>
      <span>Sortuj</span>
      <select v-model="filters.sort_by">
        <option value="name">Imię</option>
        <option value="breed">Rasa</option>
        <option value="created_at">Data dodania</option>
      </select>
    </label>
    <label>
      <span>Kierunek</span>
      <select v-model="filters.sort_direction">
        <option value="asc">Rosnąco</option>
        <option value="desc">Malejąco</option>
      </select>
    </label>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>

  <HorseList :horses="horses" :is-loading="isLoading" @select="emit('select', $event)" />
</template>
