# StableHub Docs

## Executive Summary

StableHub jest desktopową aplikacją offline-first przeznaczoną dla właścicieli prywatnych stajni oraz osób posiadających od 1 do 10 koni.

Celem produktu jest zastąpienie:
- Excela
- papierowych notatek
- kalendarzy
- rozproszonych dokumentów

jednym systemem do zarządzania:
- końmi
- zdrowiem
- kosztami
- magazynem
- obowiązkami

---

# Product Vision

## Problem

Większość właścicieli koni prowadzi dane w wielu miejscach:
- zeszyty
- Excel
- Google Calendar
- zdjęcia dokumentów
- notatki w telefonie

## Solution

StableHub ma być centralnym miejscem zarządzania prywatną stajnią.

---

# Design Philosophy

StableHub nie jest ERP-em.
StableHub nie jest systemem dla szkółek jeździeckich.
StableHub nie jest systemem dla dużych stajni sportowych.

Najważniejsze obszary:
- koszty
- zdrowie
- magazyn
- przypomnienia
- planowanie zimy

---

# Target Audience

- Prywatni właściciele koni
- Małe stajnie przydomowe
- Użytkownicy korzystający z Excela, zeszytów i kalendarzy

---

# Business Model

## StableHub Pro
- Licencja jednorazowa
- 149 PLN

## StableHub Cloud
- Synchronizacja urządzeń
- Backup w chmurze
- Aplikacja mobilna
- 99 PLN / rok

---

# MVP Scope

- Dashboard
- Konie
- Zdrowie
- Magazyn
- Finanse
- Kalendarz
- Raporty
- Backup

---

# UI & Screens

## Navigation

- Dashboard
- Konie
- Magazyn
- Finanse
- Kalendarz
- Zdrowie
- Raporty
- Ustawienia

## Dashboard

Przykładowe dane:
- Konie: 2
- Siano: 124 kostki (68 dni)
- Owies: 210 kg (41 dni)
- Koszty miesiąca: 1832 PLN
- Kowal za 4 dni
- Szczepienie za 21 dni

Widżety:
- Liczba koni
- Zapas siana
- Zapas owsa
- Koszty miesiąca
- Najbliższe wydarzenia
- Alerty magazynowe

## Konie

### Lista koni

Karta zawiera:
- Zdjęcie
- Imię
- Płeć
- Wiek
- Rasa

### Szczegóły konia

Zakładki:
- Informacje
- Zdrowie
- Finanse
- Dokumenty
- Notatki

## Zdrowie

Typy wpisów:
- Szczepienia
- Odrobaczenia
- Wizyty weterynaryjne
- Leczenie
- Urazy

## Magazyn

Towary:
- Siano
- Owies
- Słoma
- Pasze

Dla każdego produktu:
- Stan magazynowy
- Jednostka
- Średnie zużycie
- Dni zapasu

## Finanse

Kategorie:
- Pasza
- Siano
- Słoma
- Weterynarz
- Kowal
- Sprzęt
- Inne

Widoki:
- Lista wydatków
- Podsumowanie miesiąca
- Podsumowanie roku

## Kalendarz

Typy wydarzeń:
- Kowal
- Szczepienie
- Odrobaczenie
- Weterynarz
- Własne przypomnienia

## Raporty

- Koszty miesięczne
- Koszty roczne
- Zużycie siana
- Zużycie owsa

## Planowanie zimy

Obliczenia:
- Zużycie siana
- Zużycie owsa
- Koszt zimy
- Brakujące zapasy

---

# Domain Model

## Aggregates

- Horse
- Inventory
- Expense
- Reminder
- HealthRecord
- Stable

## Value Objects

- HorseId
- Money
- Weight
- Quantity
- DateRange

---

# Technical Architecture

## Stack

- Tauri 2
- Rust
- Vue 3
- TypeScript
- SQLite
- SQLx
- Pinia
- Vue Router
- Tailwind CSS
- shadcn-vue
- TanStack Query

## Architecture

- DDD
- Modular Monolith
- Clean Architecture

---

# Development Standards

- Brak logiki biznesowej w Vue
- Cała logika domenowa w Rust
- DTO nie są encjami domenowymi
- Use Case nie zna szczegółów bazy danych

---

# Internationalization

## Languages

MVP:
- Polski
- English

Future:
- German
- Dutch
- Swedish

## Currencies

- PLN
- EUR
- GBP
- USD

---

# Market Expansion

## Phase 1
- Polska

## Phase 2
- Niemcy
- Wielka Brytania
- Holandia
- Dania
- Szwecja
- Norwegia

## Phase 3
- USA
- Kanada
- Australia

---

# Killer Features

- Planowanie zimy
- Prognoza zużycia siana
- Analiza kosztów konia
- Historia zdrowia

---

# Roadmap

- V1 Desktop Offline First
- V2 Android + iOS
- V3 Cloud Sync
- V4 Shared Stable
- V5 Marketplace Extensions

---

# Success Metrics

- Zastępuje Excel
- Używany do prowadzenia prawdziwej stajni
- Posiada płacących klientów
- Pomaga finansować utrzymanie własnych koni
