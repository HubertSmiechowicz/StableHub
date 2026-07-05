<script setup lang="ts">
import { ClipboardList } from "@lucide/vue";
import type { InventoryItemDetails } from "../types/inventory";
import {
  formatDays,
  formatOptionalQuantity,
  formatQuantity,
  unitLabel
} from "../utils/inventoryLabels";

defineProps<{
  item: InventoryItemDetails;
}>();
</script>

<template>
  <section class="panel entity-details-panel">
    <div class="panel-heading">
      <h2>Szczegóły pozycji</h2>
      <ClipboardList :size="20" aria-hidden="true" />
    </div>

    <div class="entity-details">
      <div class="entity-details-hero">
        <div>
          <p class="eyebrow">Stan magazynowy</p>
          <h3>{{ item.name }}</h3>
        </div>
        <span>{{ item.status }}</span>
      </div>

      <dl class="details-grid">
        <div>
          <dt>Aktualny stan</dt>
          <dd>{{ formatQuantity(item.quantity, item.unit) }}</dd>
        </div>
        <div>
          <dt>Jednostka</dt>
          <dd>{{ unitLabel(item.unit) }}</dd>
        </div>
        <div>
          <dt>Minimalny stan</dt>
          <dd>{{ formatOptionalQuantity(item.minimum_quantity, item.unit) }}</dd>
        </div>
        <div>
          <dt>Dzienne zużycie</dt>
          <dd>{{ formatOptionalQuantity(item.daily_usage, item.unit) }}</dd>
        </div>
        <div>
          <dt>Dni zapasu</dt>
          <dd>{{ formatDays(item.days_remaining) }}</dd>
        </div>
        <div>
          <dt>Utworzono</dt>
          <dd>{{ item.created_at }}</dd>
        </div>
      </dl>
    </div>
  </section>
</template>
