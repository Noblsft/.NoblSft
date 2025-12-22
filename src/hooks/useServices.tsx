import * as React from 'react';
import type { Services } from '@/services/types.ts';
import { createServices } from '@/services';

const ServicesContext = React.createContext<Services | null>(null);
const services: Services = createServices();

export function ServicesProvider({ children }: { children: React.ReactNode }) {
  return <ServicesContext.Provider value={services}>{children}</ServicesContext.Provider>;
}

export function useServices(): Services {
  const ctx = React.useContext(ServicesContext);
  if (!ctx) throw new Error('useServices must be used within ServicesProvider');
  return ctx;
}
