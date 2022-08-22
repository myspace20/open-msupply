import React, { FC } from 'react';
import {
  TableProvider,
  DataTable,
  useColumns,
  createTableStore,
  NothingHere,
  createQueryParamsStore,
  useFormatDateTime,
  useUrlQueryParams,
  ColumnAlign,
} from '@openmsupply-client/common';
import { usePatient } from '../api';
import { usePatientModalStore } from '../hooks';
import { PatientModal } from '../PatientView';
import {
  EncounterFragmentWithId,
  EncounterRowFragmentWithId,
} from '../../Encounter';

const EncounterListComponent: FC = () => {
  const {
    updateSortQuery,
    updatePaginationQuery,
    queryParams: { sortBy, page, first, offset },
  } = useUrlQueryParams();
  const { data, isError, isLoading } = usePatient.document.encounters();
  const pagination = { page, first, offset };
  const { localisedDateTime } = useFormatDateTime();
  const { setCurrent, setDocument, setProgramType } = usePatientModalStore();

  const columns = useColumns<EncounterRowFragmentWithId>(
    [
      {
        key: 'type',
        label: 'label.encounter-type',
      },
      {
        key: 'program',
        label: 'label.program',
      },
      {
        key: 'startDatetime',
        label: 'label.encounter-start',
        formatter: dateString =>
          dateString ? localisedDateTime((dateString as string) || '') : '',
      },
      {
        key: 'endDatetime',
        label: 'label.encounter-end',
        formatter: dateString =>
          dateString ? localisedDateTime((dateString as string) || '') : '',
      },
      {
        key: 'status',
        label: 'label.status',
        align: ColumnAlign.Right,
        width: 175,
      },
    ],
    { onChangeSortBy: updateSortQuery, sortBy },
    [sortBy]
  );

  return (
    <DataTable
      id="encounter-list"
      pagination={{ ...pagination, total: data?.totalCount }}
      onChangePage={updatePaginationQuery}
      columns={columns}
      data={data?.nodes}
      isLoading={isLoading}
      isError={isError}
      onRowClick={row => {
        setDocument({ type: row.type, name: row.name });
        setProgramType(row.program);
        setCurrent(PatientModal.Encounter);
      }}
      noDataElement={<NothingHere />}
    />
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
