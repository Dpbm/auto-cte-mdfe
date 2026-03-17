.PHONY: start-docker stop-docker build-windows

stop-docker:
	@echo "Stopping docker compose..."
	docker compose down

start-docker: stop-docker
	@echo "Starting docker compose..."
	docker compose up -d --build --force-recreate

build-windows:
	@echo "Building API..."
	cd ./rateio-api/ && cargo build --target x86_64-pc-windows-gnu --release
	@echo "Building frontend.."
	cd ./frontend/ && VITE_API_BASE_URL="http://localhost:4000" npm run build
	@echo "Packing..."
	rm -rf ./package/
	mkdir -p ./package
	mv ./frontend/dist/ ./package/
	mv ./rateio-api/target/x86_64-pc-windows-gnu/release/rateio-api.exe ./package/ 
	zip -r package.zip package
