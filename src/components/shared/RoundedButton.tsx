import { Button } from '@chakra-ui/react';
type RoundedButtonProps = {
  children: string;
  onClick?: () => void;
  width?: number | string;
};

export function RoundedButton({ children, width, onClick }: RoundedButtonProps) {
  return (
    <Button onClick={onClick} width={width} rounded='10px' variant='surface'>
      {children}
    </Button>
  );
}
