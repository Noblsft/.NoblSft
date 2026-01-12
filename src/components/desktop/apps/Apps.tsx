import { Box, HStack, IconButton, Separator } from '@chakra-ui/react';
import { FaPlus } from 'react-icons/fa6';

export function Apps() {
  return (
    <Box mx='10px' my='2px'>
      <HStack gap={4}>
        <IconButton aria-label='add app' variant='surface' size='md'>
          <FaPlus />
        </IconButton>
        <Separator orientation='vertical' height='8' />
      </HStack>
    </Box>
  );
}
