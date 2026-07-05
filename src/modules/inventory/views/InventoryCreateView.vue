<script setup lang="ts">
import { ref } from "vue";
import { createInventoryItem } from "../api/inventoryApi";
import InventoryCreateForm from "../components/InventoryCreateForm.vue";
import type { CreateInventoryItemRequest } from "../types/inventory";
import { formatError } from "../../../shared/errors";

const emit = defineEmits<{
  created: [id: string];
  cancel: [];
}>();

const error = ref<string | null>(null);

async function submit(request: CreateInventoryItemRequest) {
  error.value = null;

  try {
    const item = await createInventoryItem(request);
    emit("created", item.id);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się zapisać pozycji magazynowej.");
  }
}
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł magazynu</p>
      <h2>Dodaj pozycję</h2>
    </div>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <InventoryCreateForm @submit="submit" @cancel="emit('cancel')" />
</template>
