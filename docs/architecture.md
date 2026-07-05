# StableHub Architecture

Ten dokument opisuje zasady techniczne, ktore maja utrzymac spojna architekture StableHub w kolejnych iteracjach.

## Fundament

StableHub jest desktopowa aplikacja offline-first.

Glowne zalozenia:

- aplikacja dziala lokalnie bez internetu,
- Tauri 2 uruchamia desktopowa powloke,
- Rust jest miejscem logiki aplikacyjnej i domenowej,
- Vue jest warstwa prezentacji,
- SQLite jest lokalnym plikiem bazy danych, a nie osobna usluga,
- cloud sync lub backup moga pojawic sie pozniej jako dodatek.

## Styl architektury

Projekt laczy:

- Domain Driven Design,
- Modular Monolith,
- Ports & Adapters,
- CQRS.

Moduly sa pierwszym poziomem podzialu. Warstwy istnieja wewnatrz modulow.

Przyklad:

```text
src-tauri/src/modules/horse/
  domain/
  application/
  infrastructure/
  interfaces/
```

Nie stosujemy globalnego podzialu:

```text
domain/
application/
infrastructure/
```

## Kierunek zaleznosci

Dozwolony przeplyw:

```text
interfaces -> application -> domain
infrastructure -> application/domain
```

Przeplyw danych:

```text
Vue
  -> Tauri command DTO
  -> application command/query
  -> domain model, jesli operacja wymaga reguly domenowej
  -> repository port
  -> SQLite adapter
  -> SQLite
```

## Domena

Domena ma byc czysta i mala.

Zasady:

- domena nie zna Tauri,
- domena nie zna Vue,
- domena nie zna SQLx ani SQLite,
- domena nie zna DTO,
- domena nie musi pokrywac sie z tabela bazy danych,
- domena nie musi pokrywac sie z modelem UI,
- pole trafia do domeny tylko wtedy, gdy bierze udzial w regule lub operacji domenowej,
- dane tylko informacyjne powinny pozostac w DTO, read modelu albo persistence modelu.

Przyklad dla modulu `horse`:

```text
Horse
  id
  name
  status
```

Pola takie jak rasa, masc, notatki, data utworzenia i identyfikatory dokumentow moga istniec w bazie i UI, ale nie musza byc polami encji domenowej, dopoki nie uczestnicza w regule domenowej.

Relacje miedzy modulami powinny byc utrzymywane przez identyfikatory i przypadki uzycia, a nie przez mieszanie domen. Przyklad: `health` przechowuje `horse_id` zdarzenia zdrowotnego, ale nie wlacza encji `Horse` do swojej domeny.

Modul moze agregowac read modele z innych modulow, jesli jego odpowiedzialnosc tego wymaga. Przyklad: `calendar` pokazuje w terminarzu wlasne wpisy oraz aktywne zdarzenia z `health`, ale nie przejmuje ich wlascicielstwa. Wpis zdrowotny nadal jest edytowany w module `health`.

## Application

Warstwa application zawiera:

- commands,
- queries,
- handlers/use cases,
- ports,
- DTO/read modele uzywane przez przypadki uzycia.

Warstwa application:

- orkiestruje przypadki uzycia,
- definiuje porty,
- nie zna SQLx,
- nie wykonuje zapytan SQL,
- nie zawiera logiki UI.

## CQRS

Commands zmieniaja stan.

Queries czytaja stan i zwracaja read modele dla UI.

Nie laczymy zapisu i odczytu w jednym serwisie typu "manager do wszystkiego".

Przyklad:

```text
CreateHorseCommand
CreateHorseHandler

ListHorsesQuery
ListHorsesHandler

GetHorseDetailsQuery
GetHorseDetailsHandler
```

Usuwanie encji w podstawowych modulach jest soft delete:

- command ustawia status archiwalny i `archived_at`,
- domyslne listy zwracaja tylko aktywne rekordy,
- dane pozostaja w lokalnej bazie SQLite do celow historii, audytu i przyszlych widokow archiwum.

## Ports & Adapters

Porty sa definiowane w application.

Adaptery sa implementowane w infrastructure.

Przyklad:

```text
application/ports/HorseRepository
infrastructure/persistence/sqlite/SqliteHorseRepository
```

Tauri commands sa adapterami wejsciowymi i naleza do:

```text
interfaces/tauri_commands
```

## Infrastructure

Infrastructure zna szczegoly techniczne:

- SQLx,
- SQLite,
- migracje,
- mapowanie row <-> DTO/read model,
- adaptery portow.

Infrastructure moze mapowac dane z bazy do read modelu bez budowania pelnej encji domenowej, jesli query nie wymaga reguly domenowej.

## SQLite

SQLite jest lokalnym plikiem bazy danych tworzonym przez aplikacje.

Nie instalujemy osobnej instancji SQLite.

Techniczna obsluga SQLite jest w:

```text
src-tauri/src/platform/sqlite
```

Migracje SQLite sa trzymane centralnie:

```text
src-tauri/src/platform/sqlite/migrations
```

Powod: SQLx zapisuje migracje w jednej globalnej tabeli `_sqlx_migrations` dla jednego pliku SQLite. Przy wielu osobnych migratorach modulowych latwo doprowadzic do konfliktu numerow albo bledu brakujacej migracji. Dlatego pliki migracji maja jedna globalna numeracje:

```text
0001_create_horses.sql
0002_create_inventory_items.sql
```

Moduly nadal posiadaja swoje adaptery persistence, ale globalny schemat bazy jest migrowany przez jeden migrator platformowy.

## Frontend

Frontend tez powinien byc modulowy.

`App.vue` jest lekkim shellem:

- sidebar,
- topbar,
- wybor aktywnego widoku.

Widoki i komponenty naleza do modulow:

```text
src/modules/horses/
  api/
  components/
  types/
  utils/
  views/
```

Lista, tworzenie i szczegoly powinny byc osobnymi widokami, nie jednym duzym ekranem.

Ten sam uklad obowiazuje dla kolejnych modulow frontendu, np.:

```text
src/modules/inventory/
  api/
  components/
  types/
  utils/
  views/
```

## Jezyk aplikacji

Domyslnym jezykiem systemu jest polski.

Zasady:

- komunikaty walidacyjne sa po polsku,
- ostrzezenia i bledy widoczne dla uzytkownika sa po polsku,
- fallbacki dla nieoczekiwanych bledow w UI sa po polsku,
- teksty techniczne z bibliotek moga zostac dolaczone jako szczegol bledu, ale komunikat aplikacji powinien miec polski kontekst,
- pelna wielojezycznosc/i18n zostanie dodana dopiero wtedy, gdy faktycznie pojawi sie potrzeba innych lokalizacji.

## Dokumentacja

README opisuje aktualny stan projektu.

`docs/TODO.md` zbiera przyszle prace.

`docs/architecture.md` jest zrodlem zasad architektonicznych.

Przy istotnej zmianie architektury nalezy zaktualizowac dokumentacje razem z kodem.
