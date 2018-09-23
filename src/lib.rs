#[macro_export]
macro_rules! make_config {
    (
        Features {
            $(
                $feature:ident = $feature_default:expr
            )*
        }
        TargetedFeatures {
            $(
                $targeted:ident = $targeted_default:expr
            )*
        }
    ) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct FeatureConfig {
            flags: u8,
        }

        #[allow(dead_code)]
        #[derive(Debug)]
        struct TargetedFeatureConfig {
            flags: u8,
            targets: Option<Vec<String>>
        }

        #[allow(dead_code)]
        #[derive(Debug)]
        struct Config {
            $(
                $feature: FeatureConfig,
            )*
            $(
                $targeted: TargetedFeatureConfig,
            )*
        }

        static ENABLED: u8 =    0b0001;
        static TRACE: u8 =      0b0010;
        static TEST_TRACE: u8 = 0b0100;
        static DUMP: u8 =       0b1000;

        impl Config {
            fn new() -> Config {
                Config {
                    $(
                        $feature: FeatureConfig {
                            flags: $feature_default as u8,
                        },
                    )*
                    $(
                        $targeted: TargetedFeatureConfig {
                            flags: $targeted_default as u8,
                            targets: None
                        },
                    )*
                }
            }

            fn new_from_args<T: AsRef<str>>(args: &[T]) -> Result<(Config, Vec<String>), String> {
                let mut config = Config::new();
                let mut remaining = Vec::new();
                for arg in args.into_iter().map(|a| a.as_ref()) {
                    if arg.starts_with("-trace=") {
                        let trace = &arg["-trace=".len()..];
                        $(
                            if trace == stringify!($feature) {
                                config.$feature.flags |= TRACE;
                                continue;
                            }
                        )*

                        return Err(format!("Invalid trace argument: {}", trace));
                    }

                    if arg.starts_with("-testtrace=") {
                        let test_trace = &arg["-testtrace=".len()..];
                        $(
                            if test_trace == stringify!($feature) {
                                config.$feature.flags |= TEST_TRACE;
                                continue;
                            }
                        )*

                        return Err(format!("Invalid testtrace argument: {}", test_trace));
                    }

                    if arg.starts_with("-dump=") {
                        let dump = &arg["-dump=".len()..];
                        $(
                            if dump == stringify!($feature) {
                                config.$feature.flags |= DUMP;
                                continue;
                            }
                        )*

                        return Err(format!("Invalid dump argument: {}", dump));
                    }

                    if arg.starts_with("-") {
                        let feature = &arg["-".len()..];
                        $(
                            if feature == stringify!($feature) {
                                config.$feature.flags |= ENABLED;
                                continue;
                            }
                        )*

                        return Err(format!("Invalid feature: {}", feature));
                    }

                    remaining.push(arg.to_string());
                }

                $(
                    if config.$feature.flags != 0 && config.$feature.flags & ENABLED == 0 {
                        return Err(format!("Can't trace, testtrace, or dump disabled feature {}", stringify!($feature)));
                    }
                )*

                Ok((config, remaining))
            }
        }

        macro_rules! enabled {
            ($config:expr, $prop:ident) => {
                $config.$prop.flags & ENABLED == ENABLED
            };
            ($config:expr, $prop:ident, $target:expr) => {
                if $config.$prop.flags & ENABLED == ENABLED {
                    if let Some(targets) = $config.$props.targets {
                        targets.contains($target)
                    }

                    true
                }

                false
            };
        }

        macro_rules! trace {
            ($config:expr, $prop:ident, $msg:expr) => {
                if $config.$prop.flags & TRACE == TRACE {
                    println!($msg);
                }
            };
            ($config:expr, $prop:ident, $target:expr, $msg:expr) => {
                if $config.$prop.flags & TRACE == TRACE {
                    if let Some(ref targets) = $config.$prop.targets {
                        if targets.contains($target) {
                            println!($msg);
                        }
                    } else {
                        println!($msg);
                    }
                }
            };
        }

        macro_rules! test_trace {
            ($config:expr, $prop:ident, $msg:expr) => {
                if $config.$prop.flags & TEST_TRACE == TEST_TRACE {
                    println!($msg);
                }
            };
            ($config:expr, $prop:ident, $target:expr, $msg:expr) => {
                if $config.$prop.flags & TEST_TRACE == TEST_TRACE {
                    if let Some(ref targets) = $config.$prop.targets {
                        if targets.contains($target) {
                            println!($msg);
                        }
                    } else {
                        println!($msg);
                    }
                }
            };
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic() {
        make_config! {
            Features {
                a = false
                b = false
                d = true
            }
            TargetedFeatures {
                c = true
            }
        };
        let (config, remaining) = Config::new_from_args(&vec!["-a", "extra1", "-trace=d", "-dump=d", "extra2"]).unwrap();

        assert_eq!(enabled!(config, a), true);
        assert_eq!(enabled!(config, b), false);
        assert_eq!(enabled!(config, c), true);
        assert_eq!(enabled!(config, d), true);
        assert_eq!(remaining, vec!["extra1".to_string(), "extra2".to_string()]);

        assert_eq!(config.a.flags, 0b1);
        assert_eq!(config.b.flags, 0b0);
        assert_eq!(config.c.flags, 0b1);
        assert_eq!(config.d.flags, 0b1011);
    }

    #[test]
    fn parse_failure() {
        make_config! {
            Features {
                a = false
                b = false
                d = true
            }
            TargetedFeatures {
                c = true
            }
        };

        Config::new_from_args(&vec!["-z"]).expect_err("Can't enable non-existent feature 'z'");
        Config::new_from_args(&vec!["-trace=z"]).expect_err("Can't trace non-existent feature 'z'");
        Config::new_from_args(&vec!["-testtrace=z"]).expect_err("Can't testtrace non-existent feature 'z'");
        Config::new_from_args(&vec!["-dump=z"]).expect_err("Can't dump non-existent feature 'z'");

        Config::new_from_args(&vec!["-trace=a"]).expect_err("Can't trace disabled feature 'a'");
        Config::new_from_args(&vec!["-testtrace=a"]).expect_err("Can't testtrace disabled feature 'a'");
        Config::new_from_args(&vec!["-dump=a"]).expect_err("Can't dump disabled feature 'a'");
    }
}
