#!/usr/bin/sh
echo "requirepass eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81" >> /etc/redis/redis.conf
echo "save 20 1" >> /etc/redis/redis.conf
sed -i 's/loglevel.*/loglevel warning/' /etc/redis/redis.conf
sed -i 's/notify-keyspace-events.*/notify-keyspace-events KEA/' /etc/redis/redis.conf
sed -i 's/bind.*/# bind/' /etc/redis/redis.conf
service redis-server start