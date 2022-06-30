import { Container } from '@chakra-ui/react';
import React from 'react';
import { Header } from './components/header';

export const App: React.FC = () => {
  return (
    <Container maxW='container.md'>
      <Header />
    </Container>
  );
};
