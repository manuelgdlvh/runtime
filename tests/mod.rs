pub mod runtime;
pub mod tcp;
pub mod time;

#[macro_export]
macro_rules! test_suite {
    ($name:ident, $runtime:ty) => {
        mod $name {

            mod runtime {

                #[test]
                fn test_block_on_when_finish_then_returns_output() {
                    $crate::runtime::test_block_on_when_finish_then_returns_output::<$runtime>();
                }

                #[test]
                fn test_defer_when_spawn_then_receive_result() {
                    $crate::runtime::test_defer_when_spawn_then_receive_result::<$runtime>();
                }

                #[test]
                fn test_defer_when_join_then_receive_future_output() {
                    $crate::runtime::test_defer_when_join_then_receive_future_output::<$runtime>();
                }

                #[test]
                fn test_defer_when_spawn_and_block_on_returns_then_disconnected_error() {
                    $crate::runtime::test_defer_when_spawn_and_block_on_returns_then_disconnected_error::<$runtime>();
                }

                #[test]
                #[should_panic]
                fn test_spawn_local_when_multi_threaded_then_panic() {
                    $crate::runtime::test_spawn_local_when_multi_threaded_then_panic::<$runtime>();
                }

                #[test]
                fn test_spawn_local_when_single_threaded_then_returns_result() {
                    $crate::runtime::test_spawn_local_when_single_threaded_then_returns_result::<
                        $runtime,
                    >();
                }
            }

            mod tcp {

                #[test]
                fn listener_test_when_bind_then_returns_tcp_listener() {
                    $crate::tcp::listener_test_when_bind_then_returns_tcp_listener::<$runtime>();
                }

                #[test]
                fn stream_test_when_connect_then_returns_tcp_stream() {
                    $crate::tcp::stream_test_when_connect_then_returns_tcp_stream::<$runtime>();
                }

                #[test]
                fn stream_test_when_write_then_read_sucess() {
                    $crate::tcp::stream_test_when_write_then_read_sucess::<$runtime>();
                }
            }
            mod time {
                #[test]
                fn sleep_test_when_invoked_then_wait() {
                    $crate::time::sleep_test_when_invoked_then_wait::<$runtime>();
                }

                #[test]
                fn timeout_test_when_future_returns_earlier_then_returns_result() {
                    $crate::time::timeout_test_when_future_returns_earlier_then_returns_result::<$runtime>();
                }
                #[test]
                fn timeout_test_when_sleep_returns_earlier_then_returns_error() {
                    $crate::time::timeout_test_when_sleep_returns_earlier_then_returns_error::<$runtime>();
                }

                #[test]
                fn timeout_test_when_future_does_not_yield_then_returns_result() {
                    $crate::time::timeout_test_when_future_does_not_yield_then_returns_result::<$runtime>();
                }
            }
        }

    };
}

#[cfg(feature = "tokio")]
test_suite!(tokio, ::runtime::tokio::Tokio);
