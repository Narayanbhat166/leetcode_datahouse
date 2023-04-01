# Role of producers and consumers

## The producer

Role of the producer is to scrape the data from leetcode and push the unprocessed data to a temporary storage, in this case, a redis queue. This is the high level architecture overview. There are few aspects to keep in mind

### There will be multiple producers

This is in order to keep up with the rate at which submission are made to the leetcode. Idea is to support realtime scrapping.

### It should be possible for anyone to setup the producer.

This allows to distribute the scrapping load. There must be minimal setup to be done to run the producer. Idea is to have producer run on edge devices like raspberry pis.

### Avoid duplicate submission id being scrapped by multiple producers.

There must be some kind of locking mechanism on the submission id to prevent the same submission id being scrapped by multiple producers.

## The consumer

Role of the consumer is to process the scrapped data if necessary and persist it in the database. There will be only one consumer be run. This is to make sure that database reads are not bottlenecked by many connections writing to it. It is also sufficient to have just one process handle this task as it involves no significant network latency.

### Preprocessing of data

The data scrapped from leetcode may have some data which may have to be persisted in different palces. The code has to be saved as a file in a file system ( there will be no analysis done on the code data), and submission related data has to be stored in a relational database ( I have heard that one can write powerful queries using SQL ).
