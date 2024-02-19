import { useQueryClient, useMutation } from '@openmsupply-client/common';
import { useReturnsApi } from '../utils/useReturnsApi';

export const useInsertInboundReturn = () => {
  const queryClient = useQueryClient();
  // const navigate = useNavigate();
  const api = useReturnsApi();
  return useMutation(api.insertInboundReturn, {
    onSuccess: () => {
      // TODO: redirect to details page
      // onSuccess: invoiceNumber => {
      // const route = RouteBuilder.create(AppRoute.Replenishment)
      //   .addPart(AppRoute.InboundShipment)
      //   .addPart(String(invoiceNumber))
      //   .build();
      // navigate(route, { replace: true });
      return queryClient.invalidateQueries(api.keys.base());
    },
  });
};
