# Spring RS demo for TAA

This small project shows a backend architecture in Rust that follows the principles and technologies seen in the TAA course at Univ Rennes.
 
# Project setup

## Start the database

```bash
cd postgres-docker/
docker compose up -d 
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

# Build the project

```bash
cargo build --release
```


# TODO

- [ ] Show how to provide integration test using Mock for services and DAO
- [ ] Support authentification and autorisation using openid connect and keycloak
- [ ] Improve open api generation to simplify development
- [ ] Provide a migration integration for Welds
- [ ] Support pagination
- [ ] Connect a simple frontend based on JHipster
- [ ] Write a blog post to explain the architecture for Spring developers