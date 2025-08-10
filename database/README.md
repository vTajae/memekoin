
Useful scripts:
docker exec local_postgres_db psql -U leptos_user -d postgres -c "DROP DATABASE IF EXISTS leptos_db; CREATE DATABASE leptos_db;"


Refull: 
type "fullSchemaa_.sql" | docker exec -i local_postgres_db psql -U leptos_user -d leptos_db -v ON_ERROR_STOP=1 -f -