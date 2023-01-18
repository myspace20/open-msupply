import React, { FC } from 'react';
import {
  Badge,
  ListItem,
  ListItemIcon,
  ListItemText,
  ListItemButton,
  Box,
  ListItemProps,
  BadgeProps,
} from '@mui/material';
import { styled } from '@mui/material/styles';
import { useMatch, Link } from 'react-router-dom';
import { useDrawer } from '@common/hooks';
import { ChevronDownIcon } from '@common/icons';

const useSelectedNavMenuItem = (
  to: string,
  end: boolean,
  isOpen: boolean
): boolean => {
  // This nav menu item should be selected when lower level elements
  // are selected. For example, the route /outbound-shipment/{id} should
  // highlight the nav menu item for outbound-shipments.
  // If the drawer is closed, highlight the higher level elements.
  const highlightLowerLevels = isOpen ? !end || to.endsWith('*') : false;
  // If we need to highlight the higher levels append a wildcard to the match path.
  const path = highlightLowerLevels ? to : `${to}/*`;
  const selected = useMatch({ path });
  return !!selected;
};

const getListItemCommonStyles = () => ({
  height: 40,
  borderRadius: 20,
  alignItems: 'center',
});

const StyledListItem = styled<
  FC<ListItemProps & { isSelected: boolean; to: string }>
>(ListItem, {
  shouldForwardProp: prop => prop !== 'isSelected',
})(({ theme, isSelected }) =>
  isSelected
    ? {
        ...getListItemCommonStyles(),
        backgroundColor: theme.mixins.drawer.selectedBackgroundColor,
        boxShadow: theme.shadows[3],
        fontWeight: 'bold',
        marginTop: 5,
      }
    : {
        ...getListItemCommonStyles(),
        backgroundColor: 'transparent',
        marginTop: 5,
        '&:hover': {
          boxShadow: theme.shadows[3],
          backgroundColor: theme.palette.background.toolbar,
        },
      }
);

export interface AppNavLinkProps {
  badgeProps?: BadgeProps;
  end?: boolean; // denotes lowest level menu item, using terminology from useMatch
  icon?: JSX.Element;
  inactive?: boolean;
  text?: string;
  to: string;
  visible?: boolean;
  onClick?: () => void;
}

export const AppNavLink: FC<AppNavLinkProps> = props => {
  const {
    badgeProps,
    end,
    inactive,
    icon = <span style={{ width: 2 }} />,
    text,
    to,
    visible = true,
    onClick,
  } = props;
  const drawer = useDrawer();

  const selected = useSelectedNavMenuItem(to, !!end, drawer.isOpen);
  const isSelectedParentItem = inactive && !!useMatch({ path: `${to}/*` });
  const handleClick = () => {
    // reset the clicked nav path when navigating
    // otherwise the child menu remains open
    drawer.setClickedNavPath(undefined);
    if (onClick) onClick();
    drawer.onClick();
  };

  const CustomLink = React.useMemo(
    () =>
      React.forwardRef<HTMLAnchorElement>((linkProps, ref) =>
        !end && !!inactive ? (
          <span
            {...linkProps}
            onClick={() => drawer.onExpand(to)}
            data-testid={`${to}_hover`}
          />
        ) : (
          <Link
            {...linkProps}
            ref={ref}
            to={to}
            role="link"
            aria-label={text}
            title={text}
            onClick={handleClick}
          />
        )
      ),
    [to]
  );

  return visible ? (
    <StyledListItem
      isSelected={selected}
      to={to}
      sx={
        inactive
          ? { width: '220px' }
          : {
              maxWidth: 165,
              '& .MuiTypography-root': {
                maxWidth: 130,
              },
            }
      }
    >
      <ListItemButton
        sx={{
          ...getListItemCommonStyles(),
          justifyContent: drawer.isOpen ? 'flex-start' : 'center',
          '&.MuiListItemButton-root:hover': {
            backgroundColor: 'transparent',
          },
          '& .MuiTypography-root': {
            textOverflow: 'ellipsis',
            overflow: 'hidden',
            whiteSpace: 'nowrap',
          },
        }}
        disableGutters
        component={CustomLink}
      >
        <ListItemIcon sx={{ minWidth: 20 }}>{icon}</ListItemIcon>
        {inactive && drawer.isOpen && (
          <ChevronDownIcon
            className="menu_section_icon"
            sx={{
              color: 'gray.main',
              fontSize: '1rem',
              marginLeft: 0.5,
              stroke: theme => theme.palette.gray.main,
              strokeWidth: 1.5,
              transform: 'rotate(-90deg)',
            }}
          />
        )}
        <Box className="navLinkText">
          {!end && <Box width={4} />}

          <Badge
            {...badgeProps}
            sx={{
              alignItems: 'center',
              flexGrow: 1,
              '& .MuiBadge-badge': drawer.isOpen
                ? {
                    transform: 'scale(0.75) translate(75%, -25%)',
                  }
                : {
                    top: 'unset',
                    transform: 'scale(0.75) translate(50%, -50%)',
                  },
            }}
          >
            <ListItemText
              primary={text}
              sx={{
                '& .MuiTypography-root': {
                  fontWeight:
                    selected || isSelectedParentItem ? 'bold' : 'normal',
                  color: isSelectedParentItem ? 'primary.main' : undefined,
                },
                flexGrow: 0,
              }}
            />
          </Badge>

          <ListItemIcon
            sx={{
              minWidth: 20,
              display: selected && drawer.isOpen ? 'flex' : 'none',
              alignItems: 'center',
            }}
            className="chevron"
          >
            <ChevronDownIcon
              sx={{
                transform: 'rotate(-90deg)',
                fontSize: '1rem',
                color: 'primary.main',
              }}
            />
          </ListItemIcon>
        </Box>
      </ListItemButton>
    </StyledListItem>
  ) : null;
};
