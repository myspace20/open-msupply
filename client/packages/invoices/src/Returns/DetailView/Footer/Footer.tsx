import React, { FC, memo } from 'react';
import {
  Box,
  ButtonWithIcon,
  StatusCrumbs,
  XCircleIcon,
  useTranslation,
  AppFooterPortal,
  InvoiceNodeStatus,
  useBreadcrumbs,
} from '@openmsupply-client/common';
import { getStatusTranslator, outboundStatuses } from '../../../utils';
import { useReturn, OutboundFragment } from '../../api';
import { StatusChangeButton } from './StatusChangeButton';
import { OnHoldButton } from './OnHoldButton';

const createStatusLog = (invoice: OutboundFragment) => {
  const statusIdx = outboundStatuses.findIndex(s => invoice.status === s);

  const statusLog: Record<InvoiceNodeStatus, null | undefined | string> = {
    [InvoiceNodeStatus.New]: null,
    [InvoiceNodeStatus.Allocated]: null,
    [InvoiceNodeStatus.Picked]: null,
    [InvoiceNodeStatus.Shipped]: null,
    [InvoiceNodeStatus.Delivered]: null,
    [InvoiceNodeStatus.Verified]: null,
  };

  if (statusIdx >= 0) {
    statusLog[InvoiceNodeStatus.New] = invoice.createdDatetime;
  }
  if (statusIdx >= 1) {
    statusLog[InvoiceNodeStatus.Allocated] = invoice.allocatedDatetime;
  }
  if (statusIdx >= 2) {
    statusLog[InvoiceNodeStatus.Picked] = invoice.pickedDatetime;
  }
  if (statusIdx >= 3) {
    statusLog[InvoiceNodeStatus.Shipped] = invoice.shippedDatetime;
  }
  if (statusIdx >= 4) {
    statusLog[InvoiceNodeStatus.Delivered] = invoice.deliveredDatetime;
  }
  if (statusIdx >= 5) {
    statusLog[InvoiceNodeStatus.Verified] = invoice.verifiedDatetime;
  }

  return statusLog;
};

export const FooterComponent: FC = () => {
  const t = useTranslation('distribution');
  const { data } = useReturn.document.get();
  const { navigateUpOne } = useBreadcrumbs();

  return (
    <AppFooterPortal
      Content={
        data && (
          <Box
            gap={2}
            display="flex"
            flexDirection="row"
            alignItems="center"
            height={64}
          >
            <OnHoldButton />

            <StatusCrumbs
              statuses={outboundStatuses}
              statusLog={createStatusLog(data)}
              statusFormatter={getStatusTranslator(t)}
            />

            <Box flex={1} display="flex" justifyContent="flex-end" gap={2}>
              <ButtonWithIcon
                shrinkThreshold="lg"
                Icon={<XCircleIcon />}
                label={t('button.close')}
                color="secondary"
                sx={{ fontSize: '12px' }}
                onClick={() => navigateUpOne()}
              />

              <StatusChangeButton />
            </Box>
          </Box>
        )
      }
    />
  );
};

export const Footer = memo(FooterComponent);
