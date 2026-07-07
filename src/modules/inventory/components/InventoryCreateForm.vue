<script setup lang="ts">
import { reactive, watch } from "vue";
import { Warehouse } from "@lucide/vue";
import type { CreateInventoryItemRequest, InventoryItemDetails } from "../types/inventory";
import { formatQuantity, inventoryUnitOptions } from "../utils/inventoryLabels";

const props = withDefaults(
  defineProps<{
    item?: InventoryItemDetails | null;
    title?: string;
    submitLabel?: string;
  }>(),
  {
    item: null,
    title: "Nowa pozycja magazynowa",
    submitLabel: "Zapisz pozycję"
  }
);

const emit = defineEmits<{
  submit: [request: CreateInventoryItemRequest];
  cancel: [];
}>();

const form = reactive({
  name: "",
  unit: "kg",
  minimum_quantity: null as number | null,
  daily_usage: null as number | null
});

watch(
  () => props.item,
  (item) => {
    form.name = item?.name ?? "";
    form.unit = item?.unit ?? "kg";
    form.minimum_quantity = item?.minimum_quantity ?? null;
    form.daily_usage = item?.daily_usage ?? null;
  },
  { immediate: true }
);

function submitForm() {
  emit("submit", {
    name: form.name,
    unit: form.unit,
    minimum_quantity:
      form.minimum_quantity === null ? null : Number(form.minimum_quantity),
    daily_usage: form.daily_usage === null ? null : Number(form.daily_usage)
  });
}
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <h2>{{ title }}</h2>
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
        <strong class="readonly-value">
          {{ formatQuantity(item?.quantity ?? 0, item?.unit ?? form.unit) }}
        </strong>
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
        <button class="secondary-action" type="submit">{{ submitLabel }}</button>
        <button class="ghost-action" type="button" @click="emit('cancel')">Anuluj</button>
      </div>
    </form>
  </section>
</template>
