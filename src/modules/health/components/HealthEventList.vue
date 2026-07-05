<script setup lang="ts">
import { HeartPulse } from "@lucide/vue";
import type { HealthEventSummary } from "../types/health";
import { formatCost, healthEventTypeLabel } from "../utils/healthLabels";

defineProps<{
  events: HealthEventSummary[];
  isLoading: boolean;
}>();

const emit = defineEmits<{
  select: [id: string];
}>();
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <h2>Zdarzenia zdrowotne</h2>
      <HeartPulse :size="20" aria-hidden="true" />
    </div>

    <div class="inventory-list" aria-live="polite">
      <p v-if="isLoading" class="muted">Ładowanie zdarzeń zdrowotnych...</p>
      <p v-else-if="events.length === 0" class="muted">Brak zdarzeń zdrowotnych.</p>
      <button
        v-for="event in events"
        v-else
        :key="event.id"
        class="inventory-row"
        type="button"
        @click="emit('select', event.id)"
      >
        <div>
          <strong>{{ event.title }}</strong>
          <p>
            {{ event.occurred_on }}
            <template v-if="event.occurred_time"> · {{ event.occurred_time }}</template>
            · {{ event.horse_name || "koń nieznany" }} ·
            {{ healthEventTypeLabel(event.event_type) }}
          </p>
        </div>
        <span>{{ formatCost(event.cost) }}</span>
      </button>
    </div>
  </section>
</template>
