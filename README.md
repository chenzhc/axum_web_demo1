curl -X POST localhost:4000/vehice  \
   -H 'Content-Type: application/json'   \
   -d '{"manufacturer":"Tesla","model":"Model Y", "year": 2025 }'

curl -X POST localhost:4000/vehiceQuery  \
    -H "Content-Type: application/x-www-form-urlencoded"  \
    -d "manufacturer=Tesla&model=Model Y&year=2025&first_name=Trevor&last_name=Sullivan"


curl  http://localhost:4000/vehiceQuery  --data-urlencode "manufacturer=Tesla&model=Model Y&year=2025&first_name=Trevor&last_name=Sullivan"


