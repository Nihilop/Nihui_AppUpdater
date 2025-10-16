import { createI18n } from 'vue-i18n';
import { messages } from './locales';

export const i18n = createI18n({
  legacy: false, // Use Composition API mode
  locale: 'en', // Default locale (will be overridden by config)
  fallbackLocale: 'en',
  messages,
});
