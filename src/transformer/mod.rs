use crate::{
    error::DataError,
    event::MarketEvent,
    subscription::{Map, SubKind},
};
use async_trait::async_trait;
use barter_integration::{model::Instrument, protocol::websocket::WsMessage, Transformer};
use tokio::sync::mpsc;

/// Generic OrderBook [`ExchangeTransformer`]s.
pub mod book;

/// Generic stateless [`ExchangeTransformer`] often used for transforming
/// [`PublicTrades`](crate::subscription::trade::PublicTrades) streams.
pub mod stateless;

/// Defines how to construct a [`Transformer`] used by [`MarketStream`](super::MarketStream)s to
/// translate exchange specific types to normalised Barter types.
#[async_trait]
pub trait ExchangeTransformer<Exchange, Kind>
where
    Self: Transformer<Output = MarketEvent<Kind::Event>, Error = DataError> + Sized,
    Kind: SubKind,
{
    /// Construct a new [`Self`].
    ///
    /// The [`mpsc::UnboundedSender`] can be used by [`Self`] to send messages back to the exchange.
    async fn new(
        ws_sink_tx: mpsc::UnboundedSender<WsMessage>,
        instrument_map: Map<Instrument>,
    ) -> Result<Self, DataError>;
}
