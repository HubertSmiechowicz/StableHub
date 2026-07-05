<script setup lang="ts">
import {
  CalendarDays,
  ChevronRight,
  ClipboardList,
  Coins,
  HeartPulse,
  Home,
  Package,
  Settings,
  Sprout,
  Warehouse
} from "@lucide/vue";

const navItems = [
  { label: "Dashboard", icon: Home, active: true },
  { label: "Konie", icon: Sprout },
  { label: "Magazyn", icon: Warehouse },
  { label: "Finanse", icon: Coins },
  { label: "Kalendarz", icon: CalendarDays },
  { label: "Zdrowie", icon: HeartPulse },
  { label: "Raporty", icon: ClipboardList },
  { label: "Ustawienia", icon: Settings }
];

const metrics = [
  { label: "Konie", value: "2", detail: "aktywnie prowadzone profile" },
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
  <main class="app-shell">
    <aside class="sidebar">
      <div class="brand">
        <div class="brand-mark">SH</div>
        <div>
          <p class="brand-name">StableHub</p>
          <p class="brand-subtitle">Offline stable manager</p>
        </div>
      </div>

      <nav class="nav-list" aria-label="Główna nawigacja">
        <button
          v-for="item in navItems"
          :key="item.label"
          class="nav-item"
          :class="{ active: item.active }"
          type="button"
        >
          <component :is="item.icon" :size="18" aria-hidden="true" />
          <span>{{ item.label }}</span>
        </button>
      </nav>
    </aside>

    <section class="workspace">
      <header class="topbar">
        <div>
          <p class="eyebrow">Prywatna stajnia</p>
          <h1>Witaj w StableHub</h1>
        </div>
        <button class="primary-action" type="button">
          Dodaj konia
          <ChevronRight :size="18" aria-hidden="true" />
        </button>
      </header>

      <section class="welcome-band" aria-labelledby="welcome-title">
        <div class="welcome-copy">
          <p class="eyebrow">MVP desktop offline-first</p>
          <h2 id="welcome-title">Jedno miejsce na konie, zdrowie, zapasy i koszty.</h2>
          <p>
            StableHub porządkuje codzienne obowiązki właściciela małej stajni:
            przypomnienia, magazyn pasz, historię zdrowia i planowanie zimy.
          </p>
        </div>
        <div class="stable-snapshot" aria-label="Szybki podgląd stajni">
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
            <h2>Start pracy</h2>
            <ClipboardList :size="20" aria-hidden="true" />
          </div>
          <div class="task-list">
            <label>
              <input type="checkbox" checked />
              Utwórz profile koni
            </label>
            <label>
              <input type="checkbox" />
              Wprowadź aktualne zapasy
            </label>
            <label>
              <input type="checkbox" />
              Dodaj cykliczne przypomnienia
            </label>
          </div>
        </article>
      </section>
    </section>
  </main>
</template>
