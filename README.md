# AI Forge- Bevy
[![CI](https://github.com/beamable/bevy-ai-forge/actions/workflows/ci.yaml/badge.svg)](https://github.com/beamable/bevy-ai-forge/actions/workflows/ci.yaml)

This project is POC of the possible integration of Beamable into Bevy Engine. Current implementation is pretty simple and relies strongly on code generation, but still serves as a proof of possible quick integration into new engine by using OpenAPI and [CLI Beamable tools](https://docs.beamable.com/docs/cli-guide-getting-started). it includes also custom Federated Identity and Beam Standalone Microservices integration.

Available live here: https://beamable.github.io/bevy-ai-forge/

## Forge Service configuration

It does require realm configuration describing API keys:
- `ForgeService.openApiKey`- OpenAI API key
<!-- - `ForgeService.scenarioKey`- API key to the https://app.scenario.com/ But it is not used in current version. -->

## Getting API from the microservice

It requires installed Dotnet 8, beam tools and Docker.

Grab the result from command below and save it to `microservice_api.json`:

`beam get /basic/1714908866696208.DE_1714908866696209.micro_ForgeService/admin/Docs`

`docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i /local/microservice_api.json -g rust -c /local/microservice_config.json -o /local/beam_microservice`

