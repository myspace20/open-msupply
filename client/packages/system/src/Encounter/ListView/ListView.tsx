import React, { FC } from 'react';
import {
  TableProvider,
  DataTable,
  useColumns,
  createTableStore,
  NothingHere,
  useUrlQueryParams,
  useNavigate,
  createQueryParamsStore,
  ColumnAlign,
  useFormatDateTime,
} from '@openmsupply-client/common';
import {
  useEncounter,
  EncounterFragmentWithId,
  EncounterRowFragmentWithId,
} from '../api';

const EncounterListComponent: FC = () => {
  const {
    updateSortQuery,
    updatePaginationQuery,
    queryParams: { sortBy, page, first, offset },
  } = useUrlQueryParams();
  const { data, isError, isLoading } = useEncounter.document.list();
  const pagination = { page, first, offset };
  const navigate = useNavigate();
  const { localisedDate, localisedTime } = useFormatDateTime();
  const columns = useColumns<EncounterRowFragmentWithId>(
    [
      {
        key: 'id',
        label: 'label.encounter-id',
      },
      {
        key: 'program',
        label: 'label.program',
        accessor: ({ rowData }) => rowData?.document.documentRegistry?.name,
      },
      {
        key: 'date',
        label: 'label.date',
        accessor: ({ rowData }) => rowData?.startDatetime,
        formatter: dateString =>
          dateString ? localisedDate((dateString as string) || '') : '',
      },
      {
        key: 'startDatetime',
        label: 'label.encounter-start',
        formatter: dateString =>
          dateString ? localisedTime((dateString as string) || '') : '',
      },
      {
        key: 'endDatetime',
        label: 'label.encounter-end',
        formatter: dateString =>
          dateString ? localisedTime((dateString as string) || '') : '',
      },
      {
        key: 'status',
        label: 'label.status',
        align: ColumnAlign.Right,
        width: 175,
      },
      {
        key: 'patient',
        label: 'label.patient',
        accessor: ({ rowData }) => rowData?.patientId,
      },
    ],
    { onChangeSortBy: updateSortQuery, sortBy },
    [sortBy]
  );

  return (
    <>
      <DataTable
        id="name-list"
        pagination={{ ...pagination, total: data?.totalCount }}
        onChangePage={updatePaginationQuery}
        columns={columns}
        data={data?.nodes}
        isLoading={isLoading}
        isError={isError}
        onRowClick={row => {
          navigate(encodeURIComponent(String(row.id)));
        }}
        noDataElement={<NothingHere />}
      />
    </>
  );
};

export const EncounterListView: FC = () => (
  <TableProvider
    createStore={createTableStore}
    queryParamsStore={createQueryParamsStore<EncounterFragmentWithId>({
      initialSortBy: { key: 'startDateTime' },
    })}
  >
    <EncounterListComponent />
  </TableProvider>
);
