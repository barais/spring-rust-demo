# Spring RS demo for TAA

This small project shows a backend architecture in Rust that follows the principles and technologies seen in the TAA course at Univ Rennes.
 
# Project setup

## 1. Start the environement setup
 It will start and setup:

- the idendity provider (keycloak)
- the database with the required database and schema
- a mail fakeenv

```bash
cd dev-docker/
docker compose up -d 
```

## 2. Get the public keys for your identity provider

The identity provider will start with 2 users setup:
- *springrs:springrs* with the role **USER**
- *springrsadmin:springrsadmin* with the roles **USER** and **ADMIN**

```bash
# Get public key from keycloak
( echo "-----BEGIN PUBLIC KEY-----" ; curl -s http://localhost:8082/realms/myspringrustrealm/ | jq -r .public_key | fold -w64 ; echo "-----END PUBLIC KEY-----" ) > ../keys/public.key
```

## 3. Test two simple routes

Route 1. 
```bash
curl "http://localhost:8080/api/hello"
```

Route 2. 
```bash
curl "http://localhost:8080/api/user/all"
```

## 4. Authenticate using login and password and test a route

Authenticate using login and password 

```bash
eval "$(resp=$(curl -s -d 'client_id=myspringrustclient' -d 'username=springrs' -d 'password=springrs' -d 'grant_type=password' 'http://localhost:8082/realms/myspringrustrealm/protocol/openid-connect/token') && printf 'export token=%q\nexport refreshtoken=%q\n' "$(jq -r .access_token <<<"$resp")" "$(jq -r .refresh_token <<<"$resp")")"
```


test a protected route

```bash
curl "http://localhost:8080/api/user-info" -H "Authorization: Bearer $token"
```


Refresh **access token** using **refresh token**.


```bash
curl -s -d 'client_id=myspringrustclient' -d "refresh_token=${refreshtoken}" -d 'grant_type=refresh_token'  'http://localhost:8082/realms/myspringrustrealm/protocol/openid-connect/token' |jq
```




## 4 (bis) :paperclip: if you provide configure client authentification, you must add the client_secret parameter


```bash
token=`curl -s -d 'client_id=myspringrustclient' -d 'username=springrs' -d 'password=springrs' -d 'grant_type=password' -d 'client_secret=hgbxPDD6WWpC1hrjIy7BG5pZeoMbHmLz' 'http://localhost:8082/realms/myspringrustrealm/protocol/openid-connect/token' |jq .access_token -r`

refreshtoken=`curl -s -d 'client_id=myspringrustclient' -d 'username=springrs' -d 'password=springrs' -d 'grant_type=password' -d 'client_secret=hgbxPDD6WWpC1hrjIy7BG5pZeoMbHmLz' 'http://localhost:8082/realms/myspringrustrealm/protocol/openid-connect/token' |jq .refresh_token -r`
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

# Backup keycloak realm with users

```bash
docker commit keycloak barais/springrskeycloak
docker run -ti --entrypoint=sh -v /tmp:/tmp barais/springrskeycloak
# in the container
/opt/keycloak/bin/kc.sh export --file=/tmp/export.json --realm=myspringrustrealm --users=same_file --optimized
# your export will be in /tmp/export.json
```
 
# TODO

- [x] Support pagination
- [X] Add [validation to DTO](https://github.com/AutoWDS/autowds-backend/blob/master/src/views/user.rs)
- [X] Integrate a simple mapstruct to convert dto to entity and entity to dto [based on this idea](https://leapcell.io/blog/java-mapstruct-implemented-in-rust?ref=dailydev)
- [X] Support authentification and autorisation using openid connect and keycloak
- [ ] Show how to provide integration test using Mock for services and DAO
- [ ] Provide a migration integration for Welds
- [X] Improve open api generation to simplify development
- [ ] Connect a simple frontend based on JHipster
- [ ] Write a blog post to explain the architecture for Spring developers

