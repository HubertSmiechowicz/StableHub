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
- Trzymac zasady architektury w `docs/architecture.md` jako zrodle prawdy.

## Modul `health`

### Pierwszy pion

- [x] tworzenie zdarzenia zdrowotnego,
- [x] lista zdarzen zdrowotnych,
- [x] szczegoly zdarzenia zdrowotnego,
- [x] edycja zdarzenia zdrowotnego,
- [x] soft delete zdarzenia zdrowotnego,
- [x] filtrowanie po koniu, typie i frazie tekstowej,
- [x] sortowanie listy zdarzen,
- [x] przypisanie zdarzenia do aktywnego konia.

### Do zaimplementowania pozniej

- przypomnienia o szczepieniach i odrobaczaniu,
- cykliczne harmonogramy zdrowotne,
- dokumenty i skany powiazane ze zdarzeniem,
- powiazanie kosztow z modulem `finance`,
- alerty zdrowotne na dashboardzie,
- dedykowany widok historii zdrowia konkretnego konia,
- automatyczne propozycje kolejnych terminow na podstawie typu zdarzenia.
## Modul `inventory`

### Pierwszy pion

Ustalenie: modul magazynu ma od razu zarzadzac stanem, a nie byc tylko katalogiem pozycji.

Pierwszy zakres:

- [x] tworzenie pozycji magazynowej,
- [x] lista pozycji magazynowych,
- [x] szczegoly pozycji magazynowej,
- [x] aktualny stan ilosciowy,
- [x] jednostka miary,
- [x] minimalny stan,
- [x] srednie dzienne zuzycie,
- [x] wyliczenie liczby dni zapasu w read modelu.
- [x] blokada duplikatow aktywnych pozycji o tej samej nazwie i jednostce.
- [x] edycja pozycji magazynowej.
- [x] soft delete pozycji magazynowej.
- [x] filtrowanie i sortowanie listy pozycji magazynowych.

### Do zaimplementowania pozniej

- inwentaryzacja/korekta stanu magazynowego,
- historia ruchow magazynowych,
- przyjecie dostawy,
- rejestracja zuzycia,
- powiadomienia o niskim stanie magazynowym,
- alerty na dashboardzie,
- konfiguracja progow ostrzezen,
- raport zuzycia siana, owsa i innych pozycji,
- prognozowanie zuzycia siana, paszy i owsa na podstawie danych historycznych,
- prognozowanie liczby dni zapasu na podstawie aktualnej liczebnosci stada,
- planowanie zapasow na zime na podstawie historii zuzycia i liczby koni,
- powiazanie kosztow zakupow z modulem `finance` bez mieszania odpowiedzialnosci modulow.

Zasada domenowa: jesli operacja magazynowa dotyczy ilosci i moze naruszyc reguly, np. zejscie ponizej zera, to stan magazynowy powinien byc elementem domeny, a nie tylko polem DTO lub tabeli.
