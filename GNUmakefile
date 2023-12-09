MODE := $(if $(MODE),$(MODE),development)
MIGRATION_DIR := $(if $(MIGRATION_DIR),$(MIGRATION_DIR),/tukosmo/src/core/shared/infrastructure/diesel_orm/migration/)

clean:
	rm -Rf target

build:
ifeq ($(MODE), development)
	cargo leptos build
else ifeq ($(MODE), production)
	cargo leptos build --release
endif

dependencies:
	apt install -y wget unzip

external-css:
	rm -f src/core/shared/infrastructure/leptos_ui/layout/external_styles/modern-normalize.scss
	wget -O modern-normalize.scss https://raw.githubusercontent.com/sindresorhus/modern-normalize/main/modern-normalize.css
	mv modern-normalize.scss src/core/shared/infrastructure/leptos_ui/layout/external_styles/modern-normalize.scss
	rm -Rf src/core/shared/infrastructure/leptos_ui/layout/external_styles/bulma
	wget -O bulma.zip https://github.com/jgthms/bulma/releases/download/0.9.4/bulma-0.9.4.zip
	unzip bulma.zip -d src/core/shared/infrastructure/leptos_ui/layout/external_styles
	rm bulma.zip

install: | dependencies clean external-css build

run:
ifeq ($(MODE), development)
	cargo leptos watch
else ifeq ($(MODE), production)
	echo "You must run inside a service"
endif

migration-setup:
	diesel setup --migration-dir $(MIGRATION_DIR)

##########

up:
	docker compose up

up-fresh:
	docker compose down -v
	docker compose up

rebuild:
	docker compose build --force-rm --no-cache

into-psql:
	docker exec -it tukosmo_db psql -U tukosmo -d tukosmo

into-web:
	docker exec -it tukosmo_web bash

migrate:
	docker exec -it tukosmo_web diesel migration run --migration-dir $(MIGRATION_DIR)

migration:
	docker exec -it tukosmo_web diesel migration generate new_migration --migration-dir $(MIGRATION_DIR)

remove-db:
	docker stop tukosmo_db
	docker rm tukosmo_db
