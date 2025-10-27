docker: integration-test
	docker build -t database -f ./Database/Dockerfile ./Database
	docker build -t publicapi -f ./api/masjid_app_public_api/Dockerfile ./api
	docker build -t adminmanagerapi -f ./api/masjid_app_admin_manager_api/Dockerfile ./api
integration-test: unit-test
	cd ./api/masjid_app_api_integration_testing && cargo test --quiet || exit 1
unit-test:
	cd ./api/masjid_app_api_library && cargo test --quiet || exit 1
	cd ./api/masjid_app_public_api && cargo test --quiet || exit 1
	cd ./api/masjid_app_admin_manager_api && cargo test --quiet || exit 1
