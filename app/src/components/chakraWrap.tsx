import React from 'react';
import { PropsWithChildren } from 'react';
import { ChakraProvider, ColorModeScript, extendTheme } from '@chakra-ui/react';

type ReactComponent<Props = {}> = React.FC<PropsWithChildren<Props>>;

const theme = extendTheme({
  config: {
    useSystemColorMode: false,
    initialColorMode: 'dark',
  },
});

export const ChakraWrap: ReactComponent = ({ children }) => {
  return (
    <ChakraProvider theme={theme}>
      <ColorModeScript initialColorMode={theme.config.initialColorMode} />
      {children}
    </ChakraProvider>
  );
};
