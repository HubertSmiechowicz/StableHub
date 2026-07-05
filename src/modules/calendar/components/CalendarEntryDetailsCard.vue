<script setup lang="ts">
import { ClipboardList } from "@lucide/vue";
import type { CalendarEntryDetails } from "../types/calendar";
import { calendarEntryTypeLabel, displayValue } from "../utils/calendarLabels";

defineProps<{
  entry: CalendarEntryDetails;
}>();

const emit = defineEmits<{
  edit: [];
  archive: [];
}>();
</script>

<template>
  <section class="panel entity-details-panel">
    <div class="panel-heading">
      <h2>Szczegóły wpisu</h2>
      <div class="panel-actions">
        <button class="ghost-action" type="button" @click="emit('edit')">Edytuj</button>
        <button class="danger-action" type="button" @click="emit('archive')">Usuń</button>
        <ClipboardList :size="20" aria-hidden="true" />
      </div>
    </div>

    <div class="entity-details">
      <div class="entity-details-hero">
        <div>
          <p class="eyebrow">{{ calendarEntryTypeLabel(entry.entry_type) }}</p>
          <h3>{{ entry.title }}</h3>
        </div>
        <span>{{ entry.status }}</span>
      </div>

      <dl class="details-grid">
        <div>
          <dt>Data</dt>
          <dd>{{ entry.scheduled_on }}</dd>
        </div>
        <div>
          <dt>Godzina</dt>
          <dd>{{ displayValue(entry.scheduled_time) }}</dd>
        </div>
        <div>
          <dt>Typ</dt>
          <dd>{{ calendarEntryTypeLabel(entry.entry_type) }}</dd>
        </div>
        <div>
          <dt>Utworzono</dt>
          <dd>{{ entry.created_at }}</dd>
        </div>
      </dl>

      <div class="details-note">
        <dt>Opis</dt>
        <dd>{{ displayValue(entry.description) }}</dd>
      </div>
    </div>
  </section>
</template>
