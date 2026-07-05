<script setup lang="ts">
import { computed, ref } from "vue";
import {
  CalendarDays,
  ClipboardList,
  Coins,
  HeartPulse,
  Home,
  Settings,
  Sprout,
  Warehouse
} from "@lucide/vue";
import DashboardView from "./modules/dashboard/views/DashboardView.vue";
import HorseCreateView from "./modules/horses/views/HorseCreateView.vue";
import HorseDetailsView from "./modules/horses/views/HorseDetailsView.vue";
import HorsesListView from "./modules/horses/views/HorsesListView.vue";
import InventoryCreateView from "./modules/inventory/views/InventoryCreateView.vue";
import InventoryDetailsView from "./modules/inventory/views/InventoryDetailsView.vue";
import InventoryListView from "./modules/inventory/views/InventoryListView.vue";

type ViewId =
  | "dashboard"
  | "horses"
  | "horse-create"
  | "horse-details"
  | "inventory"
  | "inventory-create"
  | "inventory-details"
  | "finance"
  | "calendar"
  | "health"
  | "reports"
  | "settings";

const currentView = ref<ViewId>("dashboard");
const selectedHorseId = ref<string | null>(null);
const selectedInventoryItemId = ref<string | null>(null);

const navItems = [
  { id: "dashboard" as const, label: "Dashboard", icon: Home },
  { id: "horses" as const, label: "Konie", icon: Sprout },
  { id: "inventory" as const, label: "Magazyn", icon: Warehouse },
  { id: "finance" as const, label: "Finanse", icon: Coins },
  { id: "calendar" as const, label: "Kalendarz", icon: CalendarDays },
  { id: "health" as const, label: "Zdrowie", icon: HeartPulse },
  { id: "reports" as const, label: "Raporty", icon: ClipboardList },
  { id: "settings" as const, label: "Ustawienia", icon: Settings }
];

const pageTitle = computed(() => {
  switch (currentView.value) {
    case "horses":
      return "Konie";
    case "horse-create":
      return "Nowy koń";
    case "horse-details":
      return "Szczegóły konia";
    case "inventory":
      return "Magazyn";
    case "inventory-create":
      return "Nowa pozycja";
    case "inventory-details":
      return "Szczegóły pozycji";
    case "finance":
      return "Finanse";
    case "calendar":
      return "Kalendarz";
    case "health":
      return "Zdrowie";
    case "reports":
      return "Raporty";
    case "settings":
      return "Ustawienia";
    default:
      return "Dashboard";
  }
});

const pageSubtitle = computed(() => {
  if (currentView.value === "dashboard") {
    return "Prywatna stajnia";
  }

  if (
    currentView.value === "horses" ||
    currentView.value === "horse-create" ||
    currentView.value === "horse-details"
  ) {
    return "Moduł koni";
  }

  if (
    currentView.value === "inventory" ||
    currentView.value === "inventory-create" ||
    currentView.value === "inventory-details"
  ) {
    return "Moduł magazynu";
  }

  return "Moduł w przygotowaniu";
});

function selectView(view: ViewId) {
  currentView.value = view;
}

function openHorseDetails(id: string) {
  selectedHorseId.value = id;
  currentView.value = "horse-details";
}

function backToHorseList() {
  currentView.value = "horses";
}

function openInventoryDetails(id: string) {
  selectedInventoryItemId.value = id;
  currentView.value = "inventory-details";
}

function backToInventoryList() {
  currentView.value = "inventory";
}
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
          :key="item.id"
          class="nav-item"
          :class="{
            active:
              currentView === item.id ||
              (item.id === 'horses' &&
                (currentView === 'horse-create' || currentView === 'horse-details')) ||
              (item.id === 'inventory' &&
                (currentView === 'inventory-create' || currentView === 'inventory-details'))
          }"
          type="button"
          @click="selectView(item.id)"
        >
          <component :is="item.icon" :size="18" aria-hidden="true" />
          <span>{{ item.label }}</span>
        </button>
      </nav>
    </aside>

    <section class="workspace">
      <header class="topbar">
        <div>
          <p class="eyebrow">{{ pageSubtitle }}</p>
          <h1>{{ pageTitle }}</h1>
        </div>
      </header>

      <DashboardView v-if="currentView === 'dashboard'" />
      <HorsesListView
        v-else-if="currentView === 'horses'"
        @create="currentView = 'horse-create'"
        @select="openHorseDetails"
      />
      <HorseCreateView
        v-else-if="currentView === 'horse-create'"
        @created="openHorseDetails"
        @cancel="backToHorseList"
      />
      <HorseDetailsView
        v-else-if="currentView === 'horse-details' && selectedHorseId"
        :horse-id="selectedHorseId"
        @back="backToHorseList"
      />
      <InventoryListView
        v-else-if="currentView === 'inventory'"
        @create="currentView = 'inventory-create'"
        @select="openInventoryDetails"
      />
      <InventoryCreateView
        v-else-if="currentView === 'inventory-create'"
        @created="openInventoryDetails"
        @cancel="backToInventoryList"
      />
      <InventoryDetailsView
        v-else-if="currentView === 'inventory-details' && selectedInventoryItemId"
        :item-id="selectedInventoryItemId"
        @back="backToInventoryList"
      />
      <section v-else class="panel empty-state">
        <h2>{{ pageTitle }}</h2>
        <p>Ten moduł czeka na implementację.</p>
      </section>
    </section>
  </main>
</template>
