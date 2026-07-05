# StableHub - Założenia Produktowe i Techniczne

## Wizja produktu
StableHub to desktopowa aplikacja offline-first przeznaczona dla właścicieli 1–10 koni utrzymywanych prywatnie.

## Model biznesowy
- Licencja jednorazowa: 149 PLN
- W przyszłości: StableHub Cloud (synchronizacja, backup, mobile)

## Grupa docelowa
- 1–10 koni
- prywatna stajnia
- nacisk na koszty, zapasy i zdrowie

## Stack technologiczny
- Tauri 2
- Rust
- Vue 3
- TypeScript
- SQLite
- SQLx
- Pinia

## Architektura
DDD + Modular Monolith + Clean Architecture

### Domeny
- Horse
- Inventory
- Finance
- Health
- Calendar
- Stable

## Dashboard

```text
🐴 Konie: 2

🌾 Siano:
124 kostki
68 dni zapasu

🥣 Owies:
210 kg
41 dni zapasu

💰 Koszty czerwca:
1832 zł

📅 Najbliższe wydarzenia:
• Kowal za 4 dni
• Szczepienie za 21 dni
```

## Nawigacja

```text
🏠 Dashboard
🐴 Konie
🌾 Magazyn
💰 Finanse
📅 Kalendarz
📋 Zdrowie
📊 Raporty
⚙️ Ustawienia
```

## Widok koni

```text
┌─────────────────────┐
│     Zdjęcie         │
│ Hades               │
│ Wałach • 8 lat      │
│ Małopolski          │
└─────────────────────┘
```

## Magazyn

```text
Siano
█████████░░░
124 kostki
68 dni zapasu
```

## Finanse

```text
Koszt miesięczny
1832 zł
```

## Planowanie zimy

```text
🐴 2 konie
📅 Listopad - Marzec
🌾 312 kostek siana
💰 5840 zł
```

## Główna przewaga
Osobisty menedżer właściciela koni skupiony na:
- kosztach
- zapasach
- zdrowiu
- przypomnieniach
- planowaniu zimy
