# StableHub TODO

Ten plik zbiera ustalenia, ktore nie powinny zginac przed implementacja. README opisuje aktualny stan projektu, a TODO przechowuje najblizsze i przyszle prace.

## Najblizsze

- Rozbudowac encje/profil konia po pierwszym pionie `CreateHorse` + `ListHorses`.
- Dodac podpowiedzi ras koni w formularzu zamiast sztywnego slownika blokujacego uzytkownika.
- Zachowac pole rasy jako dowolny tekst na tym etapie, z podpowiedziami/autocomplete w UI.
- Pilnowac, aby frontend byl modulowy: `App.vue` jako shell, a widoki i komponenty w `src/modules/...`.

## Modul `horse`

### Profil konia

Docelowo profil konia powinien byc bardziej szczegolowy i spojny z danymi z paszportu konia.

Do rozwazenia:

- imie,
- plec,
- rasa,
- data urodzenia,
- masc,
- numer identyfikacyjny,
- numer paszportu,
- numer transpondera/chipu,
- kraj urodzenia,
- hodowca,
- wlasciciel,
- znaki szczegolne/opis,
- notatki profilowe.

### Dokumenty konia

Docelowo modul powinien umozliwiac dodawanie dokumentow powiazanych z koniem.

Do rozwazenia:

- zdjecie profilowe konia,
- dodatkowe zdjecia,
- skan paszportu,
- dokumenty weterynaryjne,
- inne zalaczniki.

Na razie nie implementowac tego w pierwszym prostym modelu `horses`. Dokumenty powinny wejsc jako osobny krok, prawdopodobnie z osobnym modelem plikow/zalacznikow.

### Rasy koni

Ustalenie:

- rasy powinny miec podpowiedzi w UI,
- uzytkownik nadal moze wpisac wlasna wartosc,
- na razie nie robimy twardego enumu ras w domenie,
- na razie nie robimy tabeli `horse_breeds`,
- w przyszlosci mozna rozwazyc modul danych referencyjnych/slownikow.

Przykladowe podpowiedzi na start:

- Arabski,
- Pelnej krwi angielskiej,
- Malopolski,
- Wielkopolski,
- Slaski,
- Huculski,
- Konik polski,
- Fryzyjski,
- Hanowerski,
- Holsztynski,
- Quarter Horse,
- Appaloosa,
- Kuc walijski,
- Szetlandzki.

## Architektura

- Pilnowac przeplywu DDD: `interfaces -> application -> domain`, a `infrastructure` implementuje porty.
- Nie przepuszczac encji domenowych bezposrednio do frontendu.
- Nie mieszac SQLx ani SQLite z warstwa `application` lub `domain`.
- Rozwijac CQRS przez osobne commands i queries.
- Aktualizowac README po kazdej istotnej decyzji architektonicznej.
