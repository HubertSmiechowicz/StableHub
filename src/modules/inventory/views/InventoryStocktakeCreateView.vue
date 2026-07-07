<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import {
  getInventoryItemDetails,
  recordInventoryStocktake
} from "../api/inventoryApi";
import InventoryStocktakeForm from "../components/InventoryStocktakeForm.vue";
import type {
  InventoryItemDetails,
  RecordInventoryStocktakeRequest
} from "../types/inventory";
import { formatError } from "../../../shared/errors";

const props = defineProps<{
  itemId: string;
}>();

const emit = defineEmits<{
  saved: [id: string];
  cancel: [id: string];
}>();

const item = ref<InventoryItemDetails | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function load() {
  isLoading.value = true;
  error.value = null;

  try {
    item.value = await getInventoryItemDetails(props.itemId);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać pozycji magazynowej.");
  } finally {
    isLoading.value = false;
  }
}

async function submit(request: RecordInventoryStocktakeRequest) {
  error.value = null;

  try {
    const updated = await recordInventoryStocktake(request);
    emit("saved", updated.id);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się zapisać inwentaryzacji.");
  }
}

onMounted(load);
watch(() => props.itemId, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł magazynu</p>
      <h2>Inwentaryzacja</h2>
    </div>
    <button class="ghost-action" type="button" @click="emit('cancel', itemId)">
      Wróć do szczegółów
    </button>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie pozycji magazynowej.</p>
  </section>
  <InventoryStocktakeForm
    v-else-if="item"
    :item="item"
    @submit="submit"
    @cancel="emit('cancel', item.id)"
  />
  <section v-else class="panel empty-state compact">
    <h2>Nie znaleziono pozycji</h2>
    <p>Wróć do listy i wybierz pozycję ponownie.</p>
  </section>
</template>
