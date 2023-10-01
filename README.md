# The Leetcode datahouse

A highly distributed, robust, modular, scalable web data extractor. Bring your own machine to start contributing to the data.

This project is aimed to extract the data from leetcode for data analysis. A project for beginners in rust to get started to learn and contribute.

Services
- Producer: This service is what generates the data by extracting it from the web. It coordinates with the `Controller` extract the right data and push it to the controller. There can be multiple producers running at any given time. These are run on the client machines and are light weight.

- Consumer: The service which pushes the data from a temporary storage or a message queue to a persistent storage. There will be only one consumer running at any given time.

- Controller: Coordinates the producers to produce the right data. It stores the produced data in a temporary storage like a message queue or redis. 

The communication between `Controller` and `Producer` happens by grpc.

# Architecture
![alt architecture diagram][diagram]

[diagram]: https://github.com/Narayanbhat166/leetcode_datahouse/assets/48803246/d0224a74-25a1-49df-b4fa-8fb37d6e8cc1
