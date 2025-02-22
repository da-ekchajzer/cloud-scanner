# CLI options

```sh
List aws instances and their environmental impact (from Boavizta API)

Usage: cloud-scanner-cli [OPTIONS] <COMMAND>

Commands:
  estimate   Get estimation of impacts for a given usage duration
  inventory  List instances and  their average cpu load for the last 5 minutes (no impact data)
  serve      Serve metrics on http://localhost:3000/metrics
  help       Print this message or the help of the given subcommand(s)

Options:
  -a, --aws-region <AWS_REGION>
          AWS region (The default aws profile region is used if not provided)
  -b, --boavizta-api-url <BOAVIZTA_API_URL>
          Optional Boavizta API URL if you want to use your own instance (URL without the trailing slash, e.g. https://api.boavizta.org)
  -t, --filter-tags <FILTER_TAGS>
          Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
  -v, --verbosity...
          Enable logging, use multiple `v`s to increase verbosity
  -m, --as-metrics
          Returns OpenMetrics (Prometheus) instead of json output
  -h, --help
          Print help information
  -V, --version
          Print version information
```

## Experimental feature: block storage

Use the `--include-block-storage` command line flag or parameter to consider block storage (either in inventory or when requesting an estimation of impacts.). This parameter defaults to `false` . This means that by default block storage (volumes) are not counted in the inventory nor in the results.

⚠ In any case, for storage, the impacts of the _use_ phase are _not_ counted. Boavizta API returns only the impacts of the _manufacturing_ phase for HDD and SSD. Furthermore the impacts of storage are likely _are likely overestimated_. They are calculated by taking into account the size of the logical volume. In reality, a volume that is not full may not claim the full space on a physical device, which would result in smaller impacts.

```sh
# Experimental: get impacts of instances and attached storage
cargo run estimate --hours-use-time 1 --include-block-storage --output-verbose-json
```

## Display statistics

Use `-v` will display statistics on std error.

- First output line shows the time spend specifically gathering CPU load of instances.
- Second line shows global statistics:
  - total inventory duration: total time taken to query aws to list instances and get CPU stats
  - impact retrieval duration: time taken to query Boavizta API
  - total execution duration.

```sh
cloud-scanner-cli -v estimate -u 1
# use -- -v with cargo
# cargo  run -- -v estimate -u 1
[...]
cloud_scanner_cli::aws_inventory: Total time spend querying CPU load of instances: 372.153481ms
cloud_scanner_cli: ExecutionStatistics { inventory_duration: 911.526773ms, impact_duration: 398.993816ms, total_duration: 1.310520822s }
[...]
```
