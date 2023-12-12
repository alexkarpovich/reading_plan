include .env

APP_DIR=./reading_plan
SCRAPE_DIR=./scraping
MIGRATIONS_DIR = ${APP_DIR}/migrations
DOT_ENV_PATH = $(abspath .env)

$(info .env file path ${DOT_ENV_PATH})
$(eval export $(shell sed -ne 's/ *#.*$$//; /./ s/=.*$$// p' .env))

.PHONY=run-tg
run-tg:
	cd ${APP_DIR} && env $$(cat ${DOT_ENV_PATH} | xargs ) cargo run --bin telegram

.PHONY=run-web
run-web:
	cd ${APP_DIR} && env $(cat ${DOT_ENV_PATH} | xargs ) cargo run --bin web

.PHONY=migrate
migrate: 
	sqlx migrate run \
		--source ${MIGRATIONS_DIR} \
		--database-url postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:5433/$(POSTGRES_DB)

.PHONY=scrape
scrape:
	cd ${SCRAPE_DIR} && env $$(cat ${DOT_ENV_PATH} | xargs ) python3 -m scrapy crawl bibleonline