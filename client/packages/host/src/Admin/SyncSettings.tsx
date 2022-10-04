import React, { useEffect, useState } from 'react';
import { useTranslation } from '@common/intl';
import {
  BasicTextInput,
  CircularProgress,
  ErrorWithDetails,
  ErrorWithDetailsProps,
  Grid,
  LoadingButton,
  NumericTextInput,
  SaveIcon,
  SyncSettingsInput,
  Typography,
  useNotification,
} from '@openmsupply-client/common';
import { useHost } from '../api/hooks';
import { Setting } from './Setting';
import { mapSyncError } from '../api/api';

const isValid = (syncSettings: SyncSettingsInput | null) => {
  if (!syncSettings) return false;
  return (
    !!syncSettings.url &&
    !!syncSettings.username &&
    !!syncSettings.password &&
    !!syncSettings.intervalSeconds
  );
};

const SyncSettingsForm = ({
  isDisabled,
  isSaving,
  isValid,
  settings,
  onSave,
  setSyncSettings,
}: {
  isDisabled: boolean;
  isSaving: boolean;
  isValid: boolean;
  settings: SyncSettingsInput;
  onSave: () => void;
  setSyncSettings: (syncSettings: SyncSettingsInput) => void;
}) => {
  const t = useTranslation('common');
  const setSettings = (
    property: keyof SyncSettingsInput,
    value: number | string
  ) => setSyncSettings({ ...settings, [property]: value });

  const { url, username, password, intervalSeconds } = settings;
  return (
    <form style={{ width: '100%' }}>
      <Setting
        title={t('label.settings-url')}
        component={
          <BasicTextInput
            value={url}
            onChange={e => setSettings('url', e.target.value)}
            type={'text'}
            disabled={isDisabled}
          />
        }
      />
      <Setting
        title={t('label.settings-username')}
        component={
          <BasicTextInput
            value={username}
            onChange={e => setSettings('username', e.target.value)}
            type={'text'}
            disabled={isDisabled}
          />
        }
      />
      <Setting
        title={t('label.settings-password')}
        component={
          <BasicTextInput
            value={password}
            inputProps={{ autocomplete: 'sync-password' }}
            onChange={e => setSettings('password', e.target.value)}
            type={'password'}
            disabled={isDisabled}
          />
        }
      />
      <Setting
        title={t('label.settings-interval')}
        component={
          <NumericTextInput
            value={intervalSeconds}
            onChange={e =>
              setSettings('intervalSeconds', Number(e.target.value))
            }
            disabled={isDisabled}
          />
        }
      />
      <Grid item justifyContent="flex-end" width="100%" display="flex">
        <LoadingButton
          isLoading={isSaving}
          startIcon={<SaveIcon />}
          variant="contained"
          sx={{ fontSize: '12px' }}
          disabled={!isValid}
          onClick={onSave}
        >
          {t('button.save')}
        </LoadingButton>
      </Grid>
    </form>
  );
};

interface UpdateSyncSettingsState {
  syncSettings: SyncSettingsInput | null;
  error: ErrorWithDetailsProps | null;
  isSaving: boolean;
}

const useUpdateSyncSettingsState = () => {
  const [state, set] = useState<UpdateSyncSettingsState>({
    syncSettings: null,
    error: null,
    isSaving: false,
  });

  return {
    ...state,
    setSyncSettings: (syncSettings: UpdateSyncSettingsState['syncSettings']) =>
      set(state => ({ ...state, syncSettings })),
    setError: (error: UpdateSyncSettingsState['error']) =>
      set(state => ({ ...state, error })),
    setIsSaving: (isSaving: UpdateSyncSettingsState['isSaving']) =>
      set(state => ({ ...state, isSaving })),
  };
};

export const SyncSettings = ({}) => {
  const t = useTranslation('app');
  const { data, isError } = useHost.utils.syncSettings();
  const { mutateAsync: update } = useHost.sync.update();
  const {
    syncSettings,
    error,
    isSaving,
    setError,
    setIsSaving,
    setSyncSettings,
  } = useUpdateSyncSettingsState();
  const { success } = useNotification();

  useEffect(() => {
    if (data) {
      setSyncSettings({ ...data, password: '' });
    }
  }, [data]);

  const onSave = async () => {
    if (!syncSettings) return;
    setIsSaving(true);
    setError(null);
    try {
      const response = await update(syncSettings);
      // Map structured error
      if (response.__typename === 'SyncErrorNode') {
        setError(mapSyncError(t, response, 'error.unable-to-save-settings'));
        return setIsSaving(false);
      }
    } catch (e) {
      // Set standard error
      setError({
        error: t('error.unable-to-save-settings'),
        details: (e as Error)?.message || '',
      });
      return setIsSaving(false);
    }
    setIsSaving(false);
    success(t('success.sync-settings'))();
  };

  return (
    <Grid container>
      <Typography variant="h5" color="primary" style={{ paddingBottom: 25 }}>
        {t('heading.settings-sync')}
      </Typography>
      {!syncSettings ? (
        <Grid item justifyContent="center" width="100%" display="flex">
          <CircularProgress size={20} />
        </Grid>
      ) : (
        <>
          <SyncSettingsForm
            setSyncSettings={setSyncSettings}
            isDisabled={isError}
            isSaving={isSaving}
            isValid={isValid(syncSettings)}
            settings={syncSettings}
            onSave={onSave}
          />
          {error && <ErrorWithDetails {...error} />}
        </>
      )}
    </Grid>
  );
};
