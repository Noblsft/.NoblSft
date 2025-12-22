import { invoke } from '@tauri-apps/api/core';
jest.mock('@tauri-apps/api/core', () => ({ invoke: jest.fn() }));

import VaultService from '../../services/VaultService';

const invokeMock = invoke as unknown as jest.Mock;

describe('VaultService', () => {
  const service = new VaultService();

  beforeEach(() => {
    invokeMock.mockReset();
  });

  test('createVault calls invoke with "create_vault" and path', async () => {
    invokeMock.mockResolvedValue(undefined);
    const path = '/some/path';
    await expect(service.createVault(path)).resolves.toBeUndefined();
    expect(invokeMock).toHaveBeenCalledWith('create_vault', { path });
  });

  test('loadVault calls invoke with "load_vault" and path', async () => {
    invokeMock.mockResolvedValue(undefined);
    const path = '/other/path';
    await expect(service.loadVault(path)).resolves.toBeUndefined();
    expect(invokeMock).toHaveBeenCalledWith('load_vault', { path });
  });

  test('createVault propagates errors from invoke', async () => {
    invokeMock.mockRejectedValue(new Error('invoke failed'));
    await expect(service.createVault('/fail')).rejects.toThrow('invoke failed');
  });

  test('loadVault propagates errors from invoke', async () => {
    invokeMock.mockRejectedValue(new Error('invoke failed'));
    await expect(service.loadVault('/fail')).rejects.toThrow('invoke failed');
  });
});
