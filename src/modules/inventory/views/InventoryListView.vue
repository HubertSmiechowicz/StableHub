<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Plus } from "@lucide/vue";
import { listInventoryItems } from "../api/inventoryApi";
import InventoryItemList from "../components/InventoryItemList.vue";
import type { InventoryItemSummary } from "../types/inventory";
import { formatError } from "../../../shared/errors";

const emit = defineEmits<{
  create: [];
  select: [id: string];
}>();

const items = ref<InventoryItemSummary[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function load() {
  isLoading.value = true;
  error.value = null;

  try {
    items.value = await listInventoryItems();
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać listy pozycji magazynowych.");
  } finally {
    isLoading.value = false;
  }
}

onMounted(load);
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

  <p v-if="error" class="error-message">{{ error }}</p>

  <InventoryItemList :items="items" :is-loading="isLoading" @select="emit('select', $event)" />
</template>
