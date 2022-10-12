DOCKER_OPT=-v ${PWD}/../:/SRC --env CARGO_HOME=/SRC/target/.cargo
DOCKER_USER=`id -u`:`id -g`

build:
	docker build -t entropy-builder -f Dockerfile .
	docker run -t $(DOCKER_OPT) -u ${DOCKER_USER} --env CARGODEB_OPT="-p dirty-entropy-rs" entropy-builder
