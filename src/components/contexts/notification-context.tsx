'use client';

import { createContext, useContext, useState } from 'react';
import { AppNotification } from '../../types/app-notification';
import NotificationList from '../notification-list';

type NotificationContextValue = {
  notify: (title: string, message: string) => void;
  dismiss: (id: string) => void;
};

const NotificationContext = createContext<NotificationContextValue | null>(
  null
);

interface NotificationContextProviderProps {
  children: React.ReactNode;
}

export default function NotificationContextProvider(
  props: NotificationContextProviderProps
) {
  const [notifications, setNotifications] = useState<AppNotification[]>([]);

  function notify(title: string, message: string) {
    const id = crypto.randomUUID();

    setNotifications((n) => [...n, { id, title, message }]);

    setTimeout(() => {
      dismiss(id);
    }, 4000);
  }

  function dismiss(id: string) {
    setNotifications((n) => n.filter((n) => n.id !== id));
  }

  return (
    <NotificationContext.Provider
      value={{
        notify,
        dismiss,
      }}
    >
      <NotificationList notifications={notifications} dismiss={dismiss} />
      {props.children}
    </NotificationContext.Provider>
  );
}

export const useNotificationContext = () => {
  const context = useContext(NotificationContext);
  if (!context) {
    throw new Error(
      'useNotificationContext must be used within a NotificationContextProvider'
    );
  }
  return context;
};
