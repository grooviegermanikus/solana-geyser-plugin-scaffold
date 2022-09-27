# The Simplest Geyser Plugin Ever

### What's a Solana Geyser Plugin?
A Solana Validator can _"leak"_ accounts and transactions data outside the validator.
This flow of data is achieved through the [The Geyser Plugin Interface.](https://docs.rs/solana-geyser-plugin-interface/latest/solana_geyser_plugin_interface/geyser_plugin_interface/trait.GeyserPlugin.html)

An external library can _plug_ into that interface by implementing the necessary functions and thus listen for accounts and transactions streams.

That dynamic library is provided to the validator at runtime. The validator can then open that library and call the implemented _callbacks_ or _hooks_ with accounts and transactions data.

The library can then feed on these data and take further actions, such as logging, inserting the data into a DB or a consumer/producer system, etc.

### Solana Geyser Plugin Scaffold
This is the simplest geyser plugin implementation you will encounter, all it does is log every calls from the plugin manager to our plugin scaffold.
This is a good start to familiarize yourself with the plugin workflow, and most importantly debug.

/!\ The code is for educational purpose, in a production setting, you would want to remove any fancy logs and do the minimum work possible in the _hooks_ by leveraging threads, different process or external services, etc...

### Try It!

Run:
```bash
./scripts/run.sh
```

How do I know if it works?
```bash
./scripts/logs.sh
```

Plugin or validator crashing?
```bash
./scripts/check_errors.sh
```

### Geyser Plugin Config

The dynamic library path is provided to the validator using the `--geyser-plugin-config` parameter.
For example when using the test validator:
```bash
solana-test-validator --geyser-plugin-config config/geyser-plugin-config-mac.json
# or use ./scripts/run.sh
```

At a minimum the config file should:
- Be in JSON format
- Contain the path to your geyser plugin dynamic library _.so_ or (_dylib_ on mac)

For example:
```json
{
    "libpath": "libsolana_geyser_plugin_scaffold.dylib"
}
```
> Of course your production validator won't run on mac, so update the path accordingly and use the .so version.

Additionally, at runtime the Solana Plugin Manager will pass back the path to that config file to your plugin. The `config_file` path will be provided on the [on_load(&mut self, config_file: &str)](https://docs.rs/solana-geyser-plugin-interface/latest/solana_geyser_plugin_interface/geyser_plugin_interface/trait.GeyserPlugin.html#method.on_load) lifecycle event.
So you can add any additional config you think your plugin might need. And parse it when your plugin gets loaded.

---

# Going Further

### Read More On The Plugin Manager
- [The Geyser Plugin Manager: The Guy Calling Your Plugin](https://github.com/solana-labs/solana/tree/master/geyser-plugin-manager)

### What's Next?
The starter project might be simple, but the most important for you is to be able to debug and see the logs.

Indeed, if you can get the data through, what's next is really up to you. The question is what will you do of these data?
- Will you forward this log into a log service?
- Will you insert this into a DB? And create specific indexing for your own needs?
- Will you build a whole consumer/producer system with Kafka and other queuing pipelines?
- The sky is the limit, go ship something! 🚀

### A Note On Performance
Moving forward, please make sure to do the minimum of work into the trait callbacks! And not just the callbacks, but any synchronous execution paths that originates from them.

Indeed your plugin is running part of the validator, and the validator is ...extremly busy! You need to make sure to return as quickly as possible from the callbacks and do the minimum of work. From there, there are multiple strategies:
- Leverage threads where possible.
- Dispatch the hard work into an external process, or even outside the validator's machine.
- Take advantage of a queuing system, to scale and multiply the possibility around your data pipeline.

You might need one of the above solution, or combine all of them. The answer lies into your needs, your own infrastructure, and your team size.

### Examples Plugin Implementations
- [A PostgreSQL Plugin](https://github.com/solana-labs/solana-accountsdb-plugin-postgres)
- [A Plugin Sending to a gRPC Service](https://github.com/ckamm/solana-accountsdb-connector)
- [A RabbitMQ Producer Plugin](https://github.com/holaplex/indexer-geyser-plugin)
- [A Complete Architecture Around The Geyser Plugin](https://github.com/holaplex/indexer)
- [A Kafka Producer Plugin](https://github.com/Blockdaemon/solana-accountsdb-plugin-kafka)
- [An Amazon SQS Plugin](https://github.com/rpcpool/solana-accountsdb-sqs)
- [A Google BigTable Plugin](https://github.com/lijunwangs/solana-accountsdb-plugin-bigtable)
- [A RPC server serving requests from a PostgreSQL database](https://github.com/lijunwangs/solana-postgres-rpc-server)
