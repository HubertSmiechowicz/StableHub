<script setup lang="ts">
import { ClipboardCheck, ClipboardList, PackagePlus, TimerReset } from "@lucide/vue";
import type { InventoryItemDetails } from "../types/inventory";
import {
  formatCurrency,
  formatDays,
  formatOptionalCurrency,
  formatOptionalQuantity,
  formatQuantity,
  unitLabel
} from "../utils/inventoryLabels";

defineProps<{
  item: InventoryItemDetails;
}>();

const emit = defineEmits<{
  edit: [];
  archive: [];
  applyUsage: [];
  registerDelivery: [];
  recordStocktake: [];
}>();
</script>

<template>
  <section class="panel entity-details-panel">
    <div class="panel-heading">
      <h2>Szczegóły pozycji</h2>
      <div class="panel-actions">
        <button class="secondary-action" type="button" @click="emit('registerDelivery')">
          <PackagePlus :size="17" aria-hidden="true" />
          Zarejestruj dostawę
        </button>
        <button class="secondary-action" type="button" @click="emit('applyUsage')">
          <TimerReset :size="17" aria-hidden="true" />
          Rozlicz zużycie
        </button>
        <button class="ghost-action" type="button" @click="emit('recordStocktake')">
          <ClipboardCheck :size="17" aria-hidden="true" />
          Inwentaryzacja
        </button>
        <button class="ghost-action" type="button" @click="emit('edit')">Edytuj</button>
        <button class="danger-action" type="button" @click="emit('archive')">Usuń</button>
        <ClipboardList :size="20" aria-hidden="true" />
      </div>
    </div>

    <div class="entity-details">
      <div class="entity-details-hero">
        <div>
          <p class="eyebrow">Stan magazynowy</p>
          <h3>{{ item.name }}</h3>
        </div>
        <span>{{ item.status }}</span>
      </div>

      <dl class="details-grid">
        <div>
          <dt>Aktualny stan</dt>
          <dd>{{ formatQuantity(item.quantity, item.unit) }}</dd>
        </div>
        <div>
          <dt>Jednostka</dt>
          <dd>{{ unitLabel(item.unit) }}</dd>
        </div>
        <div>
          <dt>Minimalny stan</dt>
          <dd>{{ formatOptionalQuantity(item.minimum_quantity, item.unit) }}</dd>
        </div>
        <div>
          <dt>Dzienne zużycie</dt>
          <dd>{{ formatOptionalQuantity(item.daily_usage, item.unit) }}</dd>
        </div>
        <div>
          <dt>Dni zapasu</dt>
          <dd>{{ formatDays(item.days_remaining) }}</dd>
        </div>
        <div>
          <dt>Do rozliczenia</dt>
          <dd>{{ item.pending_usage_days }} dni / {{ formatQuantity(item.pending_usage_quantity, item.unit) }}</dd>
        </div>
        <div>
          <dt>Koszt dostaw razem</dt>
          <dd>{{ formatCurrency(item.total_delivery_cost) }}</dd>
        </div>
        <div>
          <dt>Średni koszt jednostkowy</dt>
          <dd>{{ formatOptionalCurrency(item.average_unit_cost) }}</dd>
        </div>
        <div>
          <dt>Utworzono</dt>
          <dd>{{ item.created_at }}</dd>
        </div>
      </dl>

      <section class="delivery-history">
        <div class="subsection-heading">
          <div>
            <p class="eyebrow">Koszty i dostawy</p>
            <h3>Ostatnie dostawy</h3>
          </div>
        </div>

        <div v-if="item.recent_deliveries.length" class="delivery-list">
          <article
            v-for="delivery in item.recent_deliveries"
            :key="delivery.id"
            class="delivery-row"
          >
            <div>
              <strong>{{ delivery.delivered_on }}</strong>
              <p>
                {{ formatQuantity(delivery.quantity, item.unit) }}
                <span v-if="delivery.supplier">od {{ delivery.supplier }}</span>
              </p>
            </div>
            <div>
              <strong>{{ formatCurrency(delivery.total_cost) }}</strong>
              <p>{{ formatOptionalCurrency(delivery.unit_cost) }} / {{ unitLabel(item.unit) }}</p>
            </div>
          </article>
        </div>
        <p v-else class="muted">Brak zarejestrowanych dostaw dla tej pozycji.</p>
      </section>

      <section class="delivery-history">
        <div class="subsection-heading">
          <div>
            <p class="eyebrow">Kontrola stanu</p>
            <h3>Ostatnie inwentaryzacje</h3>
          </div>
        </div>

        <div v-if="item.recent_stocktakes.length" class="delivery-list">
          <article
            v-for="stocktake in item.recent_stocktakes"
            :key="stocktake.id"
            class="delivery-row"
          >
            <div>
              <strong>{{ stocktake.counted_on }}</strong>
              <p>
                Oczekiwano {{ formatQuantity(stocktake.expected_quantity, item.unit) }}
              </p>
            </div>
            <div>
              <strong>{{ formatQuantity(stocktake.actual_quantity, item.unit) }}</strong>
              <p>Różnica: {{ formatQuantity(stocktake.variance_quantity, item.unit) }}</p>
            </div>
          </article>
        </div>
        <p v-else class="muted">Brak zapisanych inwentaryzacji dla tej pozycji.</p>
      </section>
    </div>
  </section>
</template>
