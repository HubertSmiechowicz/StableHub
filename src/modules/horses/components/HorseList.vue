<script setup lang="ts">
import { Sprout } from "@lucide/vue";
import type { HorseSummary } from "../types/horse";
import { sexLabel } from "../utils/horseLabels";

defineProps<{
  horses: HorseSummary[];
  isLoading: boolean;
}>();

const emit = defineEmits<{
  select: [id: string];
}>();
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <h2>Aktywne konie</h2>
      <Sprout :size="20" aria-hidden="true" />
    </div>

    <div class="horse-list" aria-live="polite">
      <p v-if="isLoading" class="muted">Ładowanie koni...</p>
      <p v-else-if="horses.length === 0" class="muted">Brak zapisanych koni.</p>
      <button
        v-for="horse in horses"
        v-else
        :key="horse.id"
        class="horse-row"
        type="button"
        @click="emit('select', horse.id)"
      >
        <div>
          <strong>{{ horse.name }}</strong>
          <p>{{ sexLabel(horse.sex) }} · {{ horse.breed || "rasa niepodana" }}</p>
        </div>
        <span>{{ horse.status }}</span>
      </button>
    </div>
  </section>
</template>
