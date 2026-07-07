<script setup lang="ts">
import { computed, reactive } from "vue";
import { ClipboardCheck } from "@lucide/vue";
import DatePickerField from "../../../shared/components/DatePickerField.vue";
import type { InventoryItemDetails, RecordInventoryStocktakeRequest } from "../types/inventory";
import { formatQuantity } from "../utils/inventoryLabels";

const props = defineProps<{
  item: InventoryItemDetails;
}>();

const emit = defineEmits<{
  submit: [request: RecordInventoryStocktakeRequest];
  cancel: [];
}>();

const today = new Date().toISOString().slice(0, 10);

const form = reactive({
  counted_on: today,
  actual_quantity: props.item.quantity,
  notes: ""
});

const expectedQuantity = computed(() =>
  Math.max(0, props.item.quantity - props.item.pending_usage_quantity)
);

const varianceQuantity = computed(() => Number(form.actual_quantity) - expectedQuantity.value);

function submitForm() {
  emit("submit", {
    inventory_item_id: props.item.id,
    counted_on: form.counted_on,
    actual_quantity: Number(form.actual_quantity),
    notes: form.notes.trim() || null
  });
}
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <div>
        <p class="eyebrow">Inwentaryzacja</p>
        <h2>{{ item.name }}</h2>
      </div>
      <ClipboardCheck :size="20" aria-hidden="true" />
    </div>

    <dl class="details-grid inventory-preview">
      <div>
        <dt>Aktualny stan w systemie</dt>
        <dd>{{ formatQuantity(item.quantity, item.unit) }}</dd>
      </div>
      <div>
        <dt>Oczekiwane zużycie</dt>
        <dd>{{ formatQuantity(item.pending_usage_quantity, item.unit) }}</dd>
      </div>
      <div>
        <dt>Oczekiwany stan</dt>
        <dd>{{ formatQuantity(expectedQuantity, item.unit) }}</dd>
      </div>
      <div>
        <dt>Różnica po przeliczeniu</dt>
        <dd>{{ formatQuantity(varianceQuantity, item.unit) }}</dd>
      </div>
    </dl>

    <form class="entity-form" @submit.prevent="submitForm">
      <DatePickerField v-model="form.counted_on" label="Data inwentaryzacji" required />
      <label>
        <span>Faktyczny stan</span>
        <input v-model.number="form.actual_quantity" type="number" min="0" step="0.01" required />
      </label>
      <label class="full-row">
        <span>Notatki</span>
        <textarea v-model="form.notes" rows="4" maxlength="500" />
      </label>
      <div class="form-actions">
        <button class="secondary-action" type="submit">Zapisz inwentaryzację</button>
        <button class="ghost-action" type="button" @click="emit('cancel')">Anuluj</button>
      </div>
    </form>
  </section>
</template>
