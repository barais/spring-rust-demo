# Spring RS demo for TAA

This small project shows a backend architecture in Rust that follows the principles and technologies seen in the TAA course at Univ Rennes.
 
# Project setup

## Start the database

```bash
cd dev-docker/
docker compose up -d 
```

Le docker 

```bash
# Get public key from keycloak
curl -s http://localhost:8082/realms/myspringrustrealm/ |jq .public_key -r 

# Authentificate using login and password
token=`curl -s -d 'client_id=myspringrustclient' -d 'username=springrs' -d 'password=springrs' -d 'grant_type=password' 'http://localhost:8082/realms/myspringrustrealm/protocol/openid-connect/token' |jq .access_token -r`

refreshtoken=`curl -s -d 'client_id=myspringrustclient' -d 'username=springrs' -d 'password=springrs' -d 'grant_type=password' 'http://localhost:8082/realms/myspringrustrealm/protocol/openid-connect/token' |jq .refresh_token -r`


curl -s -d 'client_id=myspringrustclient' -d "refresh_token=${refreshtoken}" -d 'grant_type=refresh_token'  'http://localhost:8082/realms/myspringrustrealm/protocol/openid-connect/token' |jq


# if you provide configure client authentification, you must add the client_secret parameter
token=`curl -s -d 'client_id=myspringrustclient' -d 'username=springrs' -d 'password=springrs' -d 'grant_type=password' -d 'client_secret=hgbxPDD6WWpC1hrjIy7BG5pZeoMbHmLz' 'http://localhost:8082/realms/myspringrustrealm/protocol/openid-connect/token' |jq .access_token -r`

refreshtoken=`curl -s -d 'client_id=myspringrustclient' -d 'username=springrs' -d 'password=springrs' -d 'grant_type=password' -d 'client_secret=hgbxPDD6WWpC1hrjIy7BG5pZeoMbHmLz' 'http://localhost:8082/realms/myspringrustrealm/protocol/openid-connect/token' |jq .refresh_token -r`


# Test protected access
curl "http://localhost:8080/api/user-info" -H "Authorization: Bearer $token"


```




## Restore database

```bash
docker cp ../schema.sql local_pgdb:/tmp/schema.sql
docker exec -t -i  local_pgdb psql -h localhost -p 5432 -U demo demo -f /tmp/schema.sql
```


## Backup database

```bash
docker exec local_pgdb /usr/bin/pg_dump  -h localhost -p 5432 -U demo demo > schema.sql
```

## Create data

```bash
curl -v -X POST http://localhost:8080/user   -H 'Content-Type: application/json'   -d '{"name":"titi","firstname":"titi","age":10}'
```

# Build the project

```bash
cargo build --release
```
# Dev mode

```bash
cargo watch -x run
```




# TODO

- [x] Support pagination
- [X] Add [validation to DTO](https://github.com/AutoWDS/autowds-backend/blob/master/src/views/user.rs)
- [X] Integrate a simple mapstruct to convert dto to entity and entity to dto [based on this idea](https://leapcell.io/blog/java-mapstruct-implemented-in-rust?ref=dailydev)
- [ ] Support authentification and autorisation using openid connect and keycloak
- [ ] Show how to provide integration test using Mock for services and DAO
- [ ] Provide a migration integration for Welds
- [ ] Improve open api generation to simplify development
- [ ] Connect a simple frontend based on JHipster
- [ ] Write a blog post to explain the architecture for Spring developers

