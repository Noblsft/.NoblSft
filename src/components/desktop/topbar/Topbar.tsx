import { Box, Flex, HStack } from '@chakra-ui/react';
import { useColorModeValue } from '@/components';

export default function Topbar() {
  const borderColor = useColorModeValue('rgba(0,0,0,0.06)', 'rgba(255,255,255,0.04)');

  return (
    <Box as="header" position="sticky" top={0} zIndex={50} width="100%">
      <Box maxW="1400px" mx="auto" px={4} height="40px">
        <Flex align="center" height="100%">
          <HStack gap={2} align="center">
            <Box
              as="button"
              aria-label="Close"
              width="12px"
              height="12px"
              minW="12px"
              p={0}
              borderRadius="full"
              bg="#ff605c"
              border={`1px solid ${borderColor}`}
              cursor="pointer"
              _hover={{ filter: 'brightness(1.05)' }}
              _focus={{ boxShadow: 'outline' }}
            />

            <Box
              as="button"
              aria-label="Minimize"
              width="12px"
              height="12px"
              minW="12px"
              p={0}
              borderRadius="full"
              bg="#ffbf2e"
              border={`1px solid ${borderColor}`}
              cursor="pointer"
              _hover={{ filter: 'brightness(1.05)' }}
              _focus={{ boxShadow: 'outline' }}
            />

            <Box
              as="button"
              aria-label="Maximize"
              width="12px"
              height="12px"
              minW="12px"
              p={0}
              borderRadius="full"
              bg="#2fd55a"
              border={`1px solid ${borderColor}`}
              cursor="pointer"
              _hover={{ filter: 'brightness(1.05)' }}
              _focus={{ boxShadow: 'outline' }}
            />
          </HStack>
        </Flex>
      </Box>
    </Box>
  );
}
