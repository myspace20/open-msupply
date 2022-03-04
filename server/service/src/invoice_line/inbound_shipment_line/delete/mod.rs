use crate::{service_provider::ServiceContext, WithDBError};
use repository::{InvoiceLineRowRepository, RepositoryError, StockLineRowRepository};

mod validate;

use validate::validate;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeleteInboundShipmentLine {
    pub id: String,
    pub invoice_id: String,
}

type OutError = DeleteInboundShipmentLineError;

pub fn delete_inbound_shipment_line(
    ctx: &ServiceContext,
    _store_id: &str,
    input: DeleteInboundShipmentLine,
) -> Result<String, OutError> {
    let line_id = ctx
        .connection
        .transaction_sync(|connection| {
            let line = validate(&input, &connection)?;

            let delete_batch_id_option = line.stock_line_id.clone();

            InvoiceLineRowRepository::new(&connection).delete(&line.id)?;

            if let Some(id) = delete_batch_id_option {
                StockLineRowRepository::new(&connection).delete(&id)?;
            }

            Ok(line.id) as Result<String, OutError>
        })
        .map_err(|error| error.to_inner_error())?;
    Ok(line_id)
}
#[derive(Debug, PartialEq)]
pub enum DeleteInboundShipmentLineError {
    LineDoesNotExist,
    DatabaseError(RepositoryError),
    InvoiceDoesNotExist,
    NotAnInboundShipment,
    NotThisStoreInvoice,
    CannotEditFinalised,
    BatchIsReserved,
    NotThisInvoiceLine(String),
}

impl From<RepositoryError> for DeleteInboundShipmentLineError {
    fn from(error: RepositoryError) -> Self {
        DeleteInboundShipmentLineError::DatabaseError(error)
    }
}

impl<ERR> From<WithDBError<ERR>> for DeleteInboundShipmentLineError
where
    ERR: Into<DeleteInboundShipmentLineError>,
{
    fn from(result: WithDBError<ERR>) -> Self {
        match result {
            WithDBError::DatabaseError(error) => error.into(),
            WithDBError::Error(error) => error.into(),
        }
    }
}
