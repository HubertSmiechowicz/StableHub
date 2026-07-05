<script setup lang="ts">
import { CalendarDays, ClipboardList, Package } from "@lucide/vue";

const metrics = [
  { label: "Konie", value: "—", detail: "aktywnie prowadzone profile" },
  { label: "Siano", value: "124", detail: "kostki, ok. 68 dni zapasu" },
  { label: "Owies", value: "210 kg", detail: "ok. 41 dni zapasu" },
  { label: "Koszty lipca", value: "1832 zł", detail: "pasza, kowal, weterynarz" }
];

const events = [
  { label: "Kowal", when: "za 4 dni", tone: "amber" },
  { label: "Szczepienie Hadesa", when: "za 21 dni", tone: "emerald" },
  { label: "Kontrola zapasu siana", when: "w piątek", tone: "blue" }
];
</script>

<template>
  <section class="welcome-band" aria-labelledby="welcome-title">
    <div class="welcome-copy">
      <p class="eyebrow">MVP desktop offline-first</p>
      <h2 id="welcome-title">Bieżący obraz stajni, zapasów i najbliższych obowiązków.</h2>
      <p>
        Dashboard będzie miejscem na aktualne metryki, powiadomienia, alerty magazynowe
        oraz przypomnienia wymagające reakcji.
      </p>
    </div>
    <div class="stable-snapshot" aria-label="Szybki podgląd planowania">
      <div class="snapshot-header">
        <Package :size="18" aria-hidden="true" />
        <span>Planowanie zimy</span>
      </div>
      <strong>312 kostek</strong>
      <span>siana potrzebne od listopada do marca</span>
    </div>
  </section>

  <section class="metric-grid" aria-label="Podsumowanie">
    <article v-for="metric in metrics" :key="metric.label" class="metric-card">
      <span>{{ metric.label }}</span>
      <strong>{{ metric.value }}</strong>
      <p>{{ metric.detail }}</p>
    </article>
  </section>

  <section class="content-grid">
    <article class="panel">
      <div class="panel-heading">
        <h2>Najbliższe wydarzenia</h2>
        <CalendarDays :size="20" aria-hidden="true" />
      </div>
      <ul class="event-list">
        <li v-for="event in events" :key="event.label">
          <span class="event-dot" :class="event.tone"></span>
          <div>
            <strong>{{ event.label }}</strong>
            <p>{{ event.when }}</p>
          </div>
        </li>
      </ul>
    </article>

    <article class="panel">
      <div class="panel-heading">
        <h2>Powiadomienia</h2>
        <ClipboardList :size="20" aria-hidden="true" />
      </div>
      <div class="task-list">
        <label>
          <input type="checkbox" checked />
          Sprawdź zapas siana przed weekendem
        </label>
        <label>
          <input type="checkbox" />
          Uzupełnij koszty z ostatniej wizyty kowala
        </label>
        <label>
          <input type="checkbox" />
          Zaplanuj odrobaczenie na kolejny miesiąc
        </label>
      </div>
    </article>
  </section>
</template>
