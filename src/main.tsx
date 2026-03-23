import React from 'react';
import ReactDOM from 'react-dom/client';
import NotificationContextProvider from './components/contexts/notification-context';
import ModalContextProvider from './components/contexts/modal-context';
import App from './app';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <NotificationContextProvider>
      <ModalContextProvider>
        <App />
      </ModalContextProvider>
    </NotificationContextProvider>
  </React.StrictMode>
);
