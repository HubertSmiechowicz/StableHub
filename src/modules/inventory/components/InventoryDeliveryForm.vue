<script setup lang="ts">
import { reactive } from "vue";
import { PackagePlus } from "@lucide/vue";
import DatePickerField from "../../../shared/components/DatePickerField.vue";
import type { InventoryItemDetails, RegisterInventoryDeliveryRequest } from "../types/inventory";
import { formatQuantity } from "../utils/inventoryLabels";

const props = defineProps<{
  item: InventoryItemDetails;
}>();

const emit = defineEmits<{
  submit: [request: RegisterInventoryDeliveryRequest];
  cancel: [];
}>();

const today = new Date().toISOString().slice(0, 10);

const form = reactive({
  delivered_on: today,
  quantity: 0,
  total_cost: 0,
  supplier: "",
  notes: ""
});

function submitForm() {
  emit("submit", {
    inventory_item_id: props.item.id,
    delivered_on: form.delivered_on,
    quantity: Number(form.quantity),
    total_cost: Number(form.total_cost),
    supplier: form.supplier.trim() || null,
    notes: form.notes.trim() || null
  });
}
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <div>
        <p class="eyebrow">Dostawa magazynowa</p>
        <h2>{{ item.name }}</h2>
      </div>
      <PackagePlus :size="20" aria-hidden="true" />
    </div>

    <p class="muted delivery-context">
      Aktualny stan: {{ formatQuantity(item.quantity, item.unit) }}
    </p>

    <form class="entity-form" @submit.prevent="submitForm">
      <DatePickerField v-model="form.delivered_on" label="Data dostawy" required />
      <label>
        <span>Ilość</span>
        <input v-model.number="form.quantity" type="number" min="0.01" step="0.01" required />
      </label>
      <label>
        <span>Koszt całkowity</span>
        <input v-model.number="form.total_cost" type="number" min="0" step="0.01" required />
      </label>
      <label>
        <span>Dostawca</span>
        <input v-model="form.supplier" type="text" maxlength="120" />
      </label>
      <label class="full-row">
        <span>Notatki</span>
        <textarea v-model="form.notes" rows="4" maxlength="500" />
      </label>
      <div class="form-actions">
        <button class="secondary-action" type="submit">Zapisz dostawę</button>
        <button class="ghost-action" type="button" @click="emit('cancel')">Anuluj</button>
      </div>
    </form>
  </section>
</template>
