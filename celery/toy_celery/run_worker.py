# -*- coding: utf-8 -*-
from worker import Worker
from app import LongTask


if __name__ == "__main__":
    # Start the process to execute task
    long_task = LongTask()
    worker = Worker(task=long_task)

    worker.start()
