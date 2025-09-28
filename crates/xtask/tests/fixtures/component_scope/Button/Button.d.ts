import * as React from 'react';

export interface ButtonOwnProps {
  /**
   * Friendly display label rendered inside the button.
   */
  label: string;
  /**
   * Optional color palette variant.
   */
  color?: 'primary' | 'secondary';
  /**
   * Called whenever the user activates the control.
   */
  onClick?: (event: React.MouseEvent<HTMLButtonElement>) => void;
}
