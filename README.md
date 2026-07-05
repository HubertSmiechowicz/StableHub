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
- pierwszy pion modułu `horse`: `CreateHorse` i `ListHorses`,
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
- Ports & Adapters,
- CQRS,
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

## Przepływ danych

StableHub respektuje kierunek zależności typowy dla DDD, Ports & Adapters i CQRS:

```text
Vue
  -> Tauri command DTO
  -> application command/query
  -> use case / handler
  -> domain model
  -> repository port
  -> SQLite adapter
  -> SQLite
```

Zasady:

- frontend nie operuje bezpośrednio na encjach domenowych,
- Tauri commands są adapterem wejściowym i nie zawierają logiki biznesowej,
- `application` definiuje komendy, query, use case'y i porty,
- `domain` zawiera reguły domenowe i nie zna Tauri, SQLite ani DTO,
- `infrastructure` implementuje porty i zna szczegóły SQLx/SQLite,
- command zmienia stan,
- query czyta dane i zwraca DTO/read model dla UI.

## Moduł `horse`

Pierwsza implementacja modułu `horse` obejmuje podstawowy rejestr koni:

- utworzenie konia przez command `CreateHorse`,
- pobranie aktywnych koni przez query `ListHorses`,
- pobranie szczegółów konia przez query `GetHorseDetails`,
- zapis lokalny w SQLite,
- migrację tabeli `horses`,
- osobną zakładkę `Konie` w UI,
- przycisk `Dodaj konia`, który otwiera formularz w module koni,
- kliknięcie pozycji na liście koni, które otwiera osobny widok szczegółów.

Na tym etapie moduł zawiera tylko podstawowe dane profilu. Docelowo profil konia powinien zostać rozszerzony o dane spójne z paszportem konia, zdjęcia, skany paszportu i inne dokumenty, ale te elementy powinny być dodawane etapami, bez mieszania ich z pierwszym prostym pionem domenowym.

Dashboard nie służy do zarządzania encjami. Docelowo ma prezentować bieżące dane, powiadomienia, alerty magazynowe, przypomnienia i skróty do działań wymagających reakcji. Zarządzanie końmi odbywa się w module `Konie`.

Frontend również jest organizowany modułowo. `App.vue` pełni rolę lekkiego shella aplikacji i przełącza widoki, a właściwe ekrany znajdują się w modułach:

```text
src/modules/dashboard/views/DashboardView.vue

src/modules/horses/
  api/
  components/
  types/
  utils/
  views/
```

Moduł `horses` ma oddzielne widoki dla listy, dodawania i szczegółów konia:

```text
HorsesListView
HorseCreateView
HorseDetailsView
```

Aktualna tabela:

```sql
CREATE TABLE IF NOT EXISTS horses (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    sex TEXT NULL,
    breed TEXT NULL,
    date_of_birth TEXT NULL,
    coat_color TEXT NULL,
    identification_number TEXT NULL,
    notes TEXT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived_at TEXT NULL
);
```

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

Bieżące ustalenia i rzeczy do zrobienia są zbierane w `docs/TODO.md`.

## Repozytorium

Repozytorium GitHub: <https://github.com/HubertSmiechowicz/StableHub.git>
