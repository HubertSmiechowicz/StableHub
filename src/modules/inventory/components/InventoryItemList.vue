<script setup lang="ts">
import { Warehouse } from "@lucide/vue";
import type { InventoryItemSummary } from "../types/inventory";
import { formatDays, formatQuantity } from "../utils/inventoryLabels";

defineProps<{
  items: InventoryItemSummary[];
  isLoading: boolean;
}>();

const emit = defineEmits<{
  select: [id: string];
}>();
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <h2>Pozycje magazynowe</h2>
      <Warehouse :size="20" aria-hidden="true" />
    </div>

    <div class="inventory-list" aria-live="polite">
      <p v-if="isLoading" class="muted">Ładowanie magazynu...</p>
      <p v-else-if="items.length === 0" class="muted">Brak pozycji magazynowych.</p>
      <button
        v-for="item in items"
        v-else
        :key="item.id"
        class="inventory-row"
        type="button"
        @click="emit('select', item.id)"
      >
        <div>
          <strong>{{ item.name }}</strong>
          <p>{{ formatQuantity(item.quantity, item.unit) }}</p>
        </div>
        <span>{{ formatDays(item.days_remaining) }}</span>
      </button>
    </div>
  </section>
</template>
