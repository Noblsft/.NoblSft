import { createSystem, defaultConfig, defineConfig } from '@chakra-ui/react';

const config = defineConfig({
  theme: {
    tokens: {
      fonts: {
        heading: { value: `'Inter', system-ui, -apple-system, Segoe UI, Roboto, sans-serif` },
        body: { value: `'Inter', system-ui, -apple-system, Segoe UI, Roboto, sans-serif` },
      },

      radii: {
        sm: { value: '10px' },
        md: { value: '14px' },
        lg: { value: '18px' },
      },

      colors: {
        neutral: {
          0: { value: '#ffffff' },
          50: { value: '#f7f7f7' },
          100: { value: '#efefef' },
          200: { value: '#e0e0e0' },
          300: { value: '#cfcfcf' },
          400: { value: '#bfbfbf' },
          500: { value: '#9f9f9f' },
          600: { value: '#808080' },
          700: { value: '#606060' },
          800: { value: '#404040' },
          900: { value: '#202020' },
          950: { value: '#000000' },
        },

        brand: {
          50: { value: '#eef2ff' },
          100: { value: '#e0e7ff' },
          200: { value: '#c7d2fe' },
          300: { value: '#a5b4fc' },
          400: { value: '#818cf8' },
          500: { value: '#6366f1' },
          600: { value: '#4f46e5' },
          700: { value: '#4338ca' },
          800: { value: '#3730a3' },
          900: { value: '#312e81' },
        },

        success: {
          50: { value: '#ecfdf5' },
          500: { value: '#10b981' },
          600: { value: '#059669' },
        },
        warning: {
          50: { value: '#fffbeb' },
          500: { value: '#f59e0b' },
          600: { value: '#d97706' },
        },
        danger: {
          50: { value: '#fef2f2' },
          500: { value: '#ef4444' },
          600: { value: '#dc2626' },
        },
      },
    },

    semanticTokens: {
      colors: {
        // Surfaces
        bg: {
          DEFAULT: { value: { _light: '{colors.neutral.50}', _dark: '{colors.neutral.950}' } },
          panel: { value: { _light: '{colors.neutral.0}', _dark: '{colors.neutral.900}' } },
          muted: { value: { _light: '{colors.neutral.100}', _dark: '{colors.neutral.900}' } },
          subtle: { value: { _light: '{colors.neutral.0}', _dark: '{colors.neutral.900}' } },
          emphasized: { value: { _light: '{colors.neutral.200}', _dark: '{colors.neutral.800}' } },
        },

        // Text
        fg: {
          DEFAULT: { value: { _light: '{colors.neutral.900}', _dark: '{colors.neutral.50}' } },
          muted: { value: { _light: '{colors.neutral.600}', _dark: '{colors.neutral.400}' } },
          subtle: { value: { _light: '{colors.neutral.500}', _dark: '{colors.neutral.500}' } },
        },

        // Borders
        border: {
          DEFAULT: { value: { _light: '{colors.neutral.200}', _dark: '{colors.neutral.800}' } },
          subtle: { value: { _light: '{colors.neutral.100}', _dark: '{colors.neutral.900}' } },
        },

        // Brand roles
        primary: {
          DEFAULT: { value: '{colors.brand.500}' },
          hover: { value: '{colors.brand.600}' },
          active: { value: '{colors.brand.700}' },
          subtle: { value: { _light: '{colors.brand.50}', _dark: '{colors.brand.900}' } },
        },

        // Status roles
        success: { value: '{colors.success.500}' },
        warning: { value: '{colors.warning.500}' },
        danger: { value: '{colors.danger.500}' },
      },
    },
  },
});

export const system = createSystem(defaultConfig, config);
