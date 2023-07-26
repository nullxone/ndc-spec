# Testing

Testing tools are provided in the specification repository to aid in the development of connectors.

## `ndc-test`

The `ndc-test` executable performs basic validation of the data returned by the capabilities and schema endpoints, and performs some basic queries.

To test a connector, provide its endpoint to `ndc-test` on the command line:

```sh
ndc-test --endpoint <ENDPOINT>
```

For example, running the reference connector and passing its URL to `ndc-test`, we will see that it issues test queries against the `articles` and `authors` tables:

```text
ndc-test --endpoint http://localhost:8100
Fetching /capabilities
Validating capabilities
Fetching /schema
Validating schema
Validating object_types
Validating tables
Validating table articles
Validating columns
Validating table authors
Validating columns
Validating commands
Validating command upsert_article
Testing /query
Testing simple queries
Querying table articles
Querying table authors
Testing aggregate queries
Querying table articles
Querying table authors
```

However, `ndc-test` cannot validate the entire schema. For example, it will not issue queries against the `articles_by_author` table, because it does not have any way to synthesize inputs for its required table argument.