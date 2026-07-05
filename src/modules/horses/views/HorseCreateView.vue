<script setup lang="ts">
import { ref } from "vue";
import { createHorse } from "../api/horseApi";
import HorseCreateForm from "../components/HorseCreateForm.vue";
import type { CreateHorseRequest } from "../types/horse";
import { formatError } from "../../../shared/errors";

const emit = defineEmits<{
  created: [id: string];
  cancel: [];
}>();

const error = ref<string | null>(null);

async function submit(request: CreateHorseRequest) {
  error.value = null;

  try {
    const horse = await createHorse(request);
    emit("created", horse.id);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się zapisać konia.");
  }
}
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł koni</p>
      <h2>Dodaj konia</h2>
    </div>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <HorseCreateForm @submit="submit" @cancel="emit('cancel')" />
</template>
