<script setup lang="ts">
import { CalendarDays } from "@lucide/vue";
import type { CalendarItemSummary } from "../types/calendar";
import {
  calendarItemTypeLabel,
  displayValue,
  sourceModuleLabel
} from "../utils/calendarLabels";

defineProps<{
  items: CalendarItemSummary[];
  isLoading: boolean;
}>();

const emit = defineEmits<{
  select: [item: CalendarItemSummary];
}>();
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <h2>Terminarz</h2>
      <CalendarDays :size="20" aria-hidden="true" />
    </div>

    <div class="inventory-list" aria-live="polite">
      <p v-if="isLoading" class="muted">Ładowanie terminarza...</p>
      <p v-else-if="items.length === 0" class="muted">Brak wpisów w terminarzu.</p>
      <button
        v-for="item in items"
        v-else
        :key="`${item.source_module}-${item.source_id}`"
        class="inventory-row"
        type="button"
        @click="emit('select', item)"
      >
        <div>
          <strong>{{ item.title }}</strong>
          <p>
            {{ item.scheduled_on }}
            <template v-if="item.scheduled_time"> · {{ item.scheduled_time }}</template>
            · {{ sourceModuleLabel(item.source_module) }}
            · {{ calendarItemTypeLabel(item.source_module, item.item_type) }}
            <template v-if="item.related_label"> · {{ item.related_label }}</template>
          </p>
        </div>
        <span>{{ displayValue(item.status) }}</span>
      </button>
    </div>
  </section>
</template>
