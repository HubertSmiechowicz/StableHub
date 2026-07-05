# StableHub

StableHub to desktopowa aplikacja offline-first dla prywatnych właścicieli małych stajni oraz osób posiadających od 1 do 10 koni.

Celem projektu jest zastąpienie Excela, papierowych notatek, kalendarzy i rozproszonych dokumentów jednym lokalnym systemem do zarządzania końmi, zdrowiem, kosztami, magazynem i obowiązkami.

## Status projektu

Projekt jest na etapie inicjalizacji technicznej.

Obecnie istnieje:

- szkielet aplikacji desktopowej Tauri 2,
- frontend Vue 3 + TypeScript + Vite,
- ekran powitalny StableHub,
- przygotowana struktura backendu pod DDD i Modular Monolith,
- przygotowane zależności pod lokalną bazę SQLite przez SQLx,
- podstawowa konfiguracja buildów desktopowych.

## Założenia produktu

StableHub nie jest ERP-em ani systemem dla dużych ośrodków jeździeckich. Produkt ma być prostym, praktycznym narzędziem dla właściciela prywatnej stajni.

Najważniejsze obszary MVP:

- Dashboard,
- Konie,
- Zdrowie,
- Magazyn,
- Finanse,
- Kalendarz,
- Raporty,
- Backup.

Kluczowe funkcje docelowe:

- planowanie zimy,
- prognoza zużycia siana i paszy,
- analiza kosztów konia,
- historia zdrowia,
- lokalne działanie bez internetu.

## Offline-first i SQLite

StableHub ma działać lokalnie jako aplikacja desktopowa. SQLite nie będzie osobną usługą instalowaną przez użytkownika. Aplikacja będzie korzystać z lokalnego pliku bazy danych tworzonego przy pierwszym uruchomieniu, np. w katalogu danych aplikacji użytkownika.

Planowany model:

```text
StableHub.exe
  -> lokalny plik stablehub.sqlite
  -> migracje SQL uruchamiane przez aplikację
  -> wszystkie podstawowe dane dostępne offline
```

W przyszłości synchronizacja lub backup w chmurze powinny być dodatkiem do lokalnego trybu pracy, a nie wymaganiem do działania aplikacji.

## Stack technologiczny

Frontend:

- Vue 3,
- TypeScript,
- Vite,
- lucide icons.

Desktop/backend:

- Tauri 2,
- Rust,
- SQLite,
- SQLx.

Architektura:

- Domain Driven Design,
- Modular Monolith,
- moduły jako główne granice odpowiedzialności,
- warstwy wewnątrz modułów.

## Struktura backendu

Backend jest organizowany module-first. Każdy moduł ma własne warstwy, zamiast globalnego podziału typu `domain/`, `application/`, `infrastructure/`.

Przykład:

```text
src-tauri/src/modules/horse/
  application/
  domain/
  infrastructure/
    persistence/
      sqlite/
        migrations/
        repositories/
  interfaces/
    tauri_commands/
```

Aktualne moduły:

- `horse`,
- `inventory`,
- `finance`,
- `health`,
- `calendar`,
- `stable`.

Katalog `src-tauri/src/platform/sqlite` jest miejscem na przyszłą techniczną obsługę SQLite: ścieżkę pliku bazy, connection pool i uruchamianie migracji.

## Uruchamianie projektu

Instalacja zależności:

```powershell
npm install
```

Podgląd frontendu w przeglądarce:

```powershell
npm run dev
```

Podgląd jako aplikacja desktopowa Tauri:

```powershell
npm run tauri dev
```

Build frontendu:

```powershell
npm run build
```

Build aplikacji desktopowej i instalatorów:

```powershell
npm run tauri build
```

Wyniki builda Tauri znajdują się w:

```text
src-tauri/target/release/
src-tauri/target/release/bundle/
```

## Dokumentacja

Dokumenty produktowe znajdują się w katalogu `docs/`.

README powinno być aktualizowane systematycznie razem ze zmianami architektury, sposobu uruchamiania, modułów i decyzji projektowych. Ten plik ma być bieżącą dokumentacją projektu, a nie tylko opisem startowym.

## Repozytorium

Projekt jest przygotowywany do pracy w Git i późniejszego podpięcia pod GitHub.