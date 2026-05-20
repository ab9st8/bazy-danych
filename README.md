# Bazy danych / Programowanie w języku Rust

**semestr letni 2025/2026**

**Autorzy:** Stanisław Ogonowski, Antoni Chmiela

---

Nasz projekt to serwer obsługujący rezerwację biletów na wydarzenia socjalne w czasie rzeczywistym. Wspiera on przepełnienie popytu na dane wydarzenie poprzez kolejkę ("waitlistę").

Więcej szczegółów na temat specyfiki projektu można znaleźć w [architecture decision record](./adr.md).

Wymagania:
- Rust + Cargo
- Docker + Docker Compose

Uruchomienie:

```shell
docker compose up
```

Powyższą komendą uruchomione zostaną kontenery dla zarówno serwera (`backend`) jak i bazy danych Postgres (`db`).

## Generator obciążenia

W repozytorium znajduje się crate `loadgen` — narzędzie CLI generujące syntetyczny ruch HTTP do backendu. Uruchomienie (z poziomu repozytorium, gdy backend działa):

```shell
cargo run -p loadgen
```

Domyślnie wykonuje 60 rezerwacji na minutę przez 60 sekund w cztery wydarzenia z seeda; po każdej udanej rezerwacji (`201`) natychmiast próbuje opłacić bilet. Każde wywołanie HTTP loguje osobną linię, a co 5 sekund pojawia się migawka stanu (`snapshot`); po zakończeniu — podsumowanie (`summary`).

Flagi konfiguracyjne:

- `--base-url <URL>` — adres backendu (domyślnie `http://localhost:3000`)
- `--rpm <N>` — docelowa liczba rezerwacji na minutę (domyślnie `60`)
- `--concurrency <N>` — maksymalna liczba jednocześnie wykonywanych żądań (domyślnie `32`)
- `--duration <czas>` — jak długo generator ma działać, np. `30s`, `2m` (domyślnie `60s`)
- `--event <UUID>` — wydarzenie docelowe; flaga powtarzalna (domyślnie cztery UUID-y z seeda)
- `--snapshot-interval <czas>` — częstotliwość migawek stanu (domyślnie `5s`)
