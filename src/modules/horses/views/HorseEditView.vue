<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { getHorseDetails, updateHorse } from "../api/horseApi";
import HorseCreateForm from "../components/HorseCreateForm.vue";
import type { CreateHorseRequest, HorseDetails } from "../types/horse";
import { formatError } from "../../../shared/errors";

const props = defineProps<{
  horseId: string;
}>();

const emit = defineEmits<{
  saved: [id: string];
  cancel: [];
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
    error.value = formatError(caught, "Nie udało się pobrać profilu konia.");
  } finally {
    isLoading.value = false;
  }
}

async function submit(request: CreateHorseRequest) {
  error.value = null;

  try {
    const updated = await updateHorse({ ...request, id: props.horseId });
    emit("saved", updated.id);
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się zaktualizować konia.");
  }
}

onMounted(load);
watch(() => props.horseId, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł koni</p>
      <h2>Edytuj konia</h2>
    </div>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>
  <section v-if="isLoading" class="panel empty-state compact">
    <h2>Ładowanie</h2>
    <p>Trwa pobieranie profilu konia.</p>
  </section>
  <HorseCreateForm
    v-else-if="horse"
    :horse="horse"
    title="Edycja konia"
    submit-label="Zapisz zmiany"
    @submit="submit"
    @cancel="emit('cancel')"
  />
</template>
