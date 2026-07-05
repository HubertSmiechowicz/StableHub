<script setup lang="ts">
import { ClipboardList } from "@lucide/vue";
import type { HorseDetails } from "../types/horse";
import { displayValue, sexLabel } from "../utils/horseLabels";

defineProps<{
  horse: HorseDetails;
}>();

const emit = defineEmits<{
  edit: [];
  archive: [];
}>();
</script>

<template>
  <section class="panel horse-details-panel">
    <div class="panel-heading">
      <h2>Profil konia</h2>
      <div class="panel-actions">
        <button class="ghost-action" type="button" @click="emit('edit')">Edytuj</button>
        <button class="danger-action" type="button" @click="emit('archive')">Usuń</button>
        <ClipboardList :size="20" aria-hidden="true" />
      </div>
    </div>

    <div class="horse-details">
      <div class="horse-details-hero">
        <div>
          <p class="eyebrow">Profil podstawowy</p>
          <h3>{{ horse.name }}</h3>
        </div>
        <span>{{ horse.status }}</span>
      </div>

      <dl class="details-grid">
        <div>
          <dt>Płeć</dt>
          <dd>{{ sexLabel(horse.sex) }}</dd>
        </div>
        <div>
          <dt>Rasa</dt>
          <dd>{{ displayValue(horse.breed) }}</dd>
        </div>
        <div>
          <dt>Data urodzenia</dt>
          <dd>{{ displayValue(horse.date_of_birth) }}</dd>
        </div>
        <div>
          <dt>Maść</dt>
          <dd>{{ displayValue(horse.coat_color) }}</dd>
        </div>
        <div>
          <dt>Identyfikator</dt>
          <dd>{{ displayValue(horse.identification_number) }}</dd>
        </div>
        <div>
          <dt>Utworzono</dt>
          <dd>{{ horse.created_at }}</dd>
        </div>
      </dl>

      <div class="details-note">
        <dt>Notatki</dt>
        <dd>{{ displayValue(horse.notes) }}</dd>
      </div>
    </div>
  </section>
</template>
