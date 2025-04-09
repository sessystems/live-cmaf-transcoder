FROM redis:alpine
ENTRYPOINT ["redis-server","--save","20","1","--loglevel","warning","--requirepass","eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81","--notify-keyspace-events","KEA"]
    
