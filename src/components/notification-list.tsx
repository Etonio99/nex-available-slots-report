import { AppNotification } from '../types/app-notification';

interface NotificationListProps {
  notifications: AppNotification[];
  dismiss: (id: string) => void;
}

const NotificationList = (props: NotificationListProps) => {
  return (
    <div className="absolute top-0 right-0 z-100">
      {props.notifications.map((notification) => (
        <div
          className="cursor-pointer bg-sandstone-900 text-sandstone-50 p-4 m-2 rounded-md shadow shadow-sandstone-950/20 w-80 slide-in-animation"
          onClick={() => props.dismiss(notification.id)}
        >
          <h3 className="text-sandstone-300 font-bold">{notification.title}</h3>
          <p className="text-sm">{notification.message}</p>
        </div>
      ))}
    </div>
  );
};

export default NotificationList;
