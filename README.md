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
