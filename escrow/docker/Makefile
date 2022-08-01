services-run:
	docker compose --env-file ./.env -p escrow_local up --build -d

services-clean:
	docker compose -p escrow_local down --rmi local -v --remove-orphans

services-run-test:
	docker compose --env-file ./.env.test -p escrow_test up --build -d

services-clean-test:
	docker compose -p escrow_test down --rmi local -v --remove-orphans
