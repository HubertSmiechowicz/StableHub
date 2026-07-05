<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { getHorseDetails } from "../api/horseApi";
import HorseDetailsCard from "../components/HorseDetailsCard.vue";
import type { HorseDetails } from "../types/horse";

const props = defineProps<{
  horseId: string;
}>();

const emit = defineEmits<{
  back: [];
}>();

const horse = ref<HorseDetails | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function load() {
  isLoading.value = true;
  error.value = null;

  try {
    horse.value = await getHorseDetails(props.horseId);
  } catch (caught) {
    error.value = String(caught);
  } finally {
    isLoading.value = false;
  }
}

onMounted(load);
watch(() => props.horseId, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł koni</p>
      <h2>Szczegóły</h2>
    </div>
    <button class="ghost-action" type="button" @click="emit('back')">Wróć do listy</button>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie profilu konia.</p>
  </section>
  <HorseDetailsCard v-else-if="horse" :horse="horse" />
  <section v-else class="panel empty-state compact">
    <h2>Nie znaleziono konia</h2>
    <p>Wróć do listy i wybierz profil ponownie.</p>
  </section>
</template>
