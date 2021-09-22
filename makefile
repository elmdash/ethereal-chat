node:
	docker run --rm -it -v `pwd`/browser:/app -u `id -u`:`id -g` -w /app --network host node:16  bash
