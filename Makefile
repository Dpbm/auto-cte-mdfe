.PHONY: start-docker stop-docker

stop-docker:
	@echo "Stopping docker compose..."
	docker compose down

start-docker: stop-docker
	@echo "Starting docker compose..."
	docker compose up -d --build --force-recreate


