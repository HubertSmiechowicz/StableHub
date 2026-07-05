<script setup lang="ts">
import { onMounted, reactive, ref, watch } from "vue";
import { Plus } from "@lucide/vue";
import { listInventoryItems } from "../api/inventoryApi";
import InventoryItemList from "../components/InventoryItemList.vue";
import type { InventoryItemSummary, InventoryListRequest } from "../types/inventory";
import { formatError } from "../../../shared/errors";

const emit = defineEmits<{
  create: [];
  select: [id: string];
}>();

const items = ref<InventoryItemSummary[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);
const filters = reactive<InventoryListRequest>({
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
    const result = await listInventoryItems(request);

    if (sequence === loadSequence) {
      items.value = result;
    }
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać listy pozycji magazynowych.");
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
      <p class="eyebrow">Moduł magazynu</p>
      <h2>Lista pozycji</h2>
    </div>
    <button class="secondary-action" type="button" @click="emit('create')">
      <Plus :size="17" aria-hidden="true" />
      Dodaj pozycję
    </button>
  </section>

  <section class="list-controls" aria-label="Filtrowanie listy magazynu">
    <label>
      <span>Szukaj</span>
      <input v-model="filters.search" type="search" placeholder="Nazwa pozycji" />
    </label>
    <label>
      <span>Sortuj</span>
      <select v-model="filters.sort_by">
        <option value="name">Nazwa</option>
        <option value="quantity">Stan</option>
        <option value="days_remaining">Dni zapasu</option>
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

  <InventoryItemList :items="items" :is-loading="isLoading" @select="emit('select', $event)" />
</template>
