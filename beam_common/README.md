# Rust API client for beam_common

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

For more information, please visit [https://api.beamable.com](https://api.beamable.com)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0
- Package version: 0.1.0
- Build date: 2024-02-15T13:53:15.416346+01:00[Europe/Warsaw]
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `beam_common` and add the following to `Cargo.toml` under `[dependencies]`:

```
beam_common = { path = "./beam_common" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.beamable.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**basic_auth_token_get**](docs/DefaultApi.md#basic_auth_token_get) | **GET** /basic/auth/token | 
*DefaultApi* | [**basic_auth_token_list_get**](docs/DefaultApi.md#basic_auth_token_list_get) | **GET** /basic/auth/token/list | 
*DefaultApi* | [**basic_auth_token_post**](docs/DefaultApi.md#basic_auth_token_post) | **POST** /basic/auth/token | 
*DefaultApi* | [**basic_auth_token_revoke_put**](docs/DefaultApi.md#basic_auth_token_revoke_put) | **PUT** /basic/auth/token/revoke | 


## Documentation For Models

 - [ChallengeSolution](docs/ChallengeSolution.md)
 - [CommonResponse](docs/CommonResponse.md)
 - [ContextInfo](docs/ContextInfo.md)
 - [GetTokenRequest](docs/GetTokenRequest.md)
 - [ListTokenResponse](docs/ListTokenResponse.md)
 - [ListTokenResponseItem](docs/ListTokenResponseItem.md)
 - [ListTokensRequest](docs/ListTokensRequest.md)
 - [RevokeTokenRequest](docs/RevokeTokenRequest.md)
 - [Token](docs/Token.md)
 - [TokenRequestWrapper](docs/TokenRequestWrapper.md)
 - [TokenResponse](docs/TokenResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

support@beamable.com

