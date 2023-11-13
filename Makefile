APP_DIR=./reading_plan
DOT_ENV_PATH = $(abspath .env)

$(info .env file path ${DOT_ENV_PATH})

.PHONY=run-tg
run-tg:
	cd ${APP_DIR} && env $$(cat ${DOT_ENV_PATH} | xargs ) cargo run --bin telegram

.PHONY=run-web
run-web:
	cd ${APP_DIR} && env $(cat ${DOT_ENV_PATH} | xargs ) cargo run --bin web
