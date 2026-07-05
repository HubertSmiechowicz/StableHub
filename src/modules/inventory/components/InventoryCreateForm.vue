<script setup lang="ts">
import { reactive } from "vue";
import { Warehouse } from "@lucide/vue";
import type { CreateInventoryItemRequest } from "../types/inventory";
import { inventoryUnitOptions } from "../utils/inventoryLabels";

const emit = defineEmits<{
  submit: [request: CreateInventoryItemRequest];
  cancel: [];
}>();

const form = reactive({
  name: "",
  unit: "kg",
  quantity: 0,
  minimum_quantity: null as number | null,
  daily_usage: null as number | null
});

function submitForm() {
  emit("submit", {
    name: form.name,
    unit: form.unit,
    quantity: Number(form.quantity),
    minimum_quantity:
      form.minimum_quantity === null ? null : Number(form.minimum_quantity),
    daily_usage: form.daily_usage === null ? null : Number(form.daily_usage)
  });
}
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <h2>Nowa pozycja magazynowa</h2>
      <Warehouse :size="20" aria-hidden="true" />
    </div>

    <form class="entity-form" @submit.prevent="submitForm">
      <label>
        <span>Nazwa</span>
        <input v-model="form.name" type="text" maxlength="100" required />
      </label>
      <label>
        <span>Jednostka</span>
        <select v-model="form.unit">
          <option v-for="unit in inventoryUnitOptions" :key="unit.value" :value="unit.value">
            {{ unit.label }}
          </option>
        </select>
      </label>
      <label>
        <span>Aktualny stan</span>
        <input v-model.number="form.quantity" type="number" min="0" step="0.01" required />
      </label>
      <label>
        <span>Minimalny stan</span>
        <input v-model.number="form.minimum_quantity" type="number" min="0" step="0.01" />
      </label>
      <label>
        <span>Średnie dzienne zużycie</span>
        <input v-model.number="form.daily_usage" type="number" min="0" step="0.01" />
      </label>
      <div class="form-actions">
        <button class="secondary-action" type="submit">Zapisz pozycję</button>
        <button class="ghost-action" type="button" @click="emit('cancel')">Anuluj</button>
      </div>
    </form>
  </section>
</template>
