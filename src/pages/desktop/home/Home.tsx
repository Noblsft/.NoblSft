import { Box, Flex } from '@chakra-ui/react';
import './home.css';

import { Sidebar, Workspace } from '@/components';

export default function Home() {
  return (
    <Box className="home" bg="bg.panel" p={4}>
      <Flex gap={4}>
        <Sidebar />
        <Workspace />
      </Flex>
    </Box>
  );
}
