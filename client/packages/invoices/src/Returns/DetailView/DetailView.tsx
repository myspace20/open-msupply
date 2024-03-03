import React, { FC } from 'react';
import {
  TableProvider,
  createTableStore,
  // useEditModal,
  DetailViewSkeleton,
  AlertModal,
  useNavigate,
  RouteBuilder,
  useTranslation,
  createQueryParamsStore,
  useEditModal,
  DetailTabs,
  // ModalMode,
} from '@openmsupply-client/common';
// import { toItemRow } from '@openmsupply-client/system';
import { ContentArea } from './ContentArea';
// import { Toolbar } from './Toolbar';
// import { Footer } from './Footer';
// import { AppBarButtons } from './AppBarButtons';
// import { SidePanel } from './SidePanel';
import { OutboundReturnDetailRowFragment, useReturns } from '../api';
import { AppRoute } from '@openmsupply-client/config';
// import { Draft } from '../..';
import { OutboundReturnEditModal } from '../modals';
// import { OutboundLineEdit } from './OutboundLineEdit';

export const DetailView: FC = () => {
  // const isDisabled = useReturn.utils.isDisabled();
  const { onOpen, onClose, isOpen } = useEditModal();
  const { data, isLoading } = useReturns.document.outboundReturn();
  const t = useTranslation('replenishment');
  const navigate = useNavigate();

  const onRowClick = () => {};

  const onAddItem = () => {};
  //  (draft?: Draft) => {
  //   onOpen(draft);
  //   setMode(ModalMode.Create);
  // };

  if (isLoading) return <DetailViewSkeleton hasGroupBy={true} hasHold={true} />;

  const tabs = [
    {
      Component: (
        <ContentArea
          onRowClick={onRowClick}
          onAddItem={onAddItem}
          rows={data?.lines?.nodes ?? []}
        />
      ),
      value: 'Details',
    },
    {
      Component: <p>To-do</p>,
      value: 'Log',
    },
  ];

  return (
    <React.Suspense
      fallback={<DetailViewSkeleton hasGroupBy={true} hasHold={true} />}
    >
      {data ? (
        <TableProvider
          createStore={createTableStore}
          queryParamsStore={createQueryParamsStore<OutboundReturnDetailRowFragment>(
            {
              initialSortBy: {
                key: 'itemName',
              },
            }
          )}
        >
          {/* <AppBarButtons onAddItem={onAddItem} /> */}
          <button onClick={() => onOpen()}>HERE</button>
          {isOpen && (
            <OutboundReturnEditModal
              isOpen={isOpen}
              onClose={onClose}
              stockLineIds={[]}
              // TODO: remove anys
              supplierId={(data as any).otherPartyId}
              returnId={(data as any).id}
            />
          )}

          {/* <Toolbar /> */}
          <DetailTabs tabs={tabs} />
          {/* <Footer /> */}
          {/* <SidePanel /> */}
        </TableProvider>
      ) : (
        <AlertModal
          open={true}
          onOk={() =>
            navigate(
              RouteBuilder.create(AppRoute.Replenishment)
                .addPart(AppRoute.OutboundReturn)
                .build()
            )
          }
          title={t('error.return-not-found')}
          message={t('messages.click-to-return-to-returns')}
        />
      )}
    </React.Suspense>
  );
};
