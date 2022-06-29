import React from 'react';
import { MdOutlineDarkMode } from 'react-icons/md';
import { FiSun } from 'react-icons/fi';
import { Flex, Heading, IconButton, useColorMode } from '@chakra-ui/react';

export const Header: React.FC = () => {
  const { colorMode, toggleColorMode } = useColorMode();

  return (
    <Flex mt={10} alignItems='center' justifyContent='space-between'>
      <Heading fontSize='2xl'>Todo</Heading>
      <IconButton
        aria-label='Toggle theme'
        icon={colorMode === 'light' ? <MdOutlineDarkMode /> : <FiSun />}
        onClick={toggleColorMode}
        variant='ghost'
      />
    </Flex>
  );
};
