
# Dokumentowe bazy danych – MongoDB

Ćwiczenie 1


---

**Imiona i nazwiska autorów:**

--- 

# Test połączenia

- informacja o wersji

```js
db.version()
```

- wynik

```js
{  
  "result": "7.0.7"  
}
```

---

- lista baz danych na serwerze

```js
show dbs;    
show databases; 
db.adminCommand('listDatabases');
```

- wynik

```js
[  
  {  
    "empty": false,  
    "name": "admin",  
    "sizeOnDisk": 40960  
  },  
  {  
    "empty": false,  
    "name": "config",  
    "sizeOnDisk": 98304  
  },   
  {  
    "empty": false,  
    "name": "local",  
    "sizeOnDisk": 40960  
  }  
]
```

---

- wybór bazy danych
	- baza o nazwie univ

```js
use univ;
```

- informacja o bieżącej bazie danych

```js
db;
```

- informacja o kolekcjach

```js
show collections;  
db.getCollectionNames();
```

---
# Proste operacje/zapytania 

- MongoDB simple query
- [https://www.mongodb.com/docs/manual/tutorial/query-documents/](https://www.mongodb.com/docs/manual/tutorial/query-documents/)


---
# Przykład 1

proste operacje na dokumentach

- wstaw/stwórz pierwszy dokument
	- w tym momencie tworzona jest baza danych i kolekcja (jeśli wcześniej nie istniały)

```js
db.student.insertOne(
    {  
        "student_id": 1,  
        "firstname": "John",  
        "lastname": "Gold",  
        "age": 25  
    }  
);
```

- sprawdź listę baz i kolekcji

```js
show databases;
```


- wynik

```js
[  
  {  
    "empty": false,  
    "name": "admin",  
    "sizeOnDisk": 40960  
  },  
  {  
    "empty": false,  
    "name": "config",  
    "sizeOnDisk": 98304  
  },   
  {  
    "empty": false,  
    "name": "local",  
    "sizeOnDisk": 40960  
  },  
  {  
    "empty": false,  
    "name": "univ",  
    "sizeOnDisk": 8192  
  }  
]
```


```ls
show collections;
```


- wynik

```js
[  
  {  
    "badge": "",  
    "name": "student"  
  }  
]
```


- wyszukaj dokumenty w kolekcji ` employees `

```js
db.student.find();  
db.student.find({});
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67e86e45b1c69e3d7e42c9c0"},  
    "age": 25,  
    "firstname": "John",  
    "lastname": "Gold",  
    "student_id": 1  
  }  
]
```


- stwórz indeks zapewniający unikalność atrybutu ` "student_id"  `  

```js
// tworzenie indeksu
db.student.createIndex({ "student_id" : 1 }, { "unique": true });  

// inf. o indeksach w kolekcji student
db.student.getIndexes();

// usunięcie indeksu o nazwie 
db.student.dropIndexes("student_id_1");
```


- wstaw kilka dokumentów do kolekcji  ` student `
	- np

```js
db.student.insertOne(  
    {  
        "student_id": 2,  
        "firstname": "James",  
        "lastname": "Bond"  
    }  
);


db.student.insertOne(  
    {  
        "firstname": "John",  
        "lastname": "Bond"  
    }  
);
```

- wyszukaj dokumenty w kolekcji  ` student `

```js
db.student.find();  
db.student.find({});  
db.student.find({"firstname": "John"});  
db.student.find({"student_id": 1}); 

db.student.find({"_id": ObjectId("67e669647013d10c86e71c87")});
```

- zmodyfikuj wybrane dokumenty
	- po wykonaniu każdego z przykładów sprawdź wynik za pomocą  ` db.student.find() `

- np

```js
db.student.updateOne(  
    {"student_id" : 1},  
    {  
        $set: {  
            "firstname": "Adam",  
            "lastname": "Silver"  
        }  
    }  
)
```


```js
db.student.updateOne(  
    {"student_id" : 1},  
    {  
        $inc: {"age": 5}  
    }  
)
```


```js
db.student.updateOne(  
    {"student_id" : 1},  
    {  
        $set: {  
            "firstname": "John",  
            "lastname": "Gold"  
        },  
        $inc: {  
            "age": -2  
        }  
    }  
)
```


```js
db.student.updateOne(  
    {"student_id" : 2},  
    {  
        $set: {  
            "age":  19  
        }  
    }  
)
```


```js
db.student.updateOne(  
    {"student_id" : 2},  
    {  
        $unset: {  
            "age": ""  
        }  
    }  
)
```

- usuń wybrane dokumenty
	- po wykonaniu każdego z przykładów sprawdź wynik za pomocą  ` db.student.find() `

- np

```js
db.student.deleteOne({"student_id": 1})  
db.student.deleteMany({"student_id": 1})  
db.student.deleteOne({"_id": ObjectId("67e669647013d10c86e71c87")});
```

- ` replace`

```JS
db.student.find({"student_id": 1});
```

```JS
db.student.replaceOne(  
    {       
	    "student_id": 1  
    },  
    {  
        "student_id": 1,  
        "firstname": "Jan",  
        "lastname": "Kowalski",  
        "hobby": "Ski"  
    }  
);
```

- wynik

```JS
[  
  {  
    "_id": {"$oid": "67ec1a5b5d92f33cd84a2598"},  
    "hobby": "Ski",  
    "lastname": "Kowalski",  
    "lirstlame": "Jan",  
    "student_id": 1  
  }  
]
```

- tablice
	- ` grades ` - tablica ocen studenta

```js
db.student.updateOne(
    {
        "student_id": 1
    },
    {
        $set : {
            "grades" : [3, 4, 5, 6, 8]
        }
    }
);
```

```js
db.student.find({"student_id": 1});
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67ec1a5b5d92f33cd84a2598"},  
    "firstname": "Jan",  
    "grades": [3, 4, 5, 6, 8],  
    "hobby": "Ski",  
    "lastname": "Kowalski",  
    "student_id": 1  
  }  
]
```


- średnia ocena

```JS
db.student.aggregate([
    {
        $match : {
		    "student_id": 1
        }
    },
    {
        $project: {
            "firstname": 1,
            "lastname": 1,
            "student_id": 1,
            "grades": 1,
            "averagegrade": { $avg: "$grades" }
        }
    }
])
```

- wynik

```JS
[  
  {  
    "_id": {"$oid": "67ec1a5b5d92f33cd84a2598"},  
    "averagegrade": 5.2,  
    "firstname": "Jan",  
    "grades": [3, 4, 5, 6, 8],  
    "lastname": "Kowalski",  
    "student_id": 1  
  }  
]
```

- dodanie elementu do tablicy

```JS
db.student.updateOne(  
    { student_id: 1 },  
    { $push: { grades: 10 } }  
);



```

```JS
db.student.find({"student_id": 1});
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67ec1a5b5d92f33cd84a2598"},  
    "firstname": "Jan",  
    "grades": [3, 4, 5, 6, 8, 10],  
    "hobby": "Ski",  
    "lastname": "Kowalski",  
    "student_id": 1  
  }  
]
```


- modyfikacja elementu w tablicy
	- indeksy zaczynają się od 0

```js
db.student.updateOne(  
    { student_id: 1 },  
    { $set: { "grades.2": 12 } }  
)
```


```JS
db.student.find({"student_id": 1});
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67ec1a5b5d92f33cd84a2598"},  
    "firstname": "Jan",  
    "grades": [3, 4, 12, 6, 8, 10],  
    "hobby": "Ski",  
    "lastname": "Kowalski",  
    "student_id": 1  
  }  
]
```


- ` $map `  -  wykonanie operacji dla każdego elementu tablicy
	- [https://www.mongodb.com/docs/manual/reference/operator/aggregation/map/](https://www.mongodb.com/docs/manual/reference/operator/aggregation/map/)
	
	- w przykładzie poniżej
		- dzielimy każdą ocenę przez 20

```js
db.student.updateOne(
    { student_id: 1 },
    [
        {
            $set: {
                grades: {
                    $map: {
                        input: "$grades",
                        as: "g",
                        in: { $divide: ["$$g", 20] }
                    }
                }
            }
        }
  ]
)
```


```js
db.student.find({"student_id": 1});
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67ec1a5b5d92f33cd84a2598"},  
    "firstname": "Jan",  
    "grades": [0.15, 0.2, 0.6, 0.3, 0.4, 0.5],  
    "hobby": "Ski",  
    "lastname": "Kowalski",  
    "student_id": 1  
  }  
]
```


---
# Zadanie 1

- Wykonaj kilka własnych eksperymentów z operacjami CRUD

- przydatne linki

	- [https://www.mongodb.com/docs/manual/crud/](https://www.mongodb.com/docs/manual/crud/)
	- [https://www.mongodb.com/docs/manual/reference/method/db.collection.find/](https://www.mongodb.com/docs/manual/reference/method/db.collection.find/)
	- [https://www.mongodb.com/docs/manual/reference/method/db.collection.insertOne/](https://www.mongodb.com/docs/manual/reference/method/db.collection.insertOne/)
	- [https://www.mongodb.com/docs/manual/reference/method/db.collection.updateOne/](https://www.mongodb.com/docs/manual/reference/method/db.collection.updateOne/)
	- [https://www.mongodb.com/docs/manual/reference/method/db.collection.deleteOne/](https://www.mongodb.com/docs/manual/reference/method/db.collection.deleteOne/)

## Zadanie 1  - rozwiązanie

> Wyniki: 
> 
> przykłady, kod, zrzuty ekranów, komentarz ...


```js
//dodanie do bazy
db.students.insertOne({
  name: "Anna",
  age: 22,
  major: "Computer Science",
  grades: [5, 4, 5]
})
db.students.insertMany([
  { name: "Kamil", age: 24, major: "Math", grades: [3, 4, 4] },
  { name: "Ola", age: 21, major: "Physics", grades: [5, 5, 5] }
])
```

```js
//wynik
{
  _id: ObjectId('69ca85d2586a94aa2a433460'),
  name: 'Anna',
  age: 22,
  major: 'Computer Science',
  grades: [
    5,
    4,
    5
  ]
}

{
  _id: ObjectId('69ca85ea586a94aa2a433461'),
  name: 'Kamil',
  age: 24,
  major: 'Math',
  grades: [
    3,
    4,
    4
  ]
}

{
  _id: ObjectId('69ca85ea586a94aa2a433462'),
  name: 'Ola',
  age: 21,
  major: 'Physics',
  grades: [
    5,
    5,
    5
  ]
}
```


---
# Przykład 2


- wybierz bazę ` employees `

```js
use employees;

db;
```


- wstaw dokument do kolekcji  ` employees `


```js
db.employees.insertOne(  
    {  
        "EmployeeID": 1,  
        "FirstName": "Nancy",  
        "LastName": "Davolio",  
        "Address" : {  
            "Street": "507 - 20th Ave. E. Apt. 2A",  
            "City": "Seattle",  
            "Country": "USA"  
        },  
        "Title": "Sales Representative",  
        "TitleOfCourtesy": "Ms.",  
        "BirthDate": ISODate("1948-12-08T00:00:00.000Z"),  
        "HireDate": ISODate("1992-05-01T00:00:00.000Z"),  
        "Phone": ["(206) 555-9857", "(206) 555-9858"],  
        "Salary": 1000  
    }  
)
```

- sprawdź zawartość kolekcji

```js
db.employees.find();
```

- wstaw kilka kolejnych dokumentów za pomocą ` insertMany `

```js
db.employees.insertMany(  
[  
    {  
        "EmployeeID": 2,  
         "FirstName": "Andrew",  
        "LastName": "Fuller",  
        "Address" : {  
            "Street": "908 W. Capital Way",  
            "City": "Tacoma",  
            "Country": "USA",  
        },  
        "Title": "Vice President, Sales",  
        "TitleOfCourtesy": "Dr.",  
        "BirthDate": ISODate("1952-02-19T00:00:00.000Z"),  
        "HireDate": ISODate("1992-08-14T00:00:00.000Z"),  
        "Phone": ["(206) 555-9482"],  
        "Salary": 10000  
    },  
    {  
        "EmployeeID": 3,  
        "FirstName": "Janet",  
        "LastName": "Leverling",  
        "Address" : {  
            "Street": "722 Moss Bay Blvd.",  
            "City": "Kirkland",  
            "Country": "USA",  
         },  
        "Title": "Sales Representative",  
        "TitleOfCourtesy": "Ms.",  
        "BirthDate": ISODate("1963-08-30T00:00:00.000Z"),  
        "HireDate": ISODate("1992-04-01T00:00:00.000Z"),  
        "Phone": ["(206) 555-3412"],  
        "Salary": 1200  
    },  
    {  
        "EmployeeID": 4,  
        "FirstName": "Margaret",  
        "LastName": "Peacock",  
        "Address" : {  
            "Street": "4110 Old Redmond Rd.",  
            "City": "Redmond",  
            "Country": "USA",  
        },  
        "Title": "Sales Representative",  
        "TitleOfCourtesy": "Mrs.",  
        "BirthDate": ISODate("1937-09-19T00:00:00.000Z"),  
        "HireDate": ISODate("1993-05-03T00:00:00.000Z"),  
        "Phone": ["(206) 555-8122"],  
        "Salary": 1100  
    },  
     {  
        "EmployeeID": 5,  
        "FirstName": "Steven",  
        "LastName": "Buchanan",  
        "Address" : {  
            "Street": "14 Garrett Hill",  
            "City": "London",  
            "Country": "UK",  
        },  
        "Title": "Sales Manager",  
        "TitleOfCourtesy": "Mr.",  
        "BirthDate": ISODate("1955-03-04T00:00:00.000Z"),  
        "HireDate": ISODate("1993-10-17T00:00:00.000Z"),  
        "Phone": ["(71) 555-4848"],  
        "Salary": 2000  
    },  
    {  
        "EmployeeID": 6,  
        "FirstName": "Michael",  
        "LastName": "Suyama",  
        "Address" : {  
            "Street": "Coventry House Miner Rd.",  
            "City": "London",  
            "Country": "UK",  
        },  
        "Title": "Sales Representative",  
        "TitleOfCourtesy": "Mr.",  
        "BirthDate": ISODate("1963-07-02T00:00:00.000Z"),  
        "HireDate": ISODate("1993-10-17T00:00:00.000Z"),  
        "Phone": ["(71) 555-7773"],  
        "Salary": 1500  
    },  
    {  
        "EmployeeID": 7,  
        "FirstName": "Robert",  
        "LastName": "King",  
        "Address" : {  
            "Street":  "Edgeham Hollow Winchester Way",  
            "City": "London",  
            "Country": "UK",  
        },  
        "Title": "Sales Representative",  
        "TitleOfCourtesy": "Mr.",  
        "BirthDate": ISODate("1960-05-29T00:00:00.000Z"),  
        "HireDate": ISODate("1994-01-02T00:00:00.000Z"),  
        "Phone": ["(71) 555-5598"],  
        "Salary": 1000  
    },  
    {  
        "EmployeeID": 8,  
        "FirstName": "Laura",  
        "LastName": "Callahan",  
        "Address" : {  
            "Street":  "4726 - 11th Ave. N.E.",  
            "City": "Seattle",  
            "Country": "USA",  
        },  
        "Title": "Inside Sales Coordinator",  
        "TitleOfCourtesy": "Ms.",  
        "BirthDate": ISODate("1958-01-09T00:00:00.000Z"),  
        "HireDate": ISODate("1994-03-05T00:00:00.000Z"),  
        "Phone": ["(206) 555-1189"],  
        "Salary": 3000  
    },  
    {  
        "EmployeeID": 9,  
        "FirstName": "Anne",  
        "LastName": "Dodsworth",  
        "Address" : {  
            "Street": "7 Houndstooth Rd.",  
            "City": "London",  
            "Country": "UK",  
        },  
        "Title": "Sales Representative",  
        "TitleOfCourtesy": "Ms.",  
        "BirthDate": ISODate("1966-01-27T00:00:00.000Z"),  
        "HireDate": ISODate("1994-11-15T00:00:00.000Z"),  
        "Phone": ["(71) 555-4444"],  
        "Salary": 1400  
    }  
]  
)
```


- w ` DataGrip`  możesz posłużyć się poleceniami ` SQL `  
	- podzbiór poleceń ` SQL ` - proste polecenia ` SELECT `

- wykonaj kilka takich poleceń
	- przeanalizuj wyniki

- np.

```sql
select * from employees;  
select * from employees where Address.Country = "USA";  
select * from employees where Address.Country = "USA" and TitleOfCourtesy = "Ms.";  
  
select * from employees where Salary > 1200;
```

- napisz odpowiadające im polecenia ` find `

- np

```js
db.employees.find({})  
db.employees.find({"Address.Country": "USA"})  
db.employees.find({$and: [{"Address.Country": "USA"}, {"TitleOfCourtesy" : "Ms."}]})  

db.employees.find(
    {
        $and: [
            {"Address.Country": "USA"},
            {"TitleOfCourtesy" : "Ms."}
        ]
    }
)
  
db.employees.find({ Salary: { $gt: 1200 } })
```


- operacja projekcji (wybór atrybutów w zbiorze wynikowym)

```js
db.employees.find({})  
```

```js
db.employees.find(
	{},
	{"FirstName": 1, "LastName":1}
)  
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67e879dbb1c69e3d7e42c9c3"},  
    "FirstName": "Nancy",  
    "LastName": "Davolio"  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c5"},  
    "FirstName": "Andrew",  
    "LastName": "Fuller"  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c6"},  
    "FirstName": "Janet",  
    "LastName": "Leverling"  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c7"},  
    "FirstName": "Margaret",  
    "LastName": "Peacock"  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c8"},  
    "FirstName": "Steven",  
    "LastName": "Buchanan"  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c9"},  
    "FirstName": "Michael",  
    "LastName": "Suyama"  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9ca"},  
    "FirstName": "Robert",  
    "LastName": "King"  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9cb"},  
    "FirstName": "Laura",  
    "LastName": "Callahan"  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9cc"},  
    "FirstName": "Anne",  
    "LastName": "Dodsworth"  
  }  
]
```

- warunek i projekcja

```js
db.employees.find(  
    {  
        "Address.Country": "USA"  
    },  
    {  
        "FirstName": 1,  
        "LastName":1  
    }  
)
```

- wynik

```js
[  
  {  
    "FirstName": "Nancy",  
    "LastName": "Davolio"  
  },  
  {  
    "FirstName": "Andrew",  
    "LastName": "Fuller"  
  },  
  {  
    "FirstName": "Janet",  
    "LastName": "Leverling"  
  },  
  {  
    "FirstName": "Margaret",  
    "LastName": "Peacock"  
  },  
  {  
    "FirstName": "Laura",  
    "LastName": "Callahan"  
  }  
]
```

- inne przykłady
	- 1 - atrybut pojawi się w zbiorze wynikowym
	- 0 - atrybut nie pojawi się
	- mogą wystąpić albo 1 albo 0 
		- nie dotyczy atrybutu ` _id_ `

```js
db.employees.find({},{"_id": 0, "FirstName": 1, "LastName": 1 })  
  
db.employees.find({},{"_id": 1, "FirstName": 0, "LastName": 0})

// to jest błąd  
db.employees.find({},{"_id": 1, "FirstName": 1, "LastName": 0})
```


- ` count `

```sql
select count(*) from employees;  
select count(*) from employees where Address.Country = "USA";  
select count(*) from employees where Address.Country = "USA" and TitleOfCourtesy = "Ms.";  
  
select count(*) from employees where Salary > 1200;  
```

```js
db.employees.find({}).count();  
db.employees.find({"Address.Country": "USA"}).count(); 

db.employees.find({$and: [{"Address.Country": "USA"}, {"TitleOfCourtesy" : "Ms."}]}).count();  


db.employees.find(  
    {  
        $and: [  
            {"Address.Country": "USA"},  
            {"TitleOfCourtesy" : "Ms."}  
        ]  
     }  
).count();
```


```js
db.employees.find({ Salary: { $gt: 1200 } }).count();
```

- wynik

```js
[  
  {  
    "result": 5  
  }  
]
```



- ` $or `  ` $and `

```sql
select * from employees where Salary > 1100  and Salary < 1500 
```

```js
db.employees.find({  
  Salary: { $gt: 1100, $lt: 1500 }  
})  
  
db.employees.find(  
    {  
        $and: [  
            { Salary: { $gt: 1100 } },  
            { Salary: { $lt: 1500 } }  
        ]  
    }  
);
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c6"},  
    "Address": {  
      "Street": "722 Moss Bay Blvd.",  
      "City": "Kirkland",  
      "Country": "USA"  
    },  
    "BirthDate": {"$date": "1963-08-30T00:00:00.000Z"},  
    "EmployeeID": 3,  
    "FirstName": "Janet",  
    "HireDate": {"$date": "1992-04-01T00:00:00.000Z"},  
    "LastName": "Leverling",  
    "Phone": ["(206) 555-3412"],  
    "Salary": 1200,  
    "Title": "Sales Representative",  
    "TitleOfCourtesy": "Ms."  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9cc"},  
    "Address": {  
      "Street": "7 Houndstooth Rd.",  
      "City": "London",  
      "Country": "UK"  
    },  
    "BirthDate": {"$date": "1966-01-27T00:00:00.000Z"},  
    "EmployeeID": 9,  
    "FirstName": "Anne",  
    "HireDate": {"$date": "1994-11-15T00:00:00.000Z"},  
    "LastName": "Dodsworth",  
    "Phone": ["(71) 555-4444"],  
    "Salary": 1400,  
    "Title": "Sales Representative",  
    "TitleOfCourtesy": "Ms."  
  }  
]
```


```sql
select * from employees where Salary <= 1100  or Salary >= 1500
```


```js
db.employees.find(  
    {  
        $or: [  
            { Salary: { $lte: 1100 } },  
            { Salary: { $gte: 1500 } }  
        ]  
    }  
);
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67e879dbb1c69e3d7e42c9c3"},  
    "Address": {  
      "Street": "507 - 20th Ave. E.\r\nApt. 2A",  
      "City": "Seattle",  
      "Country": "USA"  
    },  
    "BirthDate": {"$date": "1948-12-08T00:00:00.000Z"},  
    "EmployeeID": 1,  
    "FirstName": "Nancy",  
    "HireDate": {"$date": "1992-05-01T00:00:00.000Z"},  
    "LastName": "Davolio",  
    "Phone": ["(206) 555-9857", "(206) 555-9858"],  
    "Salary": 1000,  
    "Title": "Sales Representative",  
    "TitleOfCourtesy": "Ms."  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c5"},  
    "Address": {  
      "Street": "908 W. Capital Way",  
      "City": "Tacoma",  
      "Country": "USA"  
    },  
    "BirthDate": {"$date": "1952-02-19T00:00:00.000Z"},  
    "EmployeeID": 2,  
    "FirstName": "Andrew",  
    "HireDate": {"$date": "1992-08-14T00:00:00.000Z"},  
    "LastName": "Fuller",  
    "Phone": ["(206) 555-9482"],  
    "Salary": 10000,  
    "Title": "Vice President, Sales",  
    "TitleOfCourtesy": "Dr."  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c7"},  
    "Address": {  
      "Street": "4110 Old Redmond Rd.",  
      "City": "Redmond",  
      "Country": "USA"  
    },  
    "BirthDate": {"$date": "1937-09-19T00:00:00.000Z"},  
    "EmployeeID": 4,  
    "FirstName": "Margaret",  
    "HireDate": {"$date": "1993-05-03T00:00:00.000Z"},  
    "LastName": "Peacock",  
    "Phone": ["(206) 555-8122"],  
    "Salary": 1100,  
    "Title": "Sales Representative",  
    "TitleOfCourtesy": "Mrs."  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c8"},  
    "Address": {  
      "Street": "14 Garrett Hill",  
      "City": "London",  
      "Country": "UK"  
    },  
    "BirthDate": {"$date": "1955-03-04T00:00:00.000Z"},  
    "EmployeeID": 5,  
    "FirstName": "Steven",  
    "HireDate": {"$date": "1993-10-17T00:00:00.000Z"},  
    "LastName": "Buchanan",  
    "Phone": ["(71) 555-4848"],  
    "Salary": 2000,  
    "Title": "Sales Manager",  
    "TitleOfCourtesy": "Mr."  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c9"},  
    "Address": {  
      "Street": "Coventry House Miner Rd.",  
      "City": "London",  
      "Country": "UK"  
    },  
    "BirthDate": {"$date": "1963-07-02T00:00:00.000Z"},  
    "EmployeeID": 6,  
    "FirstName": "Michael",  
    "HireDate": {"$date": "1993-10-17T00:00:00.000Z"},  
    "LastName": "Suyama",  
    "Phone": ["(71) 555-7773"],  
    "Salary": 1500,  
    "Title": "Sales Representative",  
    "TitleOfCourtesy": "Mr."  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9ca"},  
    "Address": {  
      "Street": "Edgeham Hollow Winchester Way",  
      "City": "London",  
      "Country": "UK"  
    },  
    "BirthDate": {"$date": "1960-05-29T00:00:00.000Z"},  
    "EmployeeID": 7,  
    "FirstName": "Robert",  
    "HireDate": {"$date": "1994-01-02T00:00:00.000Z"},  
    "LastName": "King",  
    "Phone": ["(71) 555-5598"],  
    "Salary": 1000,  
    "Title": "Sales Representative",  
    "TitleOfCourtesy": "Mr."  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9cb"},  
    "Address": {  
      "Street": "4726 - 11th Ave. N.E.",  
      "City": "Seattle",  
      "Country": "USA"  
    },  
    "BirthDate": {"$date": "1958-01-09T00:00:00.000Z"},  
    "EmployeeID": 8,  
    "FirstName": "Laura",  
    "HireDate": {"$date": "1994-03-05T00:00:00.000Z"},  
    "LastName": "Callahan",  
    "Phone": ["(206) 555-1189"],  
    "Salary": 3000,  
    "Title": "Inside Sales Coordinator",  
    "TitleOfCourtesy": "Ms."  
  }  
]
```


```sql
select * from employees where year(HireDate) = 1992
```


```js
db.employees.find(  
    {  
        $expr: {  
            $eq: [ { $year: "$HireDate" }, 1992 ]  
        }  
    }  
);  
  
  
db.employees.find(  
    {  
        HireDate: {  
            $gte: ISODate("1992-01-01T00:00:00Z"),  
            $lt: ISODate("1993-01-01T00:00:00Z")  
        }  
    }  
);
```


- wynik

```js
[  
  {  
    "_id": {"$oid": "67e879dbb1c69e3d7e42c9c3"},  
    "Address": {  
      "Street": "507 - 20th Ave. E.\r\nApt. 2A",  
      "City": "Seattle",  
      "Country": "USA"  
    },  
    "BirthDate": {"$date": "1948-12-08T00:00:00.000Z"},  
    "EmployeeID": 1,  
    "FirstName": "Nancy",  
    "HireDate": {"$date": "1992-05-01T00:00:00.000Z"},  
    "LastName": "Davolio",  
    "Phone": ["(206) 555-9857", "(206) 555-9858"],  
    "Salary": 1000,  
    "Title": "Sales Representative",  
    "TitleOfCourtesy": "Ms."  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c5"},  
    "Address": {  
      "Street": "908 W. Capital Way",  
      "City": "Tacoma",  
      "Country": "USA"  
    },  
    "BirthDate": {"$date": "1952-02-19T00:00:00.000Z"},  
    "EmployeeID": 2,  
    "FirstName": "Andrew",  
    "HireDate": {"$date": "1992-08-14T00:00:00.000Z"},  
    "LastName": "Fuller",  
    "Phone": ["(206) 555-9482"],  
    "Salary": 10000,  
    "Title": "Vice President, Sales",  
    "TitleOfCourtesy": "Dr."  
  },  
  {  
    "_id": {"$oid": "67e879e2b1c69e3d7e42c9c6"},  
    "Address": {  
      "Street": "722 Moss Bay Blvd.",  
      "City": "Kirkland",  
      "Country": "USA"  
    },  
    "BirthDate": {"$date": "1963-08-30T00:00:00.000Z"},  
    "EmployeeID": 3,  
    "FirstName": "Janet",  
    "HireDate": {"$date": "1992-04-01T00:00:00.000Z"},  
    "LastName": "Leverling",  
    "Phone": ["(206) 555-3412"],  
    "Salary": 1200,  
    "Title": "Sales Representative",  
    "TitleOfCourtesy": "Ms."  
  }  
]
```



---
## Zadanie 2

- Wykonaj kilka własnych eksperymentów 

## Zadanie 2  - rozwiązanie

> Wyniki: 
> 
> przykłady, kod, zrzuty ekranów, komentarz ...


```js
--  ...
```


---
# Operacje agregacji

- aggregate
- aggregation pipline
- [https://www.mongodb.com/docs/manual/core/aggregation-pipeline/](https://www.mongodb.com/docs/manual/core/aggregation-pipeline/)


```js
db.<collection>.aggregate(

[

   {stage 1},

   {stage 2},

    …..

   {stage N}

]

)
```

- [https://www.mongodb.com/docs/manual/reference/operator/aggregation-pipeline/#std-label-aggregation-pipeline-operator-reference](https://www.mongodb.com/docs/manual/reference/operator/aggregation-pipeline/#std-label-aggregation-pipeline-operator-reference)

- Stages

	- `$match`
	- `$project`
	- `$group`
	- `$unwind`
	- `$lookup`
	- `$out`
	- ...

- Aggregation vs SQL
	- [https://www.mongodb.com/docs/manual/reference/sql-aggregation-comparison/](https://www.mongodb.com/docs/manual/reference/sql-aggregation-comparison/)



---
# Przykład 3


- baza ` employees `  z poprzedniego przykładu

```js
use employees;

db;

db.employees.find();
```


- Operacje agregacji
	- ` aggregate `

```js
db.employees.aggregate([  
    {  
        $match: {  
            "Address.Country": "USA"  
        }  
    },  
    {  
        $project: {  
            "_id": 0,  
            "LastName": 1,  
            "FirstName": 1}  
    },  
    {  
        $sort: {LastName: -1}  
    }  
])
```

- wynik

```js
[  
  {  
    "FirstName": "Nancy",  
    "LastName": "Davolio"  
  },  
  {  
    "FirstName": "Andrew",  
    "LastName": "Fuller"  
  },  
  {  
    "FirstName": "Janet",  
    "LastName": "Leverling"  
  },  
  {  
    "FirstName": "Margaret",  
    "LastName": "Peacock"  
  },  
  {  
    "FirstName": "Laura",  
    "LastName": "Callahan"  
  }  
]
```


- dodatkowe pole ` FullName `

```js
db.employees.aggregate([  
    {  
        $match: {  
            "Address.Country": "USA"  
        }  
    },  
    {  
        $project: {  
            "_id": 0,  
            "First": "$LastName",  
            "Last": "$LastName",  
            "FullName": {  
                $concat: ["$FirstName", " ", "$LastName"]  
            }  
  
       }  
    },  
    {  
        $sort: {lastname: -1}  
    }  
])
```

- wynik

```js
[  
  {  
    "First": "Davolio",  
    "FullName": "Nancy Davolio",  
    "Last": "Davolio"  
  },  
  {  
    "First": "Fuller",  
    "FullName": "Andrew Fuller",  
    "Last": "Fuller"  
  },  
  {  
    "First": "Leverling",  
    "FullName": "Janet Leverling",  
    "Last": "Leverling"  
  },  
  {  
    "First": "Peacock",  
    "FullName": "Margaret Peacock",  
    "Last": "Peacock"  
  },  
  {  
    "First": "Callahan",  
    "FullName": "Laura Callahan",  
    "Last": "Callahan"  
  }  
]
```

- grupowanie   
	- ` $group `


```sql
select TitleOfCourtesy, count(*)  
from employees  
where  Address.Country = "USA"  
group by TitleOfCourtesy
```


```js
db.employees.aggregate([  
    {  
        $match: {  
            "Address.Country": "USA"  
        }  
    },  
    {  
        $group: {  
            "_id": "$TitleOfCourtesy",  
            "cnt": {"$sum": 1}  
        }  
    }  
])
```


- wynik

```js
[  
  {  
    "_id": "Dr.",  
    "cnt": 1  
  },  
  {  
    "_id": "Mrs.",  
    "cnt": 1  
  },  
  {  
    "_id": "Ms.",  
    "cnt": 3  
  }  
]
```

- dodatkowa projekcja, zmiana nazwy

```js
db.employees.aggregate([  
    {  
        $match: {  
            "Address.Country": "USA"  
        }  
    },  
    {  
        $group: {  
            "_id": "$TitleOfCourtesy",  
            "cnt": {"$sum": 1}  
        }  
    },  
    {  
        $project: {  
            "_id": 0,  
            "TitleOfCourtesy": "$_id",  
            "cnt": 1}  
    },  
  
])
```


- grupowanie wg kilku atrybutów

```sql
select Address.Country, TitleOfCourtesy, count(*) cnt  
from employees  
group by Address.Country, TitleOfCourtesy
order by cnt desc;
```


```js
db.employees.aggregate([  
    {  
        $group: {  
            _id: {  
                country: "$Address.Country",  
                title: "$TitleOfCourtesy"  
            },  
            cnt: { $sum: 1 }  
        }  
    },  
    {  
        $project: {  
        _id: 0,  
        country: "$_id.country",  
        title: "$_id.title",  
        cnt: 1  
        }  
    },  
    {  
        $sort: {cnt: -1}  
    }  
])
```

- wynik

```js
[  
  {  
    "Country": "UK",  
    "TitleOfCourtesy": "Mr.",  
    "cnt": 3  
  },  
  {  
    "Country": "USA",  
    "TitleOfCourtesy": "Ms.",  
    "cnt": 3  
  },  
  {  
    "Country": "USA",  
    "TitleOfCourtesy": "Dr.",  
    "cnt": 1  
  },  
  {  
    "Country": "UK",  
    "TitleOfCourtesy": "Ms.",  
    "cnt": 1  
  },  
  {  
    "Country": "USA",  
    "TitleOfCourtesy": "Mrs.",  
    "cnt": 1  
  }  
]

```

- zapis wyniku
	- ` $out `
	- zapis zbioru wynikowego to kolekcji o wskazanej nazwie


```js
db.employees.aggregate([  
    {  
        $group: {  
            _id: {  
                Country: "$Address.Country",  
                TitleOfCourtesy: "$TitleOfCourtesy"  
            },  
            cnt: { $sum: 1 }  
        }  
    },  
    {  
        $project: {  
        _id: 0,  
        Country: "$_id.Country",  
        TitleOfCourtesy: "$_id.TitleOfCourtesy",  
        cnt: 1  
        }  
    },  
    {  
        $out: "em_by_country_title"  
    }  
])
```

- lista kolekcji

```js
show collections;
```

- wynik

```js
[  
  {  
    "badge": "",  
    "name": "em_by_country_title"  
  },  
  {  
    "badge": "",  
    "name": "employees"  
  }  
]
```


- zawartość kolekcji  ` em_by_cuuntry_title `

```js
db.em_by_country_title.find();
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67ec20f77b1ef941c9512dcb"},  
    "Country": "USA",  
    "TitleOfCourtesy": "Mrs.",  
    "cnt": 1  
  },  
  {  
    "_id": {"$oid": "67ec20f77b1ef941c9512dcc"},  
    "Country": "USA",  
    "TitleOfCourtesy": "Ms.",  
    "cnt": 3  
  },  
  {  
    "_id": {"$oid": "67ec20f77b1ef941c9512dcd"},  
    "Country": "USA",  
    "TitleOfCourtesy": "Dr.",  
    "cnt": 1  
  },  
  {  
    "_id": {"$oid": "67ec20f77b1ef941c9512dce"},  
    "Country": "UK",  
    "TitleOfCourtesy": "Mr.",  
    "cnt": 3  
  },  
  {  
    "_id": {"$oid": "67ec20f77b1ef941c9512dcf"},  
    "Country": "UK",  
    "TitleOfCourtesy": "Ms.",  
    "cnt": 1  
  }  
]
```

---
## Zadanie 3

- Wykonaj kilka własnych eksperymentów 


## Zadanie 3  - rozwiązanie

> Wyniki: 
> 
> przykłady, kod, zrzuty ekranów, komentarz ...


```js
--  ...
```

---
# Przykład 4


Odtwórz z backupu bazę `north0`
- plik z z backup'em bazy danych dostępny jest w moodle

- najprostsza wersja
	- ` mongorestore <folder> `

```
mongorestore dump
```

- to polecenie odtworzy wszystkie bazy danych znajdujące się we wskazanym folderze (w tym przypadku ` dump `) 
	- najłatwiej wgrać tam folder zawierający pliki z backupem i wykonać proste polecenie mongorestore 
- dokumentacja:
	- https://www.mongodb.com/docs/database-tools/mongorestore/

Wybierz bazę north0

```
use north0
```

Baza `north0` jest kopią relacyjnej bazy danych `Northwind`
- poszczególne kolekcje odpowiadają tabelom w oryginalnej bazie `Northwind`


zapoznaj się ze strukturą dokumentów w bazie `North0`

```js
db.customers.find()
db.orders.find();
db.orderdetails.find();

```

---
# Przyklad 5

produkty, kategorie, dostawcy

```js
db.categories.find().limit(2);
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "67eb06a27b1ef941c950fb49"},  
    "CategoryID": 1,  
    "CategoryName": "Beverages",  
    "Description": "Soft drinks, coffees, teas, beers, and ales"  
  },  
  {  
    "_id": {"$oid": "67eb06a27b1ef941c950fb4a"},  
    "CategoryID": 2,  
    "CategoryName": "Condiments",  
    "Description": "Sweet and savory sauces, relishes, spreads, and seasonings"  
  }  
]
```


```js
db.products.find().limit(2);
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f50"},  
    "CategoryID": 1,  
    "Discontinued": false,  
    "ProductID": 1,  
    "ProductName": "Chai",  
    "QuantityPerUnit": "10 boxes x 20 bags",  
    "ReorderLevel": 10,  
    "SupplierID": 1,  
    "UnitPrice": 18,  
    "UnitsInStock": 39,  
    "UnitsOnOrder": 0  
  },  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f51"},  
    "CategoryID": 1,  
    "Discontinued": false,  
    "ProductID": 2,  
    "ProductName": "Chang",  
    "QuantityPerUnit": "24 - 12 oz bottles",  
    "ReorderLevel": 25,  
    "SupplierID": 1,  
    "UnitPrice": 19,  
    "UnitsInStock": 17,  
    "UnitsOnOrder": 40  
  }  
]
```

```js
db.suppliers.find().limit(2);
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "63a05d3cbb3b972d6f4e09f7"},  
    "Address": "49 Gilbert St.",  
    "City": "London",  
    "CompanyName": "Exotic Liquids",  
    "ContactName": "Charlotte Cooper",  
    "ContactTitle": "Purchasing Manager",  
    "Country": "UK",  
    "Fax": null,  
    "HomePage": null,  
    "Phone": "(171) 555-2222",  
    "PostalCode": "EC1 4SD",  
    "Region": null,  
    "SupplierID": 1  
  },  
  {  
    "_id": {"$oid": "63a05d3cbb3b972d6f4e09f8"},  
    "Address": "P.O. Box 78934",  
    "City": "New Orleans",  
    "CompanyName": "New Orleans Cajun Delights",  
    "ContactName": "Shelley Burke",  
    "ContactTitle": "Order Administrator",  
    "Country": "USA",  
    "Fax": null,  
    "HomePage": "#CAJUN.HTM#",  
    "Phone": "(100) 555-4822",  
    "PostalCode": "70117",  
    "Region": "LA",  
    "SupplierID": 2  
  }  
]
```


Uwaga:
- etap ` $match ` z pustym warunkiem zwraca wszystkie dokumenty (nie jest potrzebny, można go pominąć)
	- w przykładzie został dodany żeby łatwiej można było wykonać własne eksperymenty ograniczające zbiór dokumentów z kolekcji ` products `   

```js
db.products.aggregate([  
    {  
        $match : { }  
    },  
  
    //suppliers  
    {  
        $lookup : {  
            from: "suppliers",  
            localField: "SupplierID",  
            foreignField: "SupplierID",  
            as: "Supplier"  
        }  
    },  
    {  
        $limit : 2  
    }  
])
```

- pole ` Suplier ` jest tablicą
	- zawiera 1 obiekt

- wynik

```js
[  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f50"},  
    "CategoryID": 1,  
    "Discontinued": false,  
    "ProductID": 1,  
    "ProductName": "Chai",  
    "QuantityPerUnit": "10 boxes x 20 bags",  
    "ReorderLevel": 10,  
    "Supplier": [  
      {  
        "_id": {"$oid": "63a05d3cbb3b972d6f4e09f7"},  
        "SupplierID": 1,  
        "CompanyName": "Exotic Liquids",  
        "ContactName": "Charlotte Cooper",  
        "ContactTitle": "Purchasing Manager",  
        "Address": "49 Gilbert St.",  
        "City": "London",  
        "Region": null,  
        "PostalCode": "EC1 4SD",  
        "Country": "UK",  
        "Phone": "(171) 555-2222",  
        "Fax": null,  
        "HomePage": null  
      }  
    ],  
    "SupplierID": 1,  
    "UnitPrice": 18,  
    "UnitsInStock": 39,  
    "UnitsOnOrder": 0  
  },  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f51"},  
    "CategoryID": 1,  
    "Discontinued": false,  
    "ProductID": 2,  
    "ProductName": "Chang",  
    "QuantityPerUnit": "24 - 12 oz bottles",  
    "ReorderLevel": 25,  
    "Supplier": [  
      {  
        "_id": {"$oid": "63a05d3cbb3b972d6f4e09f7"},  
        "SupplierID": 1,  
        "CompanyName": "Exotic Liquids",  
        "ContactName": "Charlotte Cooper",  
        "ContactTitle": "Purchasing Manager",  
        "Address": "49 Gilbert St.",  
        "City": "London",  
        "Region": null,  
        "PostalCode": "EC1 4SD",  
        "Country": "UK",  
        "Phone": "(171) 555-2222",  
        "Fax": null,  
        "HomePage": null  
      }  
    ],  
    "SupplierID": 1,  
    "UnitPrice": 19,  
    "UnitsInStock": 17,  
    "UnitsOnOrder": 40  
  }  
]
```

- ` $unwind `
	- pole ` Supplier `  jest obiektem/dokumentem

```js
db.products.aggregate([  
    {  
        $match : { }  
    },  
  
    //suppliers  
    {  
        $lookup : {  
            from: "suppliers",  
            localField: "SupplierID",  
            foreignField: "SupplierID",  
            as: "Supplier"  
        }  
    },  
    {  
        $unwind : "$Supplier"  
    },  
   {  
        $limit : 2  
   }  
])
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f50"},  
    "CategoryID": 1,  
    "Discontinued": false,  
    "ProductID": 1,  
    "ProductName": "Chai",  
    "QuantityPerUnit": "10 boxes x 20 bags",  
    "ReorderLevel": 10,  
    "Supplier": {  
      "_id": {"$oid": "63a05d3cbb3b972d6f4e09f7"},  
      "SupplierID": 1,  
      "CompanyName": "Exotic Liquids",  
      "ContactName": "Charlotte Cooper",  
      "ContactTitle": "Purchasing Manager",  
      "Address": "49 Gilbert St.",  
      "City": "London",  
      "Region": null,  
      "PostalCode": "EC1 4SD",  
      "Country": "UK",  
      "Phone": "(171) 555-2222",  
      "Fax": null,  
      "HomePage": null  
    },  
    "SupplierID": 1,  
    "UnitPrice": 18,  
    "UnitsInStock": 39,  
    "UnitsOnOrder": 0  
  },  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f51"},  
    "CategoryID": 1,  
    "Discontinued": false,  
    "ProductID": 2,  
    "ProductName": "Chang",  
    "QuantityPerUnit": "24 - 12 oz bottles",  
    "ReorderLevel": 25,  
    "Supplier": {  
      "_id": {"$oid": "63a05d3cbb3b972d6f4e09f7"},  
      "SupplierID": 1,  
      "CompanyName": "Exotic Liquids",  
      "ContactName": "Charlotte Cooper",  
      "ContactTitle": "Purchasing Manager",  
      "Address": "49 Gilbert St.",  
      "City": "London",  
      "Region": null,  
      "PostalCode": "EC1 4SD",  
      "Country": "UK",  
      "Phone": "(171) 555-2222",  
      "Fax": null,  
      "HomePage": null  
    },  
    "SupplierID": 1,  
    "UnitPrice": 19,  
    "UnitsInStock": 17,  
    "UnitsOnOrder": 40  
  }  
]
```


- projekcja
	- tylko wybrane pola z ` Supplier `

```js
db.products.aggregate([  
    {  
        $match : { }  
    },  
  
    //suppliers  
    {  
        $lookup : {  
            from: "suppliers",  
            localField: "SupplierID",  
            foreignField: "SupplierID",  
            as: "Supplier_tmp"  
        }  
    },  
    {  
        $unwind : "$Supplier_tmp"  
    },  
    {  
        $addFields: {  
            Supplier: {  
                SupplierID: "$SupplierID",  
                CompanyName: "$Supplier_tmp.CompanyName",  
                Country: "$Supplier_tmp.Country"  
            }  
       }  
    },  
    {  
        $project: {  
            SupplierID: 0,  
            Supplier_tmp: 0  
        }  
    },  
    {  
        $limit : 2  
    }  
])
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f50"},  
    "CategoryID": 1,  
    "Discontinued": false,  
    "ProductID": 1,  
    "ProductName": "Chai",  
    "QuantityPerUnit": "10 boxes x 20 bags",  
    "ReorderLevel": 10,  
    "Supplier": {  
      "SupplierID": 1,  
      "CompanyName": "Exotic Liquids",  
      "Country": "UK"  
    },  
    "UnitPrice": 18,  
    "UnitsInStock": 39,  
    "UnitsOnOrder": 0  
  },  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f51"},  
    "CategoryID": 1,  
    "Discontinued": false,  
    "ProductID": 2,  
    "ProductName": "Chang",  
    "QuantityPerUnit": "24 - 12 oz bottles",  
    "ReorderLevel": 25,  
    "Supplier": {  
      "SupplierID": 1,  
      "CompanyName": "Exotic Liquids",  
      "Country": "UK"  
    },  
    "UnitPrice": 19,  
    "UnitsInStock": 17,  
    "UnitsOnOrder": 40  
  }  
]
```


- dodaj do wyniku inf. o kategoriach

```js
  
db.products.aggregate([  
    {  
        $match : { }  
    },  
  
    //suppliers  
    {  
        $lookup : {  
            from: "suppliers",  
            localField: "SupplierID",  
            foreignField: "SupplierID",  
            as: "Supplier_tmp"  
        }  
    },  
    //categories  
    {  
        $lookup : {  
            from: "categories",  
            localField: "CategoryID",  
            foreignField: "CategoryID",  
            as: "Category_tmp"  
        }  
    },  
    {  
        $unwind : "$Supplier_tmp"  
    },  
    {  
        $unwind : "$Category_tmp"  
    },  
    {  
        $addFields: {  
            Supplier: {  
                SupplierID: "$SupplierID",  
                CompanyName: "$Supplier_tmp.CompanyName",  
                Country: "$Supplier_tmp.Country"  
            },  
            Category: {  
                CategoryID: "$CategoryID",  
                CategorynameName: "$Category_tmp.CategoryName",  
            }  
        }  
    },  
    {  
        $project: {  
            SupplierID: 0,  
            Supplier_tmp: 0,  
            CategoryID: 0,  
            Category_tmp: 0  
        }  
    },  
    {  
        $limit : 2  
    }  
])
```

- wynik

```js
[  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f50"},  
    "Category": {  
      "CategoryID": 1,  
      "CategorynameName": "Beverages"  
    },  
    "Discontinued": false,  
    "ProductID": 1,  
    "ProductName": "Chai",  
    "QuantityPerUnit": "10 boxes x 20 bags",  
    "ReorderLevel": 10,  
    "Supplier": {  
      "SupplierID": 1,  
      "CompanyName": "Exotic Liquids",  
      "Country": "UK"  
    },  
    "UnitPrice": 18,  
    "UnitsInStock": 39,  
    "UnitsOnOrder": 0  
  },  
  {  
    "_id": {"$oid": "63a0607abb3b972d6f4e1f51"},  
    "Category": {  
      "CategoryID": 1,  
      "CategorynameName": "Beverages"  
    },  
    "Discontinued": false,  
    "ProductID": 2,  
    "ProductName": "Chang",  
    "QuantityPerUnit": "24 - 12 oz bottles",  
    "ReorderLevel": 25,  
    "Supplier": {  
      "SupplierID": 1,  
      "CompanyName": "Exotic Liquids",  
      "Country": "UK"  
    },  
    "UnitPrice": 19,  
    "UnitsInStock": 17,  
    "UnitsOnOrder": 40  
  }  
]
```

---
# Przykład 6

- dostawcy i produkty

```js
db.suppliers.aggregate([  
    {  
        $match : { }  
    },  
  
    //products  
    {  
        $lookup : {  
            from: "products",  
            localField: "SupplierID",  
            foreignField: "SupplierID",  
            as: "Products"  
        }  
    },  
    {  
        $addFields: {  
            Products: {  
                $map: {  
                    input: "$Products",  
                    as: "product",  
                    in: {  
                        ProductID: "$$product.ProductID",  
                        ProductName: "$$product.ProductName",  
                        UnitPrice: "$$product.UnitPrice",  
                        Discontinued: "$$product.Discontinued",  
                    }  
                }  
            }  
        }  
    },  
    {  
        $limit : 2  
    }  
])
```


- wynik

```js
[  
  {  
    "_id": {"$oid": "63a05d3cbb3b972d6f4e09f7"},  
    "Address": "49 Gilbert St.",  
    "City": "London",  
    "CompanyName": "Exotic Liquids",  
    "ContactName": "Charlotte Cooper",  
    "ContactTitle": "Purchasing Manager",  
    "Country": "UK",  
    "Fax": null,  
    "HomePage": null,  
    "Phone": "(171) 555-2222",  
    "PostalCode": "EC1 4SD",  
    "Products": [  
      {  
        "ProductID": 1,  
        "ProductName": "Chai",  
        "UnitPrice": 18,  
        "Discontinued": false  
      },  
      {  
        "ProductID": 2,  
        "ProductName": "Chang",  
        "UnitPrice": 19,  
        "Discontinued": false  
      },  
      {  
        "ProductID": 3,  
        "ProductName": "Aniseed Syrup",  
        "UnitPrice": 10,  
        "Discontinued": false  
      }  
    ],  
    "Region": null,  
    "SupplierID": 1  
  },  
  {  
    "_id": {"$oid": "63a05d3cbb3b972d6f4e09f8"},  
    "Address": "P.O. Box 78934",  
    "City": "New Orleans",  
    "CompanyName": "New Orleans Cajun Delights",  
    "ContactName": "Shelley Burke",  
    "ContactTitle": "Order Administrator",  
    "Country": "USA",  
    "Fax": null,  
    "HomePage": "#CAJUN.HTM#",  
    "Phone": "(100) 555-4822",  
    "PostalCode": "70117",  
    "Products": [  
      {  
        "ProductID": 4,  
        "ProductName": "Chef Anton's Cajun Seasoning",  
        "UnitPrice": 22,  
        "Discontinued": false  
      },  
      {  
        "ProductID": 5,  
        "ProductName": "Chef Anton's Gumbo Mix",  
        "UnitPrice": 21.35,  
        "Discontinued": true  
      },  
      {  
        "ProductID": 65,  
        "ProductName": "Louisiana Fiery Hot Pepper Sauce",  
        "UnitPrice": 21.05,  
        "Discontinued": false  
      },  
      {  
        "ProductID": 66,  
        "ProductName": "Louisiana Hot Spiced Okra",  
        "UnitPrice": 17,  
        "Discontinued": false  
      }  
    ],  
    "Region": "LA",  
    "SupplierID": 2  
  }  
]
```

---
# Przyklad 7

- zamówienia

```js
db.orders.aggregate(  
    {  
        $match : { }  
    },  
  
   //customers  
    {  
        $lookup : {  
            from: "customers",  
            localField: "CustomerID",  
            foreignField: "CustomerID",  
            as: "Customer"  
        }  
  
   },  
   {  
        $unwind : "$Customer"  
   },  
   {  
        $project : {  
            "Customer._id": 0,  
            "Customer.ContactName": 0,  
            "Customer.ContactTitle": 0,  
            "Customer.City": 0,  
//             "Customer.Country": 0,  
            "Customer.Address": 0,  
            "Customer.PostalCode": 0,  
            "Customer.Region": 0,  
            "Customer.Phone": 0,  
            "Customer.Fax": 0  
        }  
    },  
  
    //employees  
    {  
        $lookup : {  
            from: "employees",  
                localField: "EmployeeID",  
                foreignField: "EmployeeID",  
                as: "Employee"  
        }  
    },  
    {  
        $unwind : "$Employee"  
    },  
    {  
        $project : {  
            "Employee._id": 0,  
            "Employee.Title": 0,  
            "Employee.TitleOfCourtesy": 0,  
            "Employee.BirthDate": 0,  
            "Employee.HireDate": 0,  
            "Employee.Address": 0,  
            "Employee.PostalCode": 0,  
            "Employee.City": 0,  
            "Employee.Region": 0,  
            "Employee.Country": 0,  
            "Employee.HomePhone": 0,  
            "Employee.Extension": 0,  
            "Employee.Photo": 0,  
            "Employee.Notes": 0,  
            "Employee.ReportsTo": 0,  
            "Employee.PhotoPath": 0  
        }  
    },  
    //shippers  
    {  
        $lookup : {  
            from: "shippers",  
            localField: "ShipVia",  
            foreignField: "ShipperID",  
            as: "Shipper"  
        }  
    },  
    {  
        $unwind : "$Shipper"  
    },  
    {  
        $project : {  
            "Shipper._id": 0,  
            "Shipper.Phone": 0  
  
        }  
    },  
    //orderdetails  
    {  
        $lookup : {  
            from: "orderdetails_tmp",  
            localField: "OrderID",  
            foreignField: "OrderID",  
            as: "Orderdetails"  
        }  
    },  
    {  
        $project : {  
              "Orderdetails._id": 0,  
              "Orderdetails.OrderID": 0  
  
        }  
    },  
    {  
        $project : {  
              "CustomerID": 0,  
              "EmployeeID": 0,  
              "ShipVia": 0  
        }  
   },  
   {  
    $out : "orders_tmp"  
   }  
  
)
```


```js
db.orders_tmp.find().limit(2);
```


- wynik

```js
[  
  {  
    "_id": {"$oid": "63a060b9bb3b972d6f4e1fc6"},  
    "Customer": {  
      "CustomerID": "VINET",  
      "CompanyName": "Vins et alcools Chevalier",  
      "Country": "France"  
    },  
    "Employee": {  
      "EmployeeID": 5,  
      "LastName": "Buchanan",  
      "FirstName": "Steven"  
    },  
    "Freight": 32.38,  
    "OrderDate": {"$date": "1996-07-04T00:00:00.000Z"},  
    "OrderID": 10248,  
    "Orderdetails": [  
      {  
        "UnitPrice": 14,  
        "Quantity": 12,  
        "Discount": 0,  
        "product": {  
          "ProductID": 11,  
          "ProductName": "Queso Cabrales",  
          "QuantityPerUnit": "1 kg pkg."  
        },  
        "TotalValue": 168  
      },  
      {  
        "UnitPrice": 9.8,  
        "Quantity": 10,  
        "Discount": 0,  
        "product": {  
          "ProductID": 42,  
          "ProductName": "Singaporean Hokkien Fried Mee",  
          "QuantityPerUnit": "32 - 1 kg pkgs."  
        },  
        "TotalValue": 98  
      },  
      {  
        "UnitPrice": 34.8,  
        "Quantity": 5,  
        "Discount": 0,  
        "product": {  
          "ProductID": 72,  
          "ProductName": "Mozzarella di Giovanni",  
          "QuantityPerUnit": "24 - 200 g pkgs."  
        },  
        "TotalValue": 174  
      }  
    ],  
    "RequiredDate": {"$date": "1996-08-01T00:00:00.000Z"},  
    "ShipAddress": "59 rue de l'Abbaye",  
    "ShipCity": "Reims",  
    "ShipCountry": "France",  
    "ShipName": "Vins et alcools Chevalier",  
    "ShipPostalCode": "51100",  
    "ShipRegion": null,  
    "ShippedDate": {"$date": "1996-07-16T00:00:00.000Z"},  
    "Shipper": {  
      "ShipperID": 3,  
      "CompanyName": "Federal Shipping"  
    }  
  },  
  {  
    "_id": {"$oid": "63a060b9bb3b972d6f4e1fc7"},  
    "Customer": {  
      "CustomerID": "TOMSP",  
      "CompanyName": "Toms Spezialitäten",  
      "Country": "Germany"  
    },  
    "Employee": {  
      "EmployeeID": 6,  
      "LastName": "Suyama",  
      "FirstName": "Michael"  
    },  
    "Freight": 11.61,  
    "OrderDate": {"$date": "1996-07-05T00:00:00.000Z"},  
    "OrderID": 10249,  
    "Orderdetails": [  
      {  
        "UnitPrice": 18.6,  
        "Quantity": 9,  
        "Discount": 0,  
        "product": {  
          "ProductID": 14,  
          "ProductName": "Tofu",  
          "QuantityPerUnit": "40 - 100 g pkgs."  
        },  
        "TotalValue": 167.4  
      },  
      {  
        "UnitPrice": 42.4,  
        "Quantity": 40,  
        "Discount": 0,  
        "product": {  
          "ProductID": 51,  
          "ProductName": "Manjimup Dried Apples",  
          "QuantityPerUnit": "50 - 300 g pkgs."  
        },  
        "TotalValue": 1696  
      }  
    ],  
    "RequiredDate": {"$date": "1996-08-16T00:00:00.000Z"},  
    "ShipAddress": "Luisenstr. 48",  
    "ShipCity": "Münster",  
    "ShipCountry": "Germany",  
    "ShipName": "Toms Spezialitäten",  
    "ShipPostalCode": "44087",  
    "ShipRegion": null,  
    "ShippedDate": {"$date": "1996-07-10T00:00:00.000Z"},  
    "Shipper": {  
      "ShipperID": 1,  
      "CompanyName": "Speedy Express"  
    }  
  }  
]
```


## Zadanie 4

- zmień (popraw) strukturę dokumentu w  kolekcji   ` orders_tmp `

- stwórz obiekt/dokument grupujący inf. o adresie wysyłki
	- np.

```js
Shippment : {
	"ShipAddress": "Luisenstr. 48",  
	"ShipCity": "Münster",  
    "ShipCountry": "Germany",  
    "ShipName": "Toms Spezialitäten",  
    "ShipPostalCode": "44087",  
    "ShipRegion": null,  
}
```

- stwórz obiekt/dokument grupujący inf. o datach (zamówienia, wysyłki ... )
	- np.

```js
Dates : {
	"OrderDate": {"$date": "1996-07-04T00:00:00.000Z
	"RequiredDate": {"$date": "1996-08-01T00:00:00.000Z"},	
	"ShippedDate": {"$date": "1996-07-16T00:00:00.000Z"},
}

```

- Dodaj inf. o pełnej wartości całego zamówienia

```js
TotalOrderValue : ...
```

## Zadanie 4  - rozwiązanie

> Wyniki: 
> 
> przykłady, kod, zrzuty ekranów, komentarz ...


```js
db.orders_tmp.updateMany(
    {},
    [
        {
            $set: {
                Shippment: {
                    ShipAddress: "$ShipAddress",
                    ShipCity: "$ShipCity",
                    ShipCountry: "$ShipCountry",
                    ShipName: "$ShipName",
                    ShipPostalCode: "$ShipPostalCode",
                    ShipRegion: "$ShipRegion"
                }
            }
        },
        {
            $unset: [
                "ShipAddress",
                "ShipCity",
                "ShipCountry",
                "ShipName",
                "ShipPostalCode",
                "ShipRegion"
            ]
        }
    ]
);
```

```js
db.orders_tmp.updateMany(
    {},
    [
        {
            $set: {
                Dates: {
                    OrderDate: "$OrderDate",
                    RequiredDate: "$RequiredDate",
                    ShippedDate: "$ShippedDate"
                }
            }
        },
        {
            $unset: [
                "OrderDate",
                "RequiredDate",
                "ShippedDate"
            ]
        }
    ]
);
db.orders_tmp.updateMany(
    {},
    [
        {
            $set: {
                TotalOrderValue: {
                    $sum: {
                        $map: {
                            input: "$OrderDetails",
                            as: "item",
                            in: {
                                $multiply: [
                                    "$$item.UnitPrice",
                                    "$$item.Quantity",
                                    { $subtract: [1, "$$item.Discount"] }
                                ]
                            }
                        }
                    }
                }
            }
        }
    ]
);

```
![zrzut ekranu wyniku](image.png)
![zrzut ekranu wyniku - ciąg dalszy](image-1.png)
