// Copyright (c) SimpleStaking, Viable Systems and Tezedge Contributors
// SPDX-License-Identifier: MIT

use clap::{App, Arg, SubCommand};
use url::Url;

pub struct SequentialTestEnv {
    pub cycles: i32,
    pub nodes: Vec<Url>,
}

impl SequentialTestEnv {
    pub fn from_args(args: &clap::ArgMatches) -> Self {
        let nodes: Vec<Url> = if let Some(nodes) = args.values_of("nodes") {
            nodes
                .map(|v| {
                    v.parse()
                        .expect("Provided value cannot be converted into valid url")
                })
                .collect()
        } else {
            panic!("No nodes provided in the --nodes arg")
        };

        SequentialTestEnv {
            cycles: args
                .value_of("cycles")
                .unwrap_or("")
                .parse::<i32>()
                .expect("Provided value cannot be converted into valid i32"),
            nodes,
        }
    }
}

pub struct BootstrapEnv {
    pub level: i32,
    pub nodes: Vec<Url>,
}

impl BootstrapEnv {
    pub fn from_args(args: &clap::ArgMatches) -> Self {
        let nodes: Vec<Url> = if let Some(nodes) = args.values_of("nodes") {
            nodes
                .map(|v| {
                    v.parse()
                        .expect("Provided value cannot be converted into valid url")
                })
                .collect()
        } else {
            panic!("No nodes provided in the --nodes arg")
        };

        BootstrapEnv {
            level: args
                .value_of("level")
                .unwrap_or("")
                .parse::<i32>()
                .expect("Provided value cannot be converted into valid i32"),
            nodes,
        }
    }
}

pub struct RpcPerformanceTestEnv {
    pub ocaml_node: Url,
    pub tezedge_new_node: Url,
    pub tezedge_old_node: Option<Url>,
    pub url_file: String,
    pub wrk_test_duration: u64,
    pub max_latency_threshold: f32,
    pub throughput_threshold: f32,
    pub latency_no_fail: bool,
    pub throughput_no_fail: bool,
}

pub struct RpcLatencyTestEnv {
    pub ocaml_node: Url,
    pub tezedge_new_node: Url,
    pub tezedge_old_node: Option<Url>,
    pub url_file: String,
    pub wrk_test_duration: u64,
    pub wrk_request_rate: u64,
}

impl RpcPerformanceTestEnv {
    pub fn from_args(args: &clap::ArgMatches) -> Self {
        RpcPerformanceTestEnv {
            ocaml_node: args
                .value_of("ocaml-node")
                .unwrap_or("")
                .parse()
                .expect("Provided value cannot be converted into valid url"),
            tezedge_new_node: args
                .value_of("tezedge-new-node")
                .unwrap_or("")
                .parse()
                .expect("Provided value cannot be converted into valid url"),
            tezedge_old_node: {
                if args.is_present("tezedge-old-node") {
                    Some(
                        args.value_of("tezedge-old-node")
                            .unwrap_or("")
                            .parse::<Url>()
                            .expect("Provided value cannot be converted into valid url"),
                    )
                } else {
                    None
                }
            },
            url_file: args
                .value_of_lossy("url-file")
                .expect("Missing URL file parameter")
                .into_owned(),
            wrk_test_duration: args
                .value_of("wrk-test-duration")
                .unwrap_or("")
                .parse::<u64>()
                .expect("Provided value cannot be converted into valid u64"),
            max_latency_threshold: args
                .value_of("max-latency-threshold")
                .unwrap_or("")
                .parse::<f32>()
                .expect("Provided value cannot be converted into valid u64")
                * 0.01,
            throughput_threshold: args
                .value_of("throughput-threshold")
                .unwrap_or("")
                .parse::<f32>()
                .expect("Provided value cannot be converted into valid u64")
                * 0.01,
            latency_no_fail: args
                .is_present("latency-no-fail"),
            throughput_no_fail: args
                .is_present("throughput-no-fail"),
        }
    }
}

impl RpcLatencyTestEnv {
    pub fn from_args(args: &clap::ArgMatches) -> Self {
        RpcLatencyTestEnv {
            ocaml_node: args
                .value_of("ocaml-node")
                .unwrap_or("")
                .parse()
                .expect("Provided value cannot be converted into valid url"),
            tezedge_new_node: args
                .value_of("tezedge-new-node")
                .unwrap_or("")
                .parse()
                .expect("Provided value cannot be converted into valid url"),
            tezedge_old_node: {
                if args.is_present("tezedge-old-node") {
                    Some(
                        args.value_of("tezedge-old-node")
                            .unwrap_or("")
                            .parse::<Url>()
                            .expect("Provided value cannot be converted into valid url"),
                    )
                } else {
                    None
                }
            },
            url_file: args
                .value_of_lossy("url-file")
                .expect("Missing URL file parameter")
                .into_owned(),
            wrk_test_duration: args
                .value_of("wrk-test-duration")
                .unwrap_or("")
                .parse::<u64>()
                .expect("Provided value cannot be converted into valid u64"),
            wrk_request_rate: args
                .value_of("wrk-request-rate")
                .unwrap_or("")
                .parse::<u64>()
                .expect("Provided value cannot be converted into valid u64"),
        }
    }
}

pub struct IndexerTestEnv {
    pub level: i32,
    pub ocaml_node: Url,
    pub tezedge_node: Url,
    pub tezedge_indexer: Url,
    pub ocaml_indexer: Url,
}

impl IndexerTestEnv {
    pub fn from_args(args: &clap::ArgMatches) -> Self {
        IndexerTestEnv {
            level: args
                .value_of("level")
                .unwrap_or("")
                .parse::<i32>()
                .expect("Provided value cannot be converted into valid i32"),
            ocaml_node: args
                .value_of("ocaml-node")
                .unwrap_or("")
                .parse()
                .expect("Provided value cannot be converted into valid url"),
            tezedge_node: args
                .value_of("tezedge-node")
                .unwrap_or("")
                .parse()
                .expect("Provided value cannot be converted into valid url"),
            tezedge_indexer: args
                .value_of("tezedge-indexer")
                .unwrap_or("")
                .parse()
                .expect("Provided value cannot be converted into valid url"),
            ocaml_indexer: args
                .value_of("ocaml-indexer")
                .unwrap_or("")
                .parse()
                .expect("Provided value cannot be converted into valid url"),
        }
    }
}

pub fn bootstrap_app() -> App<'static, 'static> {
    let app = App::new("CI bootstrap and test app")
        .version("0.1.0")
        .author("Adrian Nagy")
        .about("CI bootstraping and testing app")
        .setting(clap::AppSettings::AllArgsOverrideSelf)
        .subcommand(
            SubCommand::with_name("performance-test")
                .about("Performance test using wrk")
                .setting(clap::AppSettings::AllArgsOverrideSelf)
                .arg(
                    Arg::with_name("ocaml-node")
                    .long("ocaml-node")
                    .required(true)
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Ocaml node url")
                )
                .arg(
                    Arg::with_name("tezedge-new-node")
                    .long("tezedge-new-node")
                    .required(true)
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Tezedge node url - with updated code from the pull request")
                )
                .arg(
                    Arg::with_name("tezedge-old-node")
                    .long("tezedge-old-node")
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Tezedge node url - with old code from the target branch of the pull request")
                )
                .arg(
                    Arg::with_name("url-file")
                    .long("url-file")
                    .required(true)
                    .takes_value(true)
                    .value_name("FILE")
                    .help("File containing a list of URLs to test")
                )
                .arg(
                    Arg::with_name("wrk-test-duration")
                    .long("wrk-test-duration")
                    .takes_value(true)
                    .value_name("NUM")
                    .help("Duration of the individual tests")
                )
                .arg(
                    Arg::with_name("max-latency-threshold")
                    .long("max-latency-threshold")
                    .takes_value(true)
                    .value_name("NUM")
                    .help("Maximum tail latency delta between two node versions allowed in percentages")
                )
                .arg(
                    Arg::with_name("throughput-threshold")
                    .long("throughput-threshold")
                    .takes_value(true)
                    .value_name("NUM")
                    .help("Maximum throughput delta between two node versions allowed in percentages")
                )
                .arg(
                    Arg::with_name("latency-no-fail")
                    .long("latency-no-fail")
                    .takes_value(false)
                    .help("Do not fail the test if max latency exceeds the threshold")
                )
                .arg(
                    Arg::with_name("throughput-no-fail")
                    .long("throughput-no-fail")
                    .takes_value(false)
                    .help("Do not fail the test if max latency exceeds the threshold")
                )
            )
        .subcommand(
            SubCommand::with_name("latency-test")
                .about("Latency test using wrk2")
                .setting(clap::AppSettings::AllArgsOverrideSelf)
                .arg(
                    Arg::with_name("ocaml-node")
                    .long("ocaml-node")
                    .required(true)
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Ocaml node url")
                )
                .arg(
                    Arg::with_name("tezedge-new-node")
                    .long("tezedge-new-node")
                    .required(true)
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Tezedge node url - with updated code from the pull request")
                )
                .arg(
                    Arg::with_name("tezedge-old-node")
                    .long("tezedge-old-node")
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Tezedge node url - with old code from the target branch of the pull request")
                )
                .arg(
                    Arg::with_name("url-file")
                    .long("url-file")
                    .required(true)
                    .takes_value(true)
                    .value_name("FILE")
                    .help("File containing a list of URLs to test")
                )
                .arg(
                    Arg::with_name("wrk-test-duration")
                    .long("wrk-test-duration")
                    .takes_value(true)
                    .value_name("NUM")
                    .help("Duration of the individual tests")
                )
                .arg(
                    Arg::with_name("wrk-request-rate")
                    .long("wrk-request-rate")
                    .takes_value(true)
                    .value_name("NUM")
                    .help("Request rate for the individual test")
                )
            )
        .subcommand(
            SubCommand::with_name("indexer-test")
            .about("Indexer correctness test")
            .setting(clap::AppSettings::AllArgsOverrideSelf)
            .arg(
                Arg::with_name("level")
                .long("level")
                .takes_value(true)
                .value_name("NUM")
                .help("Block level which is used in the test as an upper bound")
            )
            .arg(
                Arg::with_name("ocaml-node")
                .long("ocaml-node")
                .takes_value(true)
                .value_name("STRING")
                .help("Ocaml node url")
            )
            .arg(
                Arg::with_name("tezedge-node")
                .long("tezedge-node")
                .takes_value(true)
                .value_name("STRING")
                .help("Tezedge node url - with updated code from the pull request")
            )
            .arg(
                Arg::with_name("tezedge-indexer")
                .long("tezedge-indexer")
                .takes_value(true)
                .value_name("STRING")
                .help("Indexer url connected to the tezedge node")
            )
            .arg(
                Arg::with_name("ocaml-indexer")
                .long("ocaml-indexer")
                .takes_value(true)
                .value_name("STRING")
                .help("Indexer url connected to the ocaml node")
            )
        )
        .subcommand(
            SubCommand::with_name("bootstrap")
            .about("Checks and waits for two nodes to be bootstrapped to the same level")
            .setting(clap::AppSettings::AllArgsOverrideSelf)
            .arg(
                Arg::with_name("level")
                .long("level")
                .takes_value(true)
                .value_name("NUM")
                .help("Block level which is used in the test as an upper bound")
            )
            .arg(
                Arg::with_name("nodes")
                .long("nodes")
                .takes_value(true)
                .multiple(true)
                .min_values(1)
                .value_name("STRING")
                .help("Node urls to be bootstrapped")
            )
        )
        .subcommand(
            SubCommand::with_name("sequential-test")
            .about("Checks and waits for two nodes to be bootstrapped to the same level")
            .setting(clap::AppSettings::AllArgsOverrideSelf)
            .arg(
                Arg::with_name("cycles")
                .long("cycles")
                .takes_value(true)
                .value_name("NUM")
                .help("Number of cycles to test")
            )
            .arg(
                Arg::with_name("nodes")
                .long("nodes")
                .takes_value(true)
                .multiple(true)
                .min_values(2)
                .max_values(2)
                .value_name("STRING")
                .help("Node urls to be bootstrapped")
            )
        );
    app
}
