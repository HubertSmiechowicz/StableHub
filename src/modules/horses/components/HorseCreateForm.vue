<script setup lang="ts">
import { reactive } from "vue";
import { Sprout } from "@lucide/vue";
import type { CreateHorseRequest } from "../types/horse";

const emit = defineEmits<{
  submit: [request: CreateHorseRequest];
  cancel: [];
}>();

const form = reactive({
  name: "",
  sex: "unknown",
  breed: "",
  date_of_birth: "",
  coat_color: "",
  identification_number: "",
  notes: ""
});

function submitForm() {
  emit("submit", {
    name: form.name,
    sex: form.sex || null,
    breed: form.breed || null,
    date_of_birth: form.date_of_birth || null,
    coat_color: form.coat_color || null,
    identification_number: form.identification_number || null,
    notes: form.notes || null
  });
}
</script>

<template>
  <section class="panel">
    <div class="panel-heading">
      <h2>Nowy koń</h2>
      <Sprout :size="20" aria-hidden="true" />
    </div>

    <form class="horse-form" @submit.prevent="submitForm">
      <label>
        <span>Imię</span>
        <input v-model="form.name" type="text" maxlength="80" required />
      </label>
      <label>
        <span>Płeć</span>
        <select v-model="form.sex">
          <option value="unknown">Nie podano</option>
          <option value="mare">Klacz</option>
          <option value="stallion">Ogier</option>
          <option value="gelding">Wałach</option>
        </select>
      </label>
      <label>
        <span>Rasa</span>
        <input v-model="form.breed" list="horse-breed-suggestions" type="text" />
        <datalist id="horse-breed-suggestions">
          <option value="Arabski" />
          <option value="Pełnej krwi angielskiej" />
          <option value="Małopolski" />
          <option value="Wielkopolski" />
          <option value="Śląski" />
          <option value="Huculski" />
          <option value="Konik polski" />
          <option value="Fryzyjski" />
          <option value="Hanowerski" />
          <option value="Holsztyński" />
          <option value="Quarter Horse" />
          <option value="Appaloosa" />
          <option value="Kuc walijski" />
          <option value="Szetlandzki" />
        </datalist>
      </label>
      <div class="form-actions">
        <button class="secondary-action" type="submit">Zapisz konia</button>
        <button class="ghost-action" type="button" @click="emit('cancel')">Anuluj</button>
      </div>
    </form>
  </section>
</template>
