services-run:
	docker compose --env-file ./.env -p english_auction_local up --build -d

services-clean:
	docker compose -p english_auction_local down --rmi local -v --remove-orphans

services-run-test:
	docker compose --env-file ./.env.test -p english_auction_test up --build -d

services-clean-test:
	docker compose -p english_auction_test down --rmi local -v --remove-orphans
