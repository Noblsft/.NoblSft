import { render } from '@testing-library/react';

import { ServicesProvider, useServices } from '../../hooks/useServices';

describe('useServices', () => {
  test('throws when used outside ServicesProvider', () => {
    function TestComp() {
      // This should throw when rendered because there is no provider
      useServices();
      return null;
    }

    // Suppress React error logging for cleaner test output
    const spy = jest.spyOn(console, 'error').mockImplementation(() => {});
    expect(() => render(<TestComp />)).toThrow('useServices must be used within ServicesProvider');
    spy.mockRestore();
  });

  test('returns services when used within ServicesProvider', () => {
    function TestComp() {
      const services = useServices();
      return <div data-testid='ok'>{services.vaultService ? 'ok' : 'no'}</div>;
    }

    const { getByTestId } = render(
      <ServicesProvider>
        <TestComp />
      </ServicesProvider>,
    );

    expect(getByTestId('ok').textContent).toBe('ok');
  });
});
