# Oracle PL/Sql

widoki, funkcje, procedury, triggery

ćwiczenie 2

(kontynuacja ćwiczenia 1)

---

Imiona i nazwiska autorów :

---

<style>
  {
    font-size: 16pt;
  }
</style>

<style scoped>
 li, p {
    font-size: 14pt;
  }
</style>

<style scoped>
 pre {
    font-size: 10pt;
  }
</style>

# Zadanie 6

Zmiana struktury bazy danych. W tabeli `trip` należy dodać redundantne pole `no_available_places`. Dodanie redundantnego pola uprości kontrolę dostępnych miejsc (sprawdzenie liczby dostępnych miejsc), ale nieco skomplikuje procedury dodawania rezerwacji, zmiany statusu czy też zmiany maksymalnej liczby miejsc na wycieczki (potrzebna będzie dodatkowa aktualizacja w tabeli `trip`).

Należy przygotować polecenie/procedurę przeliczającą wartość pola `no_available_places` dla wszystkich wycieczek (do jednorazowego wykonania)

Obsługę pola `no_available_places` można zrealizować przy pomocy procedur lub triggerów

Należy zwrócić uwagę na spójność rozwiązania.

> UWAGA
> Należy stworzyć nowe wersje tych widoków/procedur/triggerów (np. dodając do nazwy dopisek 6 - od numeru zadania). Poprzednie wersje procedur należy pozostawić w celu umożliwienia weryfikacji ich poprawności.

- zmiana struktury tabeli

```sql
alter table trip add
    no_available_places int null
```

- polecenie przeliczające wartość `no_available_places`
  - należy wykonać operację "przeliczenia" liczby wolnych miejsc i aktualizacji pola `no_available_places`

# Zadanie 6 - rozwiązanie

```sql

UPDATE trip t
SET no_available_places = max_no_places - (
    SELECT COUNT(*) 
    FROM reservation r 
    WHERE r.trip_id = t.trip_id AND r.status IN ('N', 'P')
);

COMMIT;

```

---

# Zadanie 6a - procedury

Obsługę pola `no_available_places` należy zrealizować przy pomocy procedur

- procedura dodająca rezerwację powinna aktualizować pole `no_available_places` w tabeli trip
- podobnie procedury odpowiedzialne za zmianę statusu oraz zmianę maksymalnej liczby miejsc na wycieczkę
- należy przygotować procedury oraz jeśli jest to potrzebne, zaktualizować triggery oraz widoki

> UWAGA
> Należy stworzyć nowe wersje tych widoków/procedur/triggerów (np. dodając do nazwy dopisek 6a - od numeru zadania). Poprzednie wersje procedur należy pozostawić w celu umożliwienia weryfikacji ich poprawności.

- może być potrzebne wyłączenie 'poprzednich wersji' triggerów

# Zadanie 6a - rozwiązanie

```sql

ALTER TRIGGER trg_check_available_places_5 DISABLE;

-- p_add_reservation_6a
CREATE OR REPLACE PROCEDURE p_add_reservation_6a(p_trip_id INT, p_person_id INT)
AS
    v_available_places INT;
    v_trip_date DATE;
BEGIN
    SELECT no_available_places, trip_date INTO v_available_places, v_trip_date
    FROM trip
    WHERE trip_id = p_trip_id;

    IF v_trip_date <= SYSDATE THEN
        RAISE_APPLICATION_ERROR(-20010, 'Wycieczka już się odbyła lub jest dzisiaj.');
    END IF;

    IF v_available_places <= 0 THEN
        RAISE_APPLICATION_ERROR(-20011, 'Brak wolnych miejsc na tę wycieczkę.');
    END IF;

    INSERT INTO reservation(trip_id, person_id, status)
    VALUES (p_trip_id, p_person_id, 'N');

    UPDATE trip
    SET no_available_places = no_available_places - 1
    WHERE trip_id = p_trip_id;
END;
/

-- p_modify_reservation_status_6a
CREATE OR REPLACE PROCEDURE p_modify_reservation_status_6a(p_reservation_id INT, p_status CHAR)
AS
    v_current_status CHAR(1);
    v_trip_id INT;
    v_available_places INT;
BEGIN
    SELECT status, trip_id INTO v_current_status, v_trip_id
    FROM reservation
    WHERE reservation_id = p_reservation_id;

    IF v_current_status = 'C' AND p_status IN ('N', 'P') THEN
        SELECT no_available_places INTO v_available_places
        FROM trip
        WHERE trip_id = v_trip_id;
        
        IF v_available_places <= 0 THEN
            RAISE_APPLICATION_ERROR(-20012, 'Brak wolnych miejsc, aby przywrócić rezerwację.');
        END IF;

        UPDATE trip
        SET no_available_places = no_available_places - 1
        WHERE trip_id = v_trip_id;

    ELSIF v_current_status IN ('N', 'P') AND p_status = 'C' THEN
        UPDATE trip
        SET no_available_places = no_available_places + 1
        WHERE trip_id = v_trip_id;
    END IF;

    UPDATE reservation
    SET status = p_status
    WHERE reservation_id = p_reservation_id;
END;
/

-- p_modify_max_no_places_6a
CREATE OR REPLACE PROCEDURE p_modify_max_no_places_6a(p_trip_id INT, p_max_no_places INT)
AS
    v_reserved_places INT;
BEGIN
    SELECT (max_no_places - no_available_places) INTO v_reserved_places
    FROM trip
    WHERE trip_id = p_trip_id;

    IF p_max_no_places < v_reserved_places THEN
        RAISE_APPLICATION_ERROR(-20013, 'Liczba wprowadzonych miejsc nie może być mniejsza od zarezerwowanych.');
    END IF;

    UPDATE trip
    SET 
        max_no_places = p_max_no_places,
        no_available_places = p_max_no_places - v_reserved_places
    WHERE trip_id = p_trip_id;
END;
/

-- vw_trip_6a
CREATE OR REPLACE VIEW vw_trip_6a AS
SELECT
    trip_id,
    country,
    trip_date,
    trip_name,
    max_no_places,
    no_available_places
FROM trip;

-- vw_available_trip_6a
CREATE OR REPLACE VIEW vw_available_trip_6a AS
SELECT *
FROM vw_trip_6a
WHERE trip_date > SYSDATE
  AND no_available_places > 0;

```

---

# Zadanie 6b - triggery

Obsługę pola `no_available_places` należy zrealizować przy pomocy triggerów

- podczas dodawania rezerwacji trigger powinien aktualizować pole `no_available_places` w tabeli trip
- podobnie, podczas zmiany statusu rezerwacji
- należy przygotować trigger/triggery oraz jeśli jest to potrzebne, zaktualizować procedury modyfikujące dane oraz widoki

> UWAGA
> Należy stworzyć nowe wersje tych widoków/procedur/triggerów (np. dodając do nazwy dopisek 6b - od numeru zadania). Poprzednie wersje procedur należy pozostawić w celu umożliwienia weryfikacji ich poprawności.

- może być potrzebne wyłączenie 'poprzednich wersji' triggerów

# Zadanie 6b - rozwiązanie

```sql

-- Disable previous trigger if not disabled
ALTER TRIGGER trg_check_available_places_5 DISABLE;

-- trg_check_available_places_6b
CREATE OR REPLACE TRIGGER trg_check_available_places_6b
BEFORE INSERT OR UPDATE OF status ON reservation
FOR EACH ROW
DECLARE
    v_available_places INT;
    v_trip_date DATE;
BEGIN
    IF INSERTING OR (UPDATING AND :OLD.status = 'C' AND :NEW.status IN ('N', 'P')) THEN
        SELECT no_available_places, trip_date INTO v_available_places, v_trip_date
        FROM trip
        WHERE trip_id = :NEW.trip_id;

        IF INSERTING AND v_trip_date <= SYSDATE THEN
            RAISE_APPLICATION_ERROR(-20010, 'Wycieczka już się odbyła lub jest dzisiaj.');
        END IF;

        IF v_available_places <= 0 THEN
            RAISE_APPLICATION_ERROR(-20011, 'Brak wolnych miejsc na tę wycieczkę.');
        END IF;
    END IF;
END;
/

-- trg_update_places_reservation_6b
CREATE OR REPLACE TRIGGER trg_update_places_reservation_6b
AFTER INSERT OR UPDATE OF status ON reservation
FOR EACH ROW
BEGIN
    IF INSERTING THEN
        IF :NEW.status IN ('N', 'P') THEN
            UPDATE trip
            SET no_available_places = no_available_places - 1
            WHERE trip_id = :NEW.trip_id;
        END IF;
    ELSIF UPDATING THEN
        IF :OLD.status = 'C' AND :NEW.status IN ('N', 'P') THEN
            UPDATE trip
            SET no_available_places = no_available_places - 1
            WHERE trip_id = :NEW.trip_id;
        ELSIF :OLD.status IN ('N', 'P') AND :NEW.status = 'C' THEN
            UPDATE trip
            SET no_available_places = no_available_places + 1
            WHERE trip_id = :NEW.trip_id;
        END IF;
    END IF;
END;
/

-- trg_trip_max_places_mod_6b
CREATE OR REPLACE TRIGGER trg_trip_max_places_mod_6b
BEFORE UPDATE OF max_no_places ON trip
FOR EACH ROW
BEGIN
    :NEW.no_available_places := :NEW.max_no_places - (:OLD.max_no_places - :OLD.no_available_places);
    IF :NEW.no_available_places < 0 THEN
        RAISE_APPLICATION_ERROR(-20013, 'Liczba wprowadzonych miejsc nie może być mniejsza od zarezerwowanych.');
    END IF;
END;
/

-- p_add_reservation_6b
CREATE OR REPLACE PROCEDURE p_add_reservation_6b(p_trip_id INT, p_person_id INT)
AS
BEGIN
    INSERT INTO reservation(trip_id, person_id, status)
    VALUES (p_trip_id, p_person_id, 'N');
END;
/

-- p_modify_reservation_status_6b
CREATE OR REPLACE PROCEDURE p_modify_reservation_status_6b(p_reservation_id INT, p_status CHAR)
AS
BEGIN
    UPDATE reservation
    SET status = p_status
    WHERE reservation_id = p_reservation_id;
END;
/

-- p_modify_max_no_places_6b
CREATE OR REPLACE PROCEDURE p_modify_max_no_places_6b(p_trip_id INT, p_max_no_places INT)
AS
BEGIN
    UPDATE trip
    SET max_no_places = p_max_no_places
    WHERE trip_id = p_trip_id;
END;
/

```
