# StableHub
## Product Vision & Technical Specification
Version 1.0

# Executive Summary

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

# Business Model

## StableHub Pro
- Jednorazowa licencja
- Cena: 149 PLN

## StableHub Cloud
- Synchronizacja
- Backup w chmurze
- Mobile
- 99 PLN / rok

# MVP Scope

- Dashboard
- Konie
- Zdrowie
- Magazyn
- Finanse
- Kalendarz
- Raporty
- Backup

# Dashboard

Przykład:

🐴 Konie: 2
🌾 Siano: 124 kostki (68 dni)
🥣 Owies: 210 kg (41 dni)
💰 Koszty: 1832 zł
📅 Kowal za 4 dni

# Frontend Architecture

## UI Framework
shadcn-vue

## Frontend Stack
- Vue 3
- TypeScript
- Vue Router
- Pinia
- shadcn-vue
- Tailwind CSS
- TanStack Query

## UI Design System
Domyślny motyw: Dark Mode

Background: #09090b
Card: #18181b
Border: #27272a
Accent: Emerald

# Technical Architecture

## Stack
- Tauri 2
- Rust
- Vue 3
- TypeScript
- SQLite
- SQLx

## Architecture
- DDD
- Modular Monolith
- Clean Architecture

# DDD

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

# Internationalization Strategy

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

# Market Expansion

Faza 1:
- Polska

Faza 2:
- Niemcy
- Wielka Brytania
- Holandia
- Dania
- Szwecja
- Norwegia

Faza 3:
- USA
- Kanada
- Australia

# Roadmap

V1 Desktop Offline First
V2 Android + iOS
V3 Cloud Sync
V4 Shared Stable
V5 Marketplace Extensions

# Success Metrics

- zastępuje Excel
- używany do prowadzenia prawdziwej stajni
- posiada płacących klientów
- pomaga finansować utrzymanie własnych koni
