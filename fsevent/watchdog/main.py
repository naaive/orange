import io
import logging
import sys

from watchdog.events import LoggingEventHandler
from watchdog.observers import Observer

if __name__ == "__main__":
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    logging.basicConfig(level=logging.INFO,
                        format='%(message)s',
                        datefmt='%Y-%m-%d %H:%M:%S')

    path = sys.argv[1] if len(sys.argv) > 1 else 'c://'
    event_handler = LoggingEventHandler()

    while True:
        try:
            observer = Observer()
            observer.schedule(event_handler, path, recursive=True)
            observer.start()
            observer.join()
        except Exception as e:
            logging.exception(e)
