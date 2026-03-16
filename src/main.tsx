import React from 'react';
import ReactDOM from 'react-dom/client';
import NotificationContextProvider from './components/contexts/notification-context';
import App from './app';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <NotificationContextProvider>
        <App />
      </NotificationContextProvider>
    </QueryClientProvider>
  </React.StrictMode>
);
