# Get the workspace url without https://
export WORKSPACE_HOST := $(shell echo $$GITPOD_WORKSPACE_URL | cut -c9- | rev | rev)
# Get local urls or gitpod urls
export DIRECTUS_PG_URL := $(if $(GITPOD_WORKSPACE_URL),https://8055-$(WORKSPACE_HOST),http://localhost:8055)
export DIRECTUS_MYSQL_URL := $(if $(GITPOD_WORKSPACE_URL),https://8056-$(WORKSPACE_HOST),http://localhost:8056)

# HELP
# Display target ## comments as help: https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.PHONY: help
help: ## This help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: start-debug
start-debug: # start apps and show docker logs
	docker compose -f database/docker-compose.yaml up

.PHONY: start-barebones
start-barebones:
	docker compose -f database/docker-compose.yaml up -d

.PHONY: start
start: ## Start running databases
	@make start-barebones
	@clear
	@echo "Running databases \033[32m/packages/**\033[0m"
	@echo "⚠️ On Gitpod? These URLs are \033[31mpublicly\033[0m available as long as this workspace is running!\n"
	@echo "Not on Gitpod? Https is not setup."
	@echo "\033[34mDirectus Postgres:\033[0m ${DIRECTUS_PG_URL}"
	@echo "\033[34mDirectus Mysql:\033[0m ${DIRECTUS_MYSQL_URL}"
	@echo "Login for Directus: admin@example.com, password: admin\n"

.PHONY: stop
stop: ## Stop directus and dev server
	@docker compose -f database/docker-compose.yaml down
	@echo "Databases stopped."

.PHONY: reset
reset: ## Prune volumes and clean built packages and examples
	@docker volume prune -f

.PHONY: ssh
ssh: ## Connect to gitpod env with SSH.
ifeq ($(GITPOD_WORKSPACE_URL),)
	@echo "Failed, because you're not using gitpod."
	@echo "This command would otherwise return a shell command to connect to the gitpod env from your local machine."
else
	@$(eval HOST_URL := $(shell echo "${GITPOD_WORKSPACE_URL}" | sed 's#.*${GITPOD_WORKSPACE_ID}\(\)#\1#'))
	@echo "Paste the next line into your local shell (SSH key required):"
	@echo "ssh '${GITPOD_WORKSPACE_ID}@${GITPOD_WORKSPACE_ID}.ssh${HOST_URL}'"
	@echo "\nIf you're having problems, see: https://www.gitpod.io/docs/references/ides-and-editors/command-line\n"
endif

.DEFAULT_GOAL := help
