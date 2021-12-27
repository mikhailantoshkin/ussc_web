# UDV/USSC junior python dev test task

This is a reference implementation of UDV/USSC junior python developer test task but written in Rust.

## Task
Crate web server for currency conversion. It should provide two endpoints:
### Conversion endpoint
`/convert?from=USD&to=RUB&amount=69` should return Json response with value of 69 USD converted into RUB

### Database endpoint
`/database?merge=true` should receive Json body with a list of currencies and their conversion rates
and save it to DB. If query parameter `merge` is set to `false` the existing data should be purged.

## Restrictions
- aiohttp (for python)
- redis
- use docker for web server and db (optional)

__Note__: everything not specified is left for programmers decision

# Implementation details
Currencies rates are relative to some arbitrary currency. The user is responsible for data consistency


# Running 
Start server:
```
docker-compose up -d
```
Post some data:
```
curl -X POST 'http://localhost:1234/database?merge=false' \ 
    --data-raw '[{"currency": "RUB", "rate": 12}, {"currency": "USD", "rate": 1}]' \
    -H 'Content-Type: application/json'
```

Convert currencies:
```
 curl 'http://localhost:1234/convert?from=USD&to=RUB&amount=24'
```