# ConfigLoader
Config loading for services withing this workspace.

## Usage
Will load configuration under the `$project-name/resources/` folder.
It loads `config.yaml` then `config_$env` then environment variables.

## Env variables
Env variables are read as `config.key` from `CONFIF_KEY`.