# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

**Akmal Ramadhan - Advanced Programming A - 2023-2024 Genap**

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:

    | variable              | type   | description | 
    |-----------------------|--------|-------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |

2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
> In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or `trait` in Rust) in this BambangShop case, or a single Model `struct` is enough?

Untuk kasus sekarang ini, saya rasa **tidak perlu** menggunakan _interface_ (`trait`) dan cukup menggunakan _single Model `struct`._ Kenapa? Karena _observer_-nya hanya satu yaitu _class_ Subscriber. Penggunaan _interface_ akan berguna ketika kita memiliki banyak _observer_ yang jenisnya berbeda-beda.

> `id` in `Program` and `url` in `Subscriber` is intended to be unique. Explain based on your understanding, is using `Vec` (list) sufficient or using `DashMap` (map/dictionary) like we currently use is necessary for this case?

Menurut saya, sudah tepat menggunakan `DashMap`. Jika kita menggunakan `Vec`, perlu membuat dua buah _array_ untuk menyimpan `id` dan `url` yang berbeda dan kita perlu iterasi isi _array_ untuk mencari nilai pasangan `id` dan `url`. Dengan menggunakan `DashMap`, kita bisa menyimpan `id` dan `url` dalam satu struktur saja sehingga penggunaannya lebih mudah. Terakhir, `DashMap` mendukung _concurrent access_ sehingga lebih aman digunakan jika kedepannya aplikasi ini akan di-_deploy_ dalam _multi-threading_.

> When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (`SUBSCRIBERS`) static variable, we used the `DashMap` external library for `thread safe HashMap`. Explain based on your understanding of design patterns, do we still need `DashMap` or we can implement Singleton pattern instead?

Aplikasi BambangShop merupakan aplikasi yang menggunakan _multi-threading_ sehingga kita perlu menggunakan `DashMap` dibandingkan dengan Singleton _pattern_. Kenapa? Karena `DashMap` merupakan _thread-safe HashMap_ yang mendukung _concurrent access_. Dengan begitu, data `SUBSCRIBERS` dapat diakses secara _concurrency_ tanpa ada isu. Jika menggunakan pendekatan Singleton _pattern_, objek hanya memiliki satu _instance_ selama program berjalan.  Dengan begitu, ketika program dijalankan dalam _multi-threading_, kita perlu melakukan _locking_ pada objek. Namun, hal tersebut membuat program menjadi lebih kompleks dan rawan _deadlock_.

#### Reflection Publisher-2
> In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model? 

Agar mudah di-_maintability_ dan bersifat _modularity_ dan _scalability_ dengan prinsip **Single Responsibility** dimana sebuah _class_ harus memiliki satu kepentingan. Dengan memisahkan _Service_ dan _Repository_ dari _Model_, kita bisa memisahkan antara _business logic_ dan _data access_ sehingga lebih mudah untuk di-_test_ dan di-_debug_. 

> What happens if we only use the Model? Explain your imagination on how the interactions between each model (`Program`, `Subscriber`, `Notification`) affect the code complexity for each model?

Dampaknya adalah kode akan menjadi **lebih kompleks** dan **sulit untuk di-_maintain_**. **_Coupling_** antar dua _class_ akan **menjadi lebih tinggi** sehingga jika dilakukan suatu perubahan maka dampaknya akan banyak perubahan juga dibanding jika kita memisahkan _business logic_ dan _data access_.

> Have you explored more about **Postman**? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.
 
Dengan Postman, saya dapat melakukan API _Testing_. Saya dapat melihat hasil dari _request_ yang saya kirimkan dan memastikan bahwa _response_ yang saya terima sudah sesuai dengan yang saya harapkan.  Salah satu fitur pendukungnya yaitu _Collection_ yang bisa mengelompokkan _request_ berdasarkan _folder_ dan _Environment_ memungkinkan saya untuk menyimpan _variable_ yang sering saya gunakan.

#### Reflection Publisher-3

