pub mod runtime;
pub mod tcp;
pub mod time;

macro_rules! test {
    ($runtime:ty, $threads:literal, $module:ident :: $test:ident) => {
        #[test]
        fn $test() {
            <$runtime>::new($threads).block_on($crate::$module::$test::<$runtime>());
        }
    };
}

macro_rules! test_panic {
    ($runtime:ty, $threads:literal, $module:ident :: $test:ident) => {
        #[test]
        #[should_panic]
        fn $test() {
            <$runtime>::new($threads).block_on($crate::$module::$test::<$runtime>());
        }
    };
}

#[macro_export]
macro_rules! test_suite {
    ($name:ident, $runtime:ty) => {
        mod $name {
            mod runtime {
                use runtime::Runtime;

                test!(
                    $runtime,
                    1,
                    runtime::block_on_returns_completed_future_value
                );
                test!($runtime, 1, runtime::defer_spawn_returns_joined_task_result);
                test!(
                    $runtime,
                    1,
                    runtime::defer_join_returns_deferred_future_output
                );
                test!(
                    $runtime,
                    1,
                    runtime::defer_spawn_after_runtime_dropped_returns_disconnected
                );
                test_panic!(
                    $runtime,
                    2,
                    runtime::spawn_local_panics_on_multi_threaded_runtime
                );
                test!(
                    $runtime,
                    1,
                    runtime::spawn_local_returns_result_on_single_threaded_runtime
                );
            }

            mod tcp {
                use runtime::Runtime;

                test!($runtime, 1, tcp::listener_bind_succeeds_on_localhost);
                test!($runtime, 1, tcp::stream_connect_succeeds_to_bound_listener);
                test!($runtime, 1, tcp::stream_read_write_round_trip_succeeds);
            }

            mod time {
                use runtime::Runtime;

                test!(
                    $runtime,
                    1,
                    time::sleep_delays_for_at_least_requested_duration
                );
                test!(
                    $runtime,
                    1,
                    time::timeout_returns_ok_when_future_completes_in_time
                );
                test!(
                    $runtime,
                    1,
                    time::timeout_returns_timed_out_when_future_exceeds_limit
                );
                test!(
                    $runtime,
                    1,
                    time::timeout_with_zero_duration_returns_immediate_future
                );
            }
        }
    };
}

#[cfg(feature = "tokio")]
test_suite!(tokio, ::runtime::tokio::Tokio);
