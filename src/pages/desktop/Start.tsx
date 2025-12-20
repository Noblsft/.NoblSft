import { VStack, Button, Image } from '@chakra-ui/react';
import { save } from '@tauri-apps/plugin-dialog';
import VaultService from '@/services/VaultService';
import logo from '@/assets/logo.png';

export default function Start() {
  const vaultService = new VaultService();

  const createNewVault = async () => {
    console.log('Creating vault service...');

    const path = await save({
      defaultPath: 'project.nobl',
      filters: [{ name: 'Nobl vault', extensions: ['nobl'] }],
    });

    if (!path) return;

    await vaultService.createVault(path);
  };

  return (
    <VStack>
      <Image src={logo} alt="logo" aspectRatio={4 / 3} width="350px" />
      <Button
        colorScheme="teal"
        size="lg"
        width="30%"
        onClick={async () => {
          await createNewVault();
        }}
      >
        Create new nobl file
      </Button>
      <Button colorScheme="blue" size="lg" width="30%">
        Load existing nobl file
      </Button>
    </VStack>
  );
}
