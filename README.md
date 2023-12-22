### Reading plan infrastructure

#### Installation
1. Install the following:
    - `make`
    - `docker`
    - `python3`
2. Create `.env` file near to `env-example` and specify necessary values.
3. `docker compose up -d` to start db service and others.
4. `make migrate` - to apply migrations.
5. `make scrape` - to scrape data from websites and populate DB.
6. `make run-web` - to start Axum web server.