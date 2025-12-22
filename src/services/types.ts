export type Services = {
  vaultService: {
    createVault: (path: string) => Promise<void>;
    loadVault: (path: string) => Promise<void>;
  };
};
