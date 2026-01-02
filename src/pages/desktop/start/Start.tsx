import { VStack, Image } from '@chakra-ui/react';
import { save, open } from '@tauri-apps/plugin-dialog';
import { useNavigate } from 'react-router-dom';

import logo from '@/assets/logo.png';
import { RoundedButton } from '@/components';
import { useServices } from '@/hooks/useServices.tsx';

export default function Start() {
  const { vaultService } = useServices();
  const navigate = useNavigate();

  const createNewVault = async () => {
    const path = await save({
      defaultPath: 'project.nobl',
      filters: [{ name: 'Nobl vault', extensions: ['nobl'] }],
    });

    if (!path) return;

    await vaultService.createVault(path);
  };

  const loadExistingVault = async () => {
    const path = await open({
      defaultPath: 'project.nobl',
      filters: [{ name: 'Nobl vault', extensions: ['nobl'] }],
    });

    if (!path) return;

    await vaultService.loadVault(path);
  };

  return (
    <VStack>
      <Image src={logo} alt="logo" aspectRatio={4 / 3} width="350px" />
      <RoundedButton
        width="30%"
        onClick={async () => {
          await createNewVault();
          navigate('/home');
        }}
      >
        Create new nobl file
      </RoundedButton>
      <RoundedButton
        width="30%"
        onClick={async () => {
          await loadExistingVault();
          navigate('/home');
        }}
      >
        Load existing nobl file
      </RoundedButton>
    </VStack>
  );
}
