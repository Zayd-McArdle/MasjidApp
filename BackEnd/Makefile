integration-test: unit-test
	cd ./api/masjid_app_api_integration_testing && cargo test --quiet || exit 1
unit-test: docker
	cd ./api/masjid_app_api_library && cargo test --quiet || exit 1
	cd ./api/masjid_app_public_api && cargo test --quiet || exit 1
	cd ./api/masjid_app_admin_manager_api && cargo test --quiet || exit 1
docker:
	docker build -t masjidappdatabase -f ./Database/Dockerfile ./Database
	docker build -t masjidapppublicapi -f ./api/masjid_app_public_api/Dockerfile ./api
	docker build -t masjidappadminmanagerapi -f ./api/masjid_app_admin_manager_api/Dockerfile ./api
