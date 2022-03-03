import React, { FC } from 'react';
import { AppThemeProvider } from '@common/styles';
import { SupportedLocales } from '@common/intl';
import mediaQuery from 'css-mediaquery';
import { SnackbarProvider } from 'notistack';
import { QueryClientProvider, QueryClient } from 'react-query';
import { MemoryRouter, Routes, Route } from 'react-router-dom';
import { TableProvider, createTableStore } from '../ui/layout/tables';
import { IntlTestProvider, OmSupplyApiProvider } from '..';
import { Environment } from '@openmsupply-client/config';
import { ConfirmationModalProvider } from '../ui/components/modals';
import {
  renderHook,
  RenderHookOptions,
  RenderHookResult,
} from '@testing-library/react-hooks';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // ✅ turns retries off
      retry: false,
    },
  },
});

interface StoryProviderProps {
  locale?: SupportedLocales;
}

interface TestingRouterProps {
  initialEntries: string[];
}

export const TestingRouter: FC<TestingRouterProps> = ({
  children,
  initialEntries,
}) => (
  <MemoryRouter initialEntries={initialEntries}>
    <Routes>{children}</Routes>
  </MemoryRouter>
);

export const TestingRouterContext: FC = ({ children }) => (
  <TestingRouter initialEntries={['/testing']}>
    <Route path="/testing" element={<>{children}</>} />
  </TestingRouter>
);

export const TestingProvider: FC<{ locale?: 'en' | 'fr' | 'ar' }> = ({
  children,
  locale = 'en',
}) => (
  <React.Suspense fallback={<span>?</span>}>
    <QueryClientProvider client={queryClient}>
      <OmSupplyApiProvider url={Environment.API_URL}>
        <SnackbarProvider maxSnack={3}>
          <IntlTestProvider locale={locale}>
            <TableProvider createStore={createTableStore}>
              <AppThemeProvider>{children}</AppThemeProvider>
            </TableProvider>
          </IntlTestProvider>
        </SnackbarProvider>
      </OmSupplyApiProvider>
    </QueryClientProvider>
  </React.Suspense>
);

export const StoryProvider: FC<StoryProviderProps> = ({ children }) => (
  <QueryClientProvider client={queryClient}>
    <OmSupplyApiProvider url={Environment.API_URL}>
      <SnackbarProvider maxSnack={3}>
        <IntlTestProvider locale="en">
          <TableProvider createStore={createTableStore}>
            <AppThemeProvider>
              <ConfirmationModalProvider>{children}</ConfirmationModalProvider>
            </AppThemeProvider>
          </TableProvider>
        </IntlTestProvider>
      </SnackbarProvider>
    </OmSupplyApiProvider>
  </QueryClientProvider>
);

function createMatchMedia(width: number) {
  return (query: any) => ({
    matches: mediaQuery.match(query, { width }),
    media: query,
    onchange: null,
    addListener: jest.fn(),
    removeListener: jest.fn(),
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  });
}

export const setScreenSize_ONLY_FOR_TESTING = (screenSize: number): void => {
  window.matchMedia = createMatchMedia(screenSize);
};

export const renderHookWithProvider = <Props, Result>(
  hook: (props: Props) => Result,
  options?: {
    renderHookOptions?: RenderHookOptions<Props>;
    providerProps?: { locale: 'en' | 'fr' | 'ar' };
  }
): RenderHookResult<Props, Result> =>
  renderHook(hook, {
    ...options?.renderHookOptions,
    wrapper: ({ children }) => (
      <TestingProvider {...options?.providerProps}>{children}</TestingProvider>
    ),
  });
