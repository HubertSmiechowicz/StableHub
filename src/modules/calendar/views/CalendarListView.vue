<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { ChevronLeft, ChevronRight, Plus } from "@lucide/vue";
import { listCalendarItems } from "../api/calendarApi";
import type { CalendarItemSummary } from "../types/calendar";
import {
  calendarItemTypeLabel,
  sourceModuleLabel
} from "../utils/calendarLabels";
import { formatError } from "../../../shared/errors";

const emit = defineEmits<{
  create: [];
  createForDate: [date: string];
  selectCalendarEntry: [id: string];
  selectHealthEvent: [id: string];
}>();

type CalendarDay = {
  key: string;
  date: Date;
  dateText: string;
  dayNumber: number;
  isCurrentMonth: boolean;
  isToday: boolean;
  items: CalendarItemSummary[];
};

const dayLabels = ["Pon", "Wt", "Śr", "Czw", "Pt", "Sob", "Nd"];
const items = ref<CalendarItemSummary[]>([]);
const visibleMonth = ref(startOfMonth(new Date()));
const isLoading = ref(false);
const error = ref<string | null>(null);

const monthLabel = computed(() =>
  new Intl.DateTimeFormat("pl-PL", { month: "long", year: "numeric" }).format(
    visibleMonth.value
  )
);

const calendarDays = computed<CalendarDay[]>(() => {
  const monthStart = startOfMonth(visibleMonth.value);
  const monthEnd = endOfMonth(visibleMonth.value);
  const gridStart = addDays(monthStart, -mondayIndex(monthStart));
  const gridEnd = addDays(monthEnd, 6 - mondayIndex(monthEnd));
  const todayText = toDateText(new Date());
  const byDate = groupItemsByDate(items.value);
  const days: CalendarDay[] = [];

  for (let date = gridStart; date <= gridEnd; date = addDays(date, 1)) {
    const dateText = toDateText(date);
    days.push({
      key: dateText,
      date,
      dateText,
      dayNumber: date.getDate(),
      isCurrentMonth: date.getMonth() === monthStart.getMonth(),
      isToday: dateText === todayText,
      items: byDate.get(dateText) ?? []
    });
  }

  return days;
});

let loadSequence = 0;

async function load() {
  const sequence = ++loadSequence;
  isLoading.value = true;
  error.value = null;

  try {
    const result = await listCalendarItems({
      search: null,
      source_module: null,
      item_type: null,
      date_from: toDateText(startOfMonth(visibleMonth.value)),
      date_to: toDateText(endOfMonth(visibleMonth.value)),
      sort_by: "scheduled_on",
      sort_direction: "asc"
    });

    if (sequence === loadSequence) {
      items.value = result;
    }
  } catch (caught) {
    error.value = formatError(caught, "Nie udało się pobrać terminarza.");
  } finally {
    if (sequence === loadSequence) {
      isLoading.value = false;
    }
  }
}

function selectItem(item: CalendarItemSummary) {
  if (item.source_module === "health") {
    emit("selectHealthEvent", item.source_id);
    return;
  }

  emit("selectCalendarEntry", item.source_id);
}

function previousMonth() {
  visibleMonth.value = new Date(
    visibleMonth.value.getFullYear(),
    visibleMonth.value.getMonth() - 1,
    1
  );
}

function nextMonth() {
  visibleMonth.value = new Date(
    visibleMonth.value.getFullYear(),
    visibleMonth.value.getMonth() + 1,
    1
  );
}

function goToCurrentMonth() {
  visibleMonth.value = startOfMonth(new Date());
}

function startOfMonth(date: Date) {
  return new Date(date.getFullYear(), date.getMonth(), 1);
}

function endOfMonth(date: Date) {
  return new Date(date.getFullYear(), date.getMonth() + 1, 0);
}

function addDays(date: Date, days: number) {
  const next = new Date(date);
  next.setDate(next.getDate() + days);
  return next;
}

function mondayIndex(date: Date) {
  return (date.getDay() + 6) % 7;
}

function toDateText(date: Date) {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

function groupItemsByDate(calendarItems: CalendarItemSummary[]) {
  const groups = calendarItems.reduce((groups, item) => {
    const dayItems = groups.get(item.scheduled_on) ?? [];
    dayItems.push(item);
    groups.set(item.scheduled_on, dayItems);
    return groups;
  }, new Map<string, CalendarItemSummary[]>());

  groups.forEach((dayItems) => {
    dayItems.sort((first, second) => {
      const firstTime = first.scheduled_time ?? "99:99";
      const secondTime = second.scheduled_time ?? "99:99";
      return firstTime.localeCompare(secondTime) || first.title.localeCompare(second.title, "pl");
    });
  });

  return groups;
}

onMounted(load);
watch(visibleMonth, load);
</script>

<template>
  <section class="section-toolbar">
    <div>
      <p class="eyebrow">Moduł kalendarza</p>
      <h2>Kalendarz</h2>
    </div>
    <button class="secondary-action" type="button" @click="emit('create')">
      <Plus :size="17" aria-hidden="true" />
      Dodaj wpis
    </button>
  </section>

  <section class="panel calendar-panel">
    <div class="calendar-header">
      <div>
        <p class="eyebrow">Widok miesiąca</p>
        <h2>{{ monthLabel }}</h2>
      </div>
      <div class="calendar-nav">
        <button class="ghost-action icon-action" type="button" @click="previousMonth">
          <ChevronLeft :size="18" aria-hidden="true" />
        </button>
        <button class="ghost-action" type="button" @click="goToCurrentMonth">Dzisiaj</button>
        <button class="ghost-action icon-action" type="button" @click="nextMonth">
          <ChevronRight :size="18" aria-hidden="true" />
        </button>
      </div>
    </div>

    <p v-if="error" class="error-message">{{ error }}</p>
    <p v-if="isLoading" class="muted">Ładowanie kalendarza...</p>

    <div class="calendar-weekdays" aria-hidden="true">
      <span v-for="day in dayLabels" :key="day">{{ day }}</span>
    </div>

    <div class="calendar-grid" aria-live="polite">
      <article
        v-for="day in calendarDays"
        :key="day.key"
        class="calendar-day"
        :class="{ muted: !day.isCurrentMonth, today: day.isToday }"
        role="button"
        tabindex="0"
        @click="emit('createForDate', day.dateText)"
        @keydown.enter.prevent="emit('createForDate', day.dateText)"
        @keydown.space.prevent="emit('createForDate', day.dateText)"
      >
        <header>
          <span>{{ day.dayNumber }}</span>
        </header>
        <div class="calendar-day-items">
          <button
            v-for="item in day.items"
            :key="`${item.source_module}-${item.source_id}`"
            class="calendar-event"
            :class="`source-${item.source_module}`"
            type="button"
            @click.stop="selectItem(item)"
          >
            <span class="calendar-event-time">{{ item.scheduled_time || "Cały dzień" }}</span>
            <strong>{{ item.title }}</strong>
            <small>
              {{ sourceModuleLabel(item.source_module) }} ·
              {{ calendarItemTypeLabel(item.source_module, item.item_type) }}
              <template v-if="item.related_label"> · {{ item.related_label }}</template>
            </small>
          </button>
        </div>
      </article>
    </div>
  </section>
</template>
