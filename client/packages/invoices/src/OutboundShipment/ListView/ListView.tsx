import React, { FC, useState } from 'react';
import { useNavigate } from 'react-router';

import {
  DataTable,
  useColumns,
  InvoiceRow,
  useListData,
  getNameAndColorColumn,
  TableProvider,
  createTableStore,
  Color,
  OutboundShipmentStatus,
  useOmSupplyApi,
} from '@openmsupply-client/common';
import { getOutboundShipmentListViewApi } from './api';
import { NameSearchModal } from '@openmsupply-client/system/src/Name';
import { getStatusTranslation } from '../utils';
import { Toolbar } from './Toolbar';
import { AppBarButtons } from './AppBarButtons';

export const OutboundShipmentListViewComponent: FC = () => {
  const navigate = useNavigate();
  const { api } = useOmSupplyApi();

  const {
    totalCount,
    data,
    isLoading,
    onDelete,
    onUpdate,
    sortBy,
    onChangeSortBy,
    onCreate,
    onChangePage,
    pagination,
    invalidate,
  } = useListData(
    { initialSortBy: { key: 'otherPartyName' } },
    'invoice',
    getOutboundShipmentListViewApi(api)
  );

  const onColorUpdate = (row: InvoiceRow, color: Color) => {
    onUpdate({ ...row, color: color.hex });
  };

  const columns = useColumns<InvoiceRow>(
    [
      getNameAndColorColumn(onColorUpdate),
      [
        'status',
        {
          formatter: (status, { t }) =>
            t(getStatusTranslation(status as OutboundShipmentStatus)),
        },
      ],
      'invoiceNumber',
      'entryDatetime',
      'confirmedDatetime',
      'comment',
      ['totalAfterTax', { accessor: invoice => invoice.pricing.totalAfterTax }],
      'selection',
    ],
    { onChangeSortBy, sortBy },
    [sortBy]
  );

  const [open, setOpen] = useState(false);

  return (
    <>
      <NameSearchModal
        open={open}
        onClose={() => setOpen(false)}
        onChange={async name => {
          setOpen(false);

          const createInvoice = async () => {
            const invoice = {
              id: String(Math.ceil(Math.random() * 1000000)),
              nameId: name?.id,
            };

            const result = await onCreate(invoice);

            invalidate();
            navigate(`/distribution/outbound-shipment/${result.id}`);
          };

          createInvoice();
        }}
      />

      <Toolbar onDelete={onDelete} data={data} />
      <AppBarButtons onCreate={setOpen} />

      <DataTable
        pagination={{ ...pagination, total: totalCount }}
        onChangePage={onChangePage}
        columns={columns}
        data={data ?? []}
        isLoading={isLoading}
        onRowClick={row => {
          navigate(`/distribution/outbound-shipment/${row.id}`);
        }}
      />
    </>
  );
};

export const OutboundShipmentListView: FC = () => {
  return (
    <TableProvider createStore={createTableStore}>
      <OutboundShipmentListViewComponent />
    </TableProvider>
  );
};
