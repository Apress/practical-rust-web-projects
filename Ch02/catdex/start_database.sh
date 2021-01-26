# Run this the first time:
# docker run --name catdex-db -e POSTGRES_PASSWORD=mypassword -p 5432:5432 -d postgres:12.3-alpine

# From the second time, run this instead:
docker start catdex-db

echo "Run this to set the databse URL:"
echo "export DATABASE_URL=postgres://postgres:mypassword@localhost"
