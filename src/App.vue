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
import CalendarCreateView from "./modules/calendar/views/CalendarCreateView.vue";
import CalendarDetailsView from "./modules/calendar/views/CalendarDetailsView.vue";
import CalendarEditView from "./modules/calendar/views/CalendarEditView.vue";
import CalendarListView from "./modules/calendar/views/CalendarListView.vue";
import DashboardView from "./modules/dashboard/views/DashboardView.vue";
import HealthCreateView from "./modules/health/views/HealthCreateView.vue";
import HealthDetailsView from "./modules/health/views/HealthDetailsView.vue";
import HealthEditView from "./modules/health/views/HealthEditView.vue";
import HealthListView from "./modules/health/views/HealthListView.vue";
import HorseCreateView from "./modules/horses/views/HorseCreateView.vue";
import HorseDetailsView from "./modules/horses/views/HorseDetailsView.vue";
import HorseEditView from "./modules/horses/views/HorseEditView.vue";
import HorsesListView from "./modules/horses/views/HorsesListView.vue";
import InventoryCreateView from "./modules/inventory/views/InventoryCreateView.vue";
import InventoryDetailsView from "./modules/inventory/views/InventoryDetailsView.vue";
import InventoryEditView from "./modules/inventory/views/InventoryEditView.vue";
import InventoryListView from "./modules/inventory/views/InventoryListView.vue";

type ViewId =
  | "dashboard"
  | "horses"
  | "horse-create"
  | "horse-details"
  | "horse-edit"
  | "inventory"
  | "inventory-create"
  | "inventory-details"
  | "inventory-edit"
  | "finance"
  | "calendar"
  | "calendar-create"
  | "calendar-details"
  | "calendar-edit"
  | "health"
  | "health-create"
  | "health-details"
  | "health-edit"
  | "reports"
  | "settings";

const currentView = ref<ViewId>("dashboard");
const selectedHorseId = ref<string | null>(null);
const selectedInventoryItemId = ref<string | null>(null);
const selectedHealthEventId = ref<string | null>(null);
const selectedCalendarEntryId = ref<string | null>(null);
const selectedCalendarDate = ref<string | null>(null);
const selectedHealthEventDate = ref<string | null>(null);

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
    case "horse-edit":
      return "Edytuj konia";
    case "inventory":
      return "Magazyn";
    case "inventory-create":
      return "Nowa pozycja";
    case "inventory-details":
      return "Szczegóły pozycji";
    case "inventory-edit":
      return "Edytuj pozycję";
    case "calendar":
      return "Kalendarz";
    case "calendar-create":
      return "Nowy wpis";
    case "calendar-details":
      return "Szczegóły wpisu";
    case "calendar-edit":
      return "Edytuj wpis";
    case "finance":
      return "Finanse";
    case "health":
      return "Zdrowie";
    case "health-create":
      return "Nowe zdarzenie";
    case "health-details":
      return "Szczegóły zdarzenia";
    case "health-edit":
      return "Edytuj zdarzenie";
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
    currentView.value === "horse-details" ||
    currentView.value === "horse-edit"
  ) {
    return "Moduł koni";
  }

  if (
    currentView.value === "inventory" ||
    currentView.value === "inventory-create" ||
    currentView.value === "inventory-details" ||
    currentView.value === "inventory-edit"
  ) {
    return "Moduł magazynu";
  }

  if (
    currentView.value === "calendar" ||
    currentView.value === "calendar-create" ||
    currentView.value === "calendar-details" ||
    currentView.value === "calendar-edit"
  ) {
    return "Moduł kalendarza";
  }

  if (
    currentView.value === "health" ||
    currentView.value === "health-create" ||
    currentView.value === "health-details" ||
    currentView.value === "health-edit"
  ) {
    return "Moduł zdrowia";
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

function openHorseEdit(id: string) {
  selectedHorseId.value = id;
  currentView.value = "horse-edit";
}

function backToHorseList() {
  currentView.value = "horses";
}

function openInventoryDetails(id: string) {
  selectedInventoryItemId.value = id;
  currentView.value = "inventory-details";
}

function openInventoryEdit(id: string) {
  selectedInventoryItemId.value = id;
  currentView.value = "inventory-edit";
}

function backToInventoryList() {
  currentView.value = "inventory";
}

function openHealthEventDetails(id: string) {
  selectedHealthEventId.value = id;
  currentView.value = "health-details";
}

function openHealthEventEdit(id: string) {
  selectedHealthEventId.value = id;
  currentView.value = "health-edit";
}

function backToHealthList() {
  currentView.value = "health";
}

function openHealthCreate(date: string | null = null) {
  selectedHealthEventDate.value = date;
  currentView.value = "health-create";
}

function openCalendarEntryDetails(id: string) {
  selectedCalendarEntryId.value = id;
  currentView.value = "calendar-details";
}

function openCalendarEntryEdit(id: string) {
  selectedCalendarEntryId.value = id;
  currentView.value = "calendar-edit";
}

function backToCalendarList() {
  currentView.value = "calendar";
}

function openCalendarCreate(date: string | null = null) {
  selectedCalendarDate.value = date;
  currentView.value = "calendar-create";
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
                (currentView === 'horse-create' ||
                  currentView === 'horse-details' ||
                  currentView === 'horse-edit')) ||
              (item.id === 'inventory' &&
                (currentView === 'inventory-create' ||
                  currentView === 'inventory-details' ||
                  currentView === 'inventory-edit')) ||
              (item.id === 'calendar' &&
                (currentView === 'calendar-create' ||
                  currentView === 'calendar-details' ||
                  currentView === 'calendar-edit')) ||
              (item.id === 'health' &&
                (currentView === 'health-create' ||
                  currentView === 'health-details' ||
                  currentView === 'health-edit'))
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
        @edit="openHorseEdit"
      />
      <HorseEditView
        v-else-if="currentView === 'horse-edit' && selectedHorseId"
        :horse-id="selectedHorseId"
        @saved="openHorseDetails"
        @cancel="openHorseDetails(selectedHorseId)"
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
        @edit="openInventoryEdit"
      />
      <InventoryEditView
        v-else-if="currentView === 'inventory-edit' && selectedInventoryItemId"
        :item-id="selectedInventoryItemId"
        @saved="openInventoryDetails"
        @cancel="openInventoryDetails(selectedInventoryItemId)"
      />
      <CalendarListView
        v-else-if="currentView === 'calendar'"
        @create="openCalendarCreate()"
        @create-for-date="openCalendarCreate"
        @select-calendar-entry="openCalendarEntryDetails"
        @select-health-event="openHealthEventDetails"
      />
      <CalendarCreateView
        v-else-if="currentView === 'calendar-create'"
        :initial-date="selectedCalendarDate"
        @created="openCalendarEntryDetails"
        @cancel="backToCalendarList"
        @create-health-event="openHealthCreate"
      />
      <CalendarDetailsView
        v-else-if="currentView === 'calendar-details' && selectedCalendarEntryId"
        :entry-id="selectedCalendarEntryId"
        @back="backToCalendarList"
        @edit="openCalendarEntryEdit"
      />
      <CalendarEditView
        v-else-if="currentView === 'calendar-edit' && selectedCalendarEntryId"
        :entry-id="selectedCalendarEntryId"
        @saved="openCalendarEntryDetails"
        @cancel="openCalendarEntryDetails(selectedCalendarEntryId)"
      />
      <HealthListView
        v-else-if="currentView === 'health'"
        @create="openHealthCreate()"
        @select="openHealthEventDetails"
      />
      <HealthCreateView
        v-else-if="currentView === 'health-create'"
        :initial-date="selectedHealthEventDate"
        @created="openHealthEventDetails"
        @cancel="backToHealthList"
      />
      <HealthDetailsView
        v-else-if="currentView === 'health-details' && selectedHealthEventId"
        :event-id="selectedHealthEventId"
        @back="backToHealthList"
        @edit="openHealthEventEdit"
      />
      <HealthEditView
        v-else-if="currentView === 'health-edit' && selectedHealthEventId"
        :event-id="selectedHealthEventId"
        @saved="openHealthEventDetails"
        @cancel="openHealthEventDetails(selectedHealthEventId)"
      />
      <section v-else class="panel empty-state">
        <h2>{{ pageTitle }}</h2>
        <p>Ten moduł czeka na implementację.</p>
      </section>
    </section>
  </main>
</template>
