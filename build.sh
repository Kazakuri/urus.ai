docker build --target urusai -t registry.tenka.dev/urusai . && docker push registry.tenka.dev/urusai
docker build --target urusai_nginx -t registry.tenka.dev/urusai_nginx . && docker push registry.tenka.dev/urusai_nginx
docker build --target urusai_migrations -t registry.tenka.dev/urusai_migrations . && docker push registry.tenka.dev/urusai_migrations