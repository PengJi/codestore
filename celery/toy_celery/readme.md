# steps
1. intall redis  
`brew install redis`
   
2. start redis  
`redis-server`
   
3. start flask  
`python app.py`
   
4. start worker  
`python run_worker.py`  
equivalent to execute `celery -A xxx worker --loglevel=info`
   
5. start task
`http://127.0.0.1:5000/`

# references
[动手实现一个简单的Celery](https://juejin.cn/post/6844903957312045064)
[ayuLiao/toy_celery](https://github.com/ayuLiao/toy_celery)