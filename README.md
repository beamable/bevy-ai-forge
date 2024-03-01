# AI Forge- Bevy



## Forge Service configuration

It does require realm configuration describing API keys:
- `ForgeService.openApiKey`- OpenAI API key
- `ForgeService.scenarioKey`- API key to the https://app.scenario.com/

## Getting API from the microservice

Grab the result from command below and save it to `oapi/microservice_api.json`:

`beam get /basic/1714908866696208.DE_1714908866696209.micro_ForgeService/admin/Docs`

`openapi-generator-cli generate -i oapi/microservice_api.json -g rust -c microservice_config.json -o beam_microservice`

## Getting API from the Beamable backend

`beam oapi download --output oapi --filter "auth,inventory"`

// TODO Work with multiple files

`openapi-generator-cli generate -i oapi/basic_auth.json -g rust -c beam_common_config.json -o beam_common --skip-validate-spec`

`openapi-generator-cli generate -i oapi/object_inventory.json -g rust -c beam_common_inventory_config.json -o beam_common_inventory --skip-validate-spec`
