<script setup lang="ts">
import { ClipboardList } from "@lucide/vue";
import type { HealthEventDetails } from "../types/health";
import {
  displayValue,
  formatCost,
  healthEventTypeLabel
} from "../utils/healthLabels";

defineProps<{
  event: HealthEventDetails;
}>();

const emit = defineEmits<{
  edit: [];
  archive: [];
}>();
</script>

<template>
  <section class="panel entity-details-panel">
    <div class="panel-heading">
      <h2>Szczegóły zdarzenia</h2>
      <div class="panel-actions">
        <button class="ghost-action" type="button" @click="emit('edit')">Edytuj</button>
        <button class="danger-action" type="button" @click="emit('archive')">Usuń</button>
        <ClipboardList :size="20" aria-hidden="true" />
      </div>
    </div>

    <div class="entity-details">
      <div class="entity-details-hero">
        <div>
          <p class="eyebrow">{{ healthEventTypeLabel(event.event_type) }}</p>
          <h3>{{ event.title }}</h3>
        </div>
        <span>{{ event.status }}</span>
      </div>

      <dl class="details-grid">
        <div>
          <dt>Koń</dt>
          <dd>{{ displayValue(event.horse_name) }}</dd>
        </div>
        <div>
          <dt>Data</dt>
          <dd>{{ event.occurred_on }}</dd>
        </div>
        <div>
          <dt>Godzina</dt>
          <dd>{{ displayValue(event.occurred_time) }}</dd>
        </div>
        <div>
          <dt>Typ</dt>
          <dd>{{ healthEventTypeLabel(event.event_type) }}</dd>
        </div>
        <div>
          <dt>Koszt</dt>
          <dd>{{ formatCost(event.cost) }}</dd>
        </div>
        <div>
          <dt>Utworzono</dt>
          <dd>{{ event.created_at }}</dd>
        </div>
        <div>
          <dt>Zaktualizowano</dt>
          <dd>{{ event.updated_at }}</dd>
        </div>
      </dl>

      <div class="details-note">
        <dt>Notatki</dt>
        <dd>{{ displayValue(event.notes) }}</dd>
      </div>
    </div>
  </section>
</template>
