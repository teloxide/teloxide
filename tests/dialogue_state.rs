#[cfg(feature = "macros")]
use teloxide::macros::DialogueState;
// We put tests here because macro expand in unit tests in the crate was a
// failure

#[test]
#[cfg(feature = "macros")]
fn compile_test() {
    #[allow(dead_code)]
    #[derive(DialogueState, Clone)]
    #[handler_out(Result<(), teloxide::RequestError>)]
    enum State {
        #[handler(handle_start)]
        Start,

        #[handler(handle_have_data)]
        HaveData(String),
    }

    impl Default for State {
        fn default() -> Self {
            Self::Start
        }
    }

    async fn handle_start() -> Result<(), teloxide::RequestError> {
        Ok(())
    }

    async fn handle_have_data() -> Result<(), teloxide::RequestError> {
        Ok(())
    }
}

#[test]
#[cfg(feature = "macros")]
fn compile_test_generics() {
    #[allow(dead_code)]
    #[derive(DialogueState, Clone)]
    #[handler_out(Result<(), teloxide::RequestError>)]
    enum State<X: Clone + Send + Sync + 'static> {
        #[handler(handle_start)]
        Start,

        #[handler(handle_have_data)]
        HaveData(X),
    }

    impl<X: Clone + Send + Sync + 'static> Default for State<X> {
        fn default() -> Self {
            Self::Start
        }
    }

    async fn handle_start() -> Result<(), teloxide::RequestError> {
        Ok(())
    }

    async fn handle_have_data() -> Result<(), teloxide::RequestError> {
        Ok(())
    }
}
