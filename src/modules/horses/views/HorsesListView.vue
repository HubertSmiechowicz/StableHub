<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Plus } from "@lucide/vue";
import { listHorses } from "../api/horseApi";
import HorseList from "../components/HorseList.vue";
import type { HorseSummary } from "../types/horse";

const emit = defineEmits<{
  create: [];
  select: [id: string];
}>();

const horses = ref<HorseSummary[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function load() {
  isLoading.value = true;
  error.value = null;

  try {
    horses.value = await listHorses();
  } catch (caught) {
    error.value = String(caught);
  } finally {
    isLoading.value = false;
  }
}

onMounted(load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł koni</p>
      <h2>Lista koni</h2>
    </div>
    <button class="secondary-action" type="button" @click="emit('create')">
      <Plus :size="17" aria-hidden="true" />
      Dodaj konia
    </button>
  </section>

  <p v-if="error" class="error-message">{{ error }}</p>

  <HorseList :horses="horses" :is-loading="isLoading" @select="emit('select', $event)" />
</template>
