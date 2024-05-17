use repository::{
    InvoiceLineRow, InvoiceLineType, InvoiceRow, InvoiceStatus, ItemRow, RepositoryError,
    StockLine, StockLineRow, StorageConnection,
};

use crate::{
    invoice::common::{calculate_foreign_currency_total, calculate_total_after_tax},
    invoice_line::StockOutType,
};

use super::{InsertStockOutLine, InsertStockOutLineError};

pub fn generate(
    connection: &StorageConnection,
    input: InsertStockOutLine,
    item_row: ItemRow,
    batch: StockLine,
    invoice: InvoiceRow,
) -> Result<(InvoiceLineRow, StockLineRow), InsertStockOutLineError> {
    let adjust_total_number_of_packs =
        should_adjust_total_number_of_packs(invoice.status.clone(), &input.r#type);

    let update_batch = generate_batch_update(
        input.clone(),
        batch.stock_line_row.clone(),
        adjust_total_number_of_packs,
    );
    let new_line = generate_line(connection, input, item_row, update_batch.clone(), invoice)?;

    Ok((new_line, update_batch))
}

fn generate_batch_update(
    input: InsertStockOutLine,
    batch: StockLineRow,
    adjust_total_number_of_packs: bool,
) -> StockLineRow {
    let mut update_batch = batch;

    let reduction = input.number_of_packs;

    update_batch.available_number_of_packs -= reduction;
    if adjust_total_number_of_packs {
        update_batch.total_number_of_packs -= reduction;
    }

    update_batch.location_id = input.location_id.or(update_batch.location_id);
    update_batch.batch = input.batch.or(update_batch.batch);
    update_batch.expiry_date = input.expiry_date.or(update_batch.expiry_date);
    update_batch.pack_size = input.pack_size.unwrap_or(update_batch.pack_size);
    update_batch.cost_price_per_pack = input
        .cost_price_per_pack
        .unwrap_or(update_batch.cost_price_per_pack);
    update_batch.sell_price_per_pack = input
        .sell_price_per_pack
        .unwrap_or(update_batch.sell_price_per_pack);

    update_batch
}

fn generate_line(
    connection: &StorageConnection,
    InsertStockOutLine {
        id,
        r#type: _,
        invoice_id,
        stock_line_id,
        number_of_packs,
        total_before_tax,
        note,
        tax_percentage: _,
        location_id: _,
        batch: _,
        pack_size: _,
        expiry_date: _,
        cost_price_per_pack: _,
        sell_price_per_pack: _,
    }: InsertStockOutLine,
    ItemRow {
        id: item_id,
        name: item_name,
        code: item_code,
        ..
    }: ItemRow,
    StockLineRow {
        sell_price_per_pack,
        cost_price_per_pack,
        pack_size,
        batch,
        expiry_date,
        location_id,
        note: _,
        ..
    }: StockLineRow,
    InvoiceRow {
        tax_percentage,
        currency_id,
        currency_rate,
        ..
    }: InvoiceRow,
) -> Result<InvoiceLineRow, RepositoryError> {
    let total_before_tax = total_before_tax.unwrap_or(cost_price_per_pack * number_of_packs);
    let total_after_tax = calculate_total_after_tax(total_before_tax, tax_percentage);
    let foreign_currency_price_before_tax = calculate_foreign_currency_total(
        connection,
        total_before_tax,
        currency_id,
        &currency_rate,
    )?;

    Ok(InvoiceLineRow {
        id,
        invoice_id,
        item_link_id: item_id,
        location_id,
        pack_size,
        batch,
        expiry_date,
        sell_price_per_pack,
        cost_price_per_pack,
        r#type: InvoiceLineType::StockOut,
        number_of_packs,
        item_name,
        item_code,
        stock_line_id: Some(stock_line_id),
        total_before_tax,
        total_after_tax,
        tax_percentage,
        note,
        inventory_adjustment_reason_id: None,
        return_reason_id: None,
        foreign_currency_price_before_tax,
    })
}

fn should_adjust_total_number_of_packs(status: InvoiceStatus, r#type: &StockOutType) -> bool {
    match r#type {
        StockOutType::InventoryReduction => true,
        _ => status == InvoiceStatus::Picked,
    }
}
