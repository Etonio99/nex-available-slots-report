export type ProcessStep =
  | 'CheckApiKey'
  | 'EnterSubdomain'
  | 'SelectLocations'
  | 'EnterAppointmentTypeName'
  | 'EnterDays'
  | 'Confirm'
  | 'CollectContext'
  | 'CollectAnalytics'
  | 'Complete';
