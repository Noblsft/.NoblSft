import { Services } from '@/services/types.ts';
import VaultService from '@/services/VaultService.ts';

export * from './types.ts';

export function createServices(): Services {
  return {
    vaultService: new VaultService(),
  };
}
