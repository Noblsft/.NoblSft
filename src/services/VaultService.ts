import { invoke } from '@tauri-apps/api/core';

export default class VaultService {
  public async createVault(path: string): Promise<void> {
    await invoke('create_vault', { path });
  }

  public async loadVault(path: string): Promise<void> {
    await invoke('load_vault', { path });
  }
}
