# Setup

## Launch MongoDB within container
```sh
docker-compose up -d
```

## Install Kafka locally
```sh
wget -O kafka.tgz https://dlcdn.apache.org/kafka/3.6.1/kafka_2.13-3.6.1.tgz
tar -xzf kafka.tgz
```

## Start ZooKeeper
* Run this command inside Kafka directory (previous step);
```sh
# cd kafka
bin/zookeeper-server-start.sh config/zookeeper.properties
```

## Start Kafka server
```sh
bin/kafka-server-start.sh config/server.properties
```

## Start standalone cluster
```sh
# cd config
bin/connect-standalone.sh connect-standalone.properties mongo-source.properties 
```