.PHONY: docker database publicapi adminmanagerapi

docker: database publicapi adminmanagerapi

database:
	docker build -t database -f ./Database/Dockerfile ./Database

publicapi:
	docker build -t publicapi -f ./api/masjid_app_public_api/Dockerfile ./api

adminmanagerapi:
	docker build -t adminmanagerapi -f ./api/masjid_app_admin_manager_api/Dockerfile ./api