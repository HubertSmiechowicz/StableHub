<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { getInventoryItemDetails, updateInventoryItem } from "../api/inventoryApi";
import InventoryCreateForm from "../components/InventoryCreateForm.vue";
import type {
  CreateInventoryItemRequest,
  InventoryItemDetails
} from "../types/inventory";
import { formatError } from "../../../shared/errors";

const props = defineProps<{
  itemId: string;
}>();

const emit = defineEmits<{
  saved: [id: string];
  cancel: [];
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

async function submit(request: CreateInventoryItemRequest) {
  error.value = null;

  try {
    const updated = await updateInventoryItem({ ...request, id: props.itemId });
    emit("saved", updated.id);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się zaktualizować pozycji magazynowej.");
  }
}

onMounted(load);
watch(() => props.itemId, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł magazynu</p>
      <h2>Edytuj pozycję</h2>
    </div>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie pozycji magazynowej.</p>
  </section>
  <InventoryCreateForm
    v-else-if="item"
    :item="item"
    title="Edycja pozycji magazynowej"
    submit-label="Zapisz zmiany"
    @submit="submit"
    @cancel="emit('cancel')"
  />
</template>
