use crate::{
    dispatching::{
        core::{
            DemuxBuilder, Guard, Guards, Handler, IntoGuard, IntoHandler, OrGuard, Parser,
            RecombineFrom,
        },
        dispatcher_context::DispatcherContext,
        handlers::{
            guard_handlers::{GuardHandler, GuardsHandler},
            update_kind_handler::UpdateKindHandler,
        },
        updates::UpdateRest,
    },
    types::Update,
};

pub struct UpdateKindHandlerBuilder<Upd, UpdateParser, Err> {
    update_parser: UpdateParser,
    demux: DemuxBuilder<DispatcherContext<Upd>, Err>,
    guards: Guards<DispatcherContext<Upd>>,
    last_guard: Option<Box<dyn Guard<DispatcherContext<Upd>> + Send + Sync>>,
}

impl<Upd, UpdateParser, Err> UpdateKindHandlerBuilder<Upd, UpdateParser, Err>
where
    UpdateParser: Parser<Update, Upd, UpdateRest>,
    Update: RecombineFrom<UpdateParser, Upd, UpdateRest>,
{
    pub fn new(update_parser: UpdateParser) -> Self {
        UpdateKindHandlerBuilder {
            update_parser,
            demux: DemuxBuilder::new(),
            guards: Guards::new(),
            last_guard: None,
        }
    }
}

impl<Upd: Send + Sync + 'static, UpdateParser, Err> UpdateKindHandlerBuilder<Upd, UpdateParser, Err>
where
    Err: Send + Sync + 'static,
    UpdateParser: Parser<Update, Upd, UpdateRest>,
    Update: RecombineFrom<UpdateParser, Upd, UpdateRest>,
{
    pub fn by<F, H>(mut self, f: F) -> UpdateKindHandler<Upd, UpdateParser, H, Err>
    where
        H: Handler<DispatcherContext<Upd>, Err> + 'static,
        F: IntoHandler<H>,
    {
        self.create_guards_service();

        let UpdateKindHandlerBuilder { update_parser, demux, .. } = self;
        UpdateKindHandler::new(update_parser, f.into_handler(), demux.build())
    }
}

impl<Upd: Send + Sync + 'static, UpdateParser, Err: Send + Sync + 'static>
    UpdateKindHandlerBuilder<Upd, UpdateParser, Err>
{
    pub fn with_guard<G: Guard<DispatcherContext<Upd>> + Send + Sync + 'static>(
        mut self,
        guard: impl IntoGuard<DispatcherContext<Upd>, G> + 'static,
    ) -> Self {
        self.add_last_to_guards();
        self.last_guard = Some(Box::new(guard.into_guard()) as _);
        self
    }

    pub fn or_with_guard<G: Guard<DispatcherContext<Upd>> + Send + Sync + 'static>(
        mut self,
        guard: impl IntoGuard<DispatcherContext<Upd>, G> + 'static,
    ) -> Self {
        let prev = self
            .last_guard
            .take()
            .expect("or function must be called after using .with_* funtion!");
        self.last_guard = Some(Box::new(OrGuard::new(prev, guard.into_guard())) as _);
        self
    }

    pub fn or_else<F, H>(mut self, func: F) -> Self
    where
        F: IntoHandler<H>,
        H: Handler<DispatcherContext<Upd>, Err> + Send + Sync + 'static,
        Err: Send + Sync + 'static,
    {
        let prev_guard = self
            .last_guard
            .take()
            .expect("or_else function must be called after using .with_* funtion!");
        let wrong_handler = func.into_handler();

        self.create_guards_service();
        self.demux.add_service(GuardHandler::new(prev_guard, wrong_handler));

        self
    }

    fn create_guards_service(&mut self) {
        self.add_last_to_guards();

        if !self.guards.is_empty() {
            let mut guards = Guards::new();
            std::mem::swap(&mut guards, &mut self.guards);
            self.demux.add_service(GuardsHandler::new(guards));
        }
    }

    fn add_last_to_guards(&mut self) {
        let prev = self.last_guard.take();
        if let Some(prev) = prev {
            self.guards.add_boxed_guard(prev);
        }
    }
}
