import { useMemo, useCallback } from 'react';
import {
  useIsGrouped,
  useQueryParams,
  useTableStore,
  useTranslation,
  useNotification,
  useNavigate,
  SortUtils,
  useSortBy,
  FieldSelectorControl,
  useQueryClient,
  useParams,
  useQuery,
  useAuthContext,
  useGql,
  useMutation,
  useFieldsSelector,
  InvoiceNodeStatus,
} from '@openmsupply-client/common';
import { ItemRowFragment } from '@openmsupply-client/system';
import {
  inboundLinesToSummaryItems,
  isInboundDisabled,
  useIsInboundStatusChangeDisabled,
  isA,
} from './../../utils';
import { InboundItem } from './../../types';
import { getInboundQueries, ListParams } from './api';
import { useInboundShipmentColumns } from '../DetailView/ContentArea';
import {
  getSdk,
  InboundFragment,
  InboundRowFragment,
} from './operations.generated';

export const useInboundApi = () => {
  const { storeId } = useAuthContext();
  const keys = {
    base: () => ['inbound'] as const,
    detail: (id: string) => [...keys.base(), storeId, id] as const,
    list: () => [...keys.base(), storeId, 'list'] as const,
    paramList: (params: ListParams) => [...keys.list(), params] as const,
  };

  const { client } = useGql();
  const queries = getInboundQueries(getSdk(client), storeId);
  return { ...queries, storeId, keys };
};

const useInboundNumber = () => {
  const { invoiceNumber = '' } = useParams();
  return invoiceNumber;
};

export const useInbound = () => {
  const invoiceNumber = useInboundNumber();
  const api = useInboundApi();
  return useQuery(
    api.keys.detail(invoiceNumber),
    () => api.get.byNumber(invoiceNumber),
    // Don't refetch when the edit modal opens, for example. But, don't cache data when this query
    // is inactive. For example, when navigating away from the page and back again, refetch.
    {
      refetchOnMount: false,
      cacheTime: 0,
    }
  );
};

export const useIsInboundDisabled = (): boolean => {
  const { data } = useInbound();
  if (!data) return true;
  return isInboundDisabled(data);
};

export const useIsStatusChangeDisabled = (): boolean => {
  const { data } = useInbound();
  if (!data) return true;
  return useIsInboundStatusChangeDisabled(data);
};

export const useInboundSelector = <T = InboundFragment>(
  select?: (data: InboundFragment) => T
) => {
  const invoiceNumber = useInboundNumber();
  const api = useInboundApi();

  return useQuery(
    api.keys.detail(invoiceNumber),
    () => api.get.byNumber(invoiceNumber),
    { select }
  );
};

export const useInboundFields = <KeyOfInvoice extends keyof InboundFragment>(
  keyOrKeys: KeyOfInvoice | KeyOfInvoice[]
): FieldSelectorControl<InboundFragment, KeyOfInvoice> => {
  const { data } = useInbound();
  const { mutateAsync } = useUpdateInbound();
  const invoiceNumber = useInboundNumber();
  const api = useInboundApi();
  return useFieldsSelector(
    api.keys.detail(invoiceNumber),
    () => api.get.byNumber(invoiceNumber),
    patch => mutateAsync({ ...patch, id: data?.id ?? '' }),
    keyOrKeys
  );
};

export const useInboundLines = (itemId?: string) => {
  const selectItems = useCallback(
    (invoice: InboundFragment) => {
      return itemId
        ? invoice.lines.nodes.filter(({ item }) => itemId === item.id)
        : invoice.lines.nodes.filter(line => isA.stockInLine(line));
    },
    [itemId]
  );

  return useInboundSelector(selectItems);
};

export const useInboundItems = () => {
  const { sortBy, onChangeSortBy } = useSortBy<InboundItem>({
    key: 'itemName',
  });

  const selectItems = useCallback((invoice: InboundFragment) => {
    return inboundLinesToSummaryItems(
      invoice.lines.nodes.filter(line => isA.stockInLine(line))
    ).sort(
      SortUtils.getDataSorter(sortBy.key as keyof InboundItem, !!sortBy.isDesc)
    );
  }, []);

  const { data } = useInboundSelector(selectItems);

  return { data, sortBy, onSort: onChangeSortBy };
};

export const useNextItem = (
  currentItemId: string
): { next: ItemRowFragment | null; disabled: boolean } => {
  const next: ItemRowFragment | null = null;
  const disabled = true;
  const { data } = useInboundItems();

  if (!data) return { next, disabled };

  const numberOfItems = data.length;
  const currentIndex = data.findIndex(({ itemId }) => itemId === currentItemId);
  const nextIndex = currentIndex + 1;
  const nextItem = data?.[nextIndex];
  if (!nextItem) return { next, disabled };

  return {
    next: nextItem.lines[0].item,
    disabled: currentIndex === numberOfItems - 1,
  };
};

export const useSaveInboundLines = () => {
  const queryClient = useQueryClient();
  const invoiceNumber = useInboundNumber();
  const api = useInboundApi();
  return useMutation(api.updateLines, {
    onSettled: () =>
      queryClient.invalidateQueries(api.keys.detail(invoiceNumber)),
  });
};

export const useDeleteInboundLines = () => {
  const inboundNumber = useInboundNumber();
  const queryClient = useQueryClient();
  const api = useInboundApi();
  const queryKey = api.keys.detail(inboundNumber);

  return useMutation(api.deleteLines, {
    onMutate: async lines => {
      await queryClient.cancelQueries(queryKey);
      const previous = queryClient.getQueryData<InboundFragment>(queryKey);
      if (previous) {
        const nodes = previous.lines.nodes.filter(
          ({ id: lineId }) => !lines.find(({ id }) => lineId === id)
        );
        queryClient.setQueryData<InboundFragment>(queryKey, {
          ...previous,
          lines: {
            __typename: 'InvoiceLineConnector',
            nodes,
            totalCount: nodes.length,
          },
        });
      }
      return { previous, lines };
    },
    onError: (_error, _vars, ctx) => {
      // Having issues typing this correctly. If typing ctx in the args list,
      // then TS infers the wrong type for the useMutation call and all
      // hell breaks loose.
      const context = ctx as {
        previous: InboundFragment;
        lines: { id: string; invoiceId: string }[];
      };
      queryClient.setQueryData(queryKey, context?.previous);
    },
    onSettled: () => queryClient.invalidateQueries(queryKey),
  });
};

export const useDeleteSelectedLines = (): {
  onDelete: () => Promise<void>;
} => {
  const { success, info } = useNotification();
  const { items, lines } = useInboundRows();
  const { mutate } = useDeleteInboundLines();
  const isDisabled = useIsInboundDisabled();
  const t = useTranslation('replenishment');

  const selectedRows = useTableStore(state => {
    const { isGrouped } = state;

    return isGrouped
      ? items
          ?.filter(({ id }) => state.rowState[id]?.isSelected)
          .map(({ lines }) => lines.flat())
          .flat()
      : lines?.filter(({ id }) => state.rowState[id]?.isSelected);
  });

  const onDelete = async () => {
    if (isDisabled) {
      info(t('label.cant-delete-disabled'))();
      return;
    }
    if (selectedRows && selectedRows?.length > 0) {
      const number = selectedRows?.length;
      const onSuccess = success(t('messages.deleted-lines', { number }));
      mutate(selectedRows, {
        onSuccess,
      });
    } else {
      const infoSnack = info(t('label.select-rows-to-delete-them'));
      infoSnack();
    }
  };

  return { onDelete };
};

export const useInbounds = () => {
  const queryParams = useQueryParams<InboundRowFragment>({
    initialSortBy: { key: 'otherPartyName' },
  });
  const api = useInboundApi();

  return {
    ...useQuery(api.keys.paramList(queryParams), () =>
      api.get.list({
        first: queryParams.first,
        offset: queryParams.offset,
        sortBy: queryParams.sortBy,
        filterBy: queryParams.filter.filterBy,
      })
    ),
    ...queryParams,
  };
};

export const useInsertInbound = () => {
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const api = useInboundApi();
  return useMutation(api.insert, {
    onSuccess: invoiceNumber => {
      navigate(String(invoiceNumber));
      return queryClient.invalidateQueries(api.keys.base());
    },
  });
};

export const useUpdateInbound = () => {
  const queryClient = useQueryClient();
  const api = useInboundApi();
  return useMutation(api.update, {
    onSuccess: () => queryClient.invalidateQueries(api.keys.base()),
  });
};

export const useDeleteSelectedInbounds = () => {
  const queryClient = useQueryClient();
  const { data: rows } = useInbounds();
  const api = useInboundApi();
  const { mutate } = useMutation(api.delete);
  const t = useTranslation('replenishment');

  const { success, info } = useNotification();

  const selectedRows = useTableStore(
    state =>
      rows?.nodes.filter(({ id }) => state.rowState[id]?.isSelected) ?? []
  );

  const deleteAction = () => {
    const count = selectedRows?.length;
    if (selectedRows && count > 0) {
      const canDeleteRows = selectedRows.every(
        ({ status }) => status === InvoiceNodeStatus.New
      );
      if (!canDeleteRows) {
        const cannotDeleteSnack = info(t('messages.cant-delete-invoices'));
        cannotDeleteSnack();
      } else {
        mutate(selectedRows, {
          onSettled: () => queryClient.invalidateQueries(api.keys.base()),
        });
        const deletedMessage = t('messages.deleted-invoices', { count });
        const successSnack = success(deletedMessage);
        successSnack();
      }
    } else {
      const selectRowsSnack = info(t('messages.select-rows-to-delete'));
      selectRowsSnack();
    }
  };

  return deleteAction;
};

export const useInboundServiceLines = () => {
  const selectLines = useCallback((invoice: InboundFragment) => {
    return invoice.lines.nodes.filter(isA.serviceLine);
  }, []);

  return useInboundSelector(selectLines);
};

export const useInboundRows = () => {
  const { isGrouped, toggleIsGrouped } = useIsGrouped('inboundShipment');
  const { data: lines } = useInboundLines();
  const { data: items } = useInboundItems();
  const { columns, onChangeSortBy, sortBy } = useInboundShipmentColumns();

  const sortedItems = useMemo(() => {
    const currentColumn = columns.find(({ key }) => key === sortBy.key);
    if (!currentColumn?.getSortValue) return items;
    const sorter = SortUtils.getColumnSorter(
      currentColumn?.getSortValue,
      !!sortBy.isDesc
    );
    return [...(items ?? [])].sort(sorter);
  }, [items, sortBy.key, sortBy.isDesc]);

  const sortedLines = useMemo(() => {
    const currentColumn = columns.find(({ key }) => key === sortBy.key);
    if (!currentColumn?.getSortValue) return lines;
    const sorter = SortUtils.getColumnSorter(
      currentColumn?.getSortValue,
      !!sortBy.isDesc
    );
    return [...(lines ?? [])].sort(sorter);
  }, [lines, sortBy.key, sortBy.isDesc]);

  const rows = isGrouped ? sortedItems : sortedLines;

  return {
    isGrouped,
    toggleIsGrouped,
    columns,
    rows,
    lines: sortedLines,
    items: sortedItems,
    onChangeSortBy,
    sortBy,
  };
};
