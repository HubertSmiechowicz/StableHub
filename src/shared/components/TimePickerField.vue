<script setup lang="ts">
import { ref } from "vue";
import { Clock } from "@lucide/vue";

defineProps<{
  modelValue: string;
  label: string;
  required?: boolean;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const input = ref<HTMLInputElement | null>(null);

function openPicker() {
  const picker = input.value as (HTMLInputElement & { showPicker?: () => void }) | null;

  if (picker?.showPicker) {
    picker.showPicker();
    return;
  }

  picker?.focus();
}

function blockManualInput(event: Event) {
  event.preventDefault();
}
</script>

<template>
  <label class="picker-field">
    <span>{{ label }}</span>
    <div class="picker-control" role="button" tabindex="0" @click="openPicker" @keydown.enter.prevent="openPicker" @keydown.space.prevent="openPicker">
      <input
        ref="input"
        :value="modelValue"
        type="time"
        inputmode="none"
        :required="required"
        @keydown="blockManualInput"
        @paste="blockManualInput"
        @drop="blockManualInput"
        @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      />
      <button type="button" class="picker-button" :aria-label="`Wybierz godzinę: ${label}`" @click.stop="openPicker">
        <Clock :size="17" aria-hidden="true" />
      </button>
    </div>
  </label>
</template>
