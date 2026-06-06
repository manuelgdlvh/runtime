pub mod runtime;
pub mod tcp;
pub mod time;

#[macro_export]
macro_rules! test_suite {
    ($name:ident, $runtime:ty) => {
        mod $name {

            mod runtime {

                #[test]
                fn block_on_returns_completed_future_value() {
                    $crate::runtime::block_on_returns_completed_future_value::<$runtime>();
                }

                #[test]
                fn defer_spawn_returns_joined_task_result() {
                    $crate::runtime::defer_spawn_returns_joined_task_result::<$runtime>();
                }

                #[test]
                fn defer_join_returns_deferred_future_output() {
                    $crate::runtime::defer_join_returns_deferred_future_output::<$runtime>();
                }

                #[test]
                fn defer_spawn_after_runtime_dropped_returns_disconnected() {
                    $crate::runtime::defer_spawn_after_runtime_dropped_returns_disconnected::<
                        $runtime,
                    >();
                }

                #[test]
                #[should_panic]
                fn spawn_local_panics_on_multi_threaded_runtime() {
                    $crate::runtime::spawn_local_panics_on_multi_threaded_runtime::<$runtime>();
                }

                #[test]
                fn spawn_local_returns_result_on_single_threaded_runtime() {
                    $crate::runtime::spawn_local_returns_result_on_single_threaded_runtime::<
                        $runtime,
                    >();
                }
            }

            mod tcp {

                #[test]
                fn listener_bind_succeeds_on_localhost() {
                    $crate::tcp::listener_bind_succeeds_on_localhost::<$runtime>();
                }

                #[test]
                fn stream_connect_succeeds_to_bound_listener() {
                    $crate::tcp::stream_connect_succeeds_to_bound_listener::<$runtime>();
                }

                #[test]
                fn stream_read_write_round_trip_succeeds() {
                    $crate::tcp::stream_read_write_round_trip_succeeds::<$runtime>();
                }
            }
            mod time {
                #[test]
                fn sleep_delays_for_at_least_requested_duration() {
                    $crate::time::sleep_delays_for_at_least_requested_duration::<$runtime>();
                }

                #[test]
                fn timeout_returns_ok_when_future_completes_in_time() {
                    $crate::time::timeout_returns_ok_when_future_completes_in_time::<$runtime>();
                }
                #[test]
                fn timeout_returns_timed_out_when_future_exceeds_limit() {
                    $crate::time::timeout_returns_timed_out_when_future_exceeds_limit::<$runtime>();
                }

                #[test]
                fn timeout_with_zero_duration_returns_immediate_future() {
                    $crate::time::timeout_with_zero_duration_returns_immediate_future::<$runtime>();
                }
            }
        }
    };
}

#[cfg(feature = "tokio")]
test_suite!(tokio, ::runtime::tokio::Tokio);
