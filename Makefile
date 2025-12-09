.PHONY: build-dockerfile clean-docker

clean-docker:
	docker image rm $(NAME) -f

build-dockerfile: clean-docker
	docker build --no-cache -t $(NAME) --file $(NAME).Dockerfile .
