<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import {
  applyInventoryUsage,
  archiveInventoryItem,
  getInventoryItemDetails
} from "../api/inventoryApi";
import InventoryItemDetailsCard from "../components/InventoryItemDetailsCard.vue";
import type { InventoryItemDetails } from "../types/inventory";
import { formatError } from "../../../shared/errors";

const props = defineProps<{
  itemId: string;
}>();

const emit = defineEmits<{
  back: [];
  edit: [id: string];
  registerDelivery: [id: string];
  recordStocktake: [id: string];
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

async function archiveCurrentItem() {
  if (!confirm("Czy na pewno usunąć tę pozycję magazynową?")) {
    return;
  }

  error.value = null;

  try {
    await archiveInventoryItem(props.itemId);
    emit("back");
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się usunąć pozycji magazynowej.");
  }
}

async function applyCurrentUsage() {
  error.value = null;

  try {
    item.value = await applyInventoryUsage(props.itemId);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się rozliczyć zużycia magazynowego.");
  }
}

onMounted(load);
watch(() => props.itemId, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł magazynu</p>
      <h2>Szczegóły</h2>
    </div>
    <button class="ghost-action" type="button" @click="emit('back')">Wróć do listy</button>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie pozycji magazynowej.</p>
  </section>
  <InventoryItemDetailsCard
    v-else-if="item"
    :item="item"
    @edit="emit('edit', item.id)"
    @archive="archiveCurrentItem"
    @apply-usage="applyCurrentUsage"
    @register-delivery="emit('registerDelivery', item.id)"
    @record-stocktake="emit('recordStocktake', item.id)"
  />
  <section v-else class="panel empty-state compact">
    <h2>Nie znaleziono pozycji</h2>
    <p>Wróć do listy i wybierz pozycję ponownie.</p>
  </section>
</template>
