Test database for development use these commands to start the database and connect to it.
Default password is devuser_p

NOTE. THIS IS THE ONLY SUPPLIED .env file. 
An example will be provided at a later date

```
docker volume create --name p_bench
docker-compose up -d

psql -U duser -h localhost -p 6998 -d p_bench
```