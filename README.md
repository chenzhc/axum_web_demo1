curl -X POST localhost:4000/vehice  \
   -H 'Content-Type: application/json'   \
   -d '{"manufacturer":"Tesla","model":"Model Y", "year": 2025 }'

curl -X POST localhost:4000/vehiceQuery  \
    -H "Content-Type: application/x-www-form-urlencoded"  \
    -d "manufacturer=Tesla&model=Model Y&year=2025"


