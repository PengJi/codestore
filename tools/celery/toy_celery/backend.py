import redis


class Backend:
    """
    use redis to store result.
    """

    def __init__(self):
        host = "localhost"
        port = 6379
        password = None
        self.redis_instance = redis.StrictRedis(host=host, port=port, password=password, db=0, socket_timeout=8.0)

    def enqueue(self, item, queue_name):
        self.redis_instance.lpush(queue_name, item)

    def dequeue(self, queue_name):
        dequed_item = self.redis_instance.brpop(queue_name, timeout=3)
        if not dequed_item:
            return None
        dequed_item = dequed_item[1]
        return dequed_item
