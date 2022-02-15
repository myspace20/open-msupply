import React, { FC, useState } from 'react';
import { Grid, Paper, Typography } from '@mui/material';
import { Story } from '@storybook/react';
import { FlatButton } from './FlatButton';
import { SplitButton } from './SplitButton';
import { BookIcon, TruckIcon } from '@common/icons';
import { BaseButton, ButtonWithIcon } from '.';
import { DialogButton, IconButton } from '../buttons';
import { Color } from '../menus';
import { ToggleButton } from './ToggleButton';
import { ColorSelectButton } from './ColorSelectButton';
import { useTranslation } from '@common/intl';
import { SplitButtonOption } from '@common/components';

const ops: [
  SplitButtonOption<string>,
  SplitButtonOption<string>,
  SplitButtonOption<string>
] = [
  { label: 'Create a merge commit', value: 'createAndMerge' },
  { label: 'Squash and merge', value: 'squashAndMerge' },
  { label: 'Rebase and merge', value: 'rebaseAndMerge' },
];

const getOnClick = (someText: string) => () => {
  alert(someText);
};

const Wrapper: FC<{ text: string }> = ({ children, text }) => {
  return (
    <Grid item>
      <Paper
        sx={{
          width: 300,
          height: 180,
          justifyContent: 'center',
          alignItems: 'center',
          display: 'flex',
          flexDirection: 'column',
        }}
      >
        <Typography sx={{ marginBottom: 2 }} variant="subtitle1">
          {text}
        </Typography>
        {children}
      </Paper>
    </Grid>
  );
};

const Template: Story<{ color: 'primary' | 'secondary' }> = ({ color }) => {
  const t = useTranslation(['app', 'common']);
  const [selected, setSelected] = useState(false);
  const [selectedColor, setColor] = useState<Color>({
    hex: '#8f90a6',
    name: 'grey',
  });
  const [selectedOption, setSelectedOption] = React.useState<
    SplitButtonOption<string>
  >(ops[0]);

  return (
    <>
      <Grid container gap={2}>
        <Wrapper text="Base Button: Outlined variant, primary color">
          <BaseButton
            variant="outlined"
            color={color}
            onClick={getOnClick('Base button')}
          >
            Base Button
          </BaseButton>
        </Wrapper>

        <Wrapper text="Base Button: Contained variant, primary color">
          <BaseButton
            variant="contained"
            color={color}
            onClick={getOnClick('Base button')}
          >
            Base Button
          </BaseButton>
        </Wrapper>

        <Wrapper text="Button with Icon, contained & primary">
          <ButtonWithIcon
            variant="contained"
            color={color}
            Icon={<TruckIcon />}
            label={t('distribution')}
            onClick={getOnClick('With Icon!')}
          />
        </Wrapper>

        <Wrapper text="Button with Icon, outlined & primary">
          <ButtonWithIcon
            variant="outlined"
            color={color}
            Icon={<TruckIcon />}
            label={t('distribution')}
            onClick={getOnClick('With Icon!')}
          />
        </Wrapper>

        <Wrapper text="Dialog OK button">
          <DialogButton variant="ok" onClick={getOnClick('OK!')} />
        </Wrapper>

        <Wrapper text="Dialog OK & Next button">
          <DialogButton variant="next" onClick={getOnClick('OK & Next!')} />
        </Wrapper>

        <Wrapper text="Dialog cancel button">
          <DialogButton variant="cancel" onClick={getOnClick('Cancel!')} />
        </Wrapper>

        <Wrapper text="Flat button">
          <FlatButton
            color={color}
            startIcon={<BookIcon />}
            label="Docs"
            onClick={() => console.info('clicked')}
          />
        </Wrapper>

        <Wrapper text="Icon button">
          <IconButton
            icon={<BookIcon />}
            label="Docs"
            onClick={() => console.info('clicked')}
          />
        </Wrapper>

        <Wrapper text="Toggle button">
          <ToggleButton
            value={selected}
            selected={selected}
            onClick={() => setSelected(state => !state)}
            label="Admin"
          />
        </Wrapper>

        <Wrapper text="Color select">
          <Typography>
            Selected color: {JSON.stringify(selectedColor)}
          </Typography>
          <ColorSelectButton
            color={selectedColor.hex}
            onChange={newColor => setColor(newColor)}
          />
        </Wrapper>

        <Wrapper text="Split button">
          <SplitButton
            color={color}
            options={ops}
            onClick={option => alert(JSON.stringify(option))}
            selectedOption={selectedOption}
            onSelectOption={setSelectedOption}
          />
        </Wrapper>
      </Grid>
    </>
  );
};

export const Primary = Template.bind({}, { color: 'primary' });
export const Secondary = Template.bind({}, { color: 'secondary' });

export default {
  title: 'Buttons/ButtonShowcase',
};
