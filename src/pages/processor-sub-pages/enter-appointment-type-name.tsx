import { useState } from 'react';
import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import Input from '../../components/input';
import { BiRename } from 'react-icons/bi';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { errorMessages } from '../../types/processor-error';

const EnterAppointmentTypeName = (props: ProcessSubPageProps) => {
  const [appointmentTypeName, setAppointmentTypeName] = useState<string>('');

  const continueProcess = async () => {
    if (!appointmentTypeName) {
      return;
    }

    await props.appActions.updateProcessorData({
      appointment_type_name: appointmentTypeName,
    });
    await props.appActions.advanceProcessor();
  };

  return (
    <ProcessorSubPage title="Enter Appointment Type Name">
      <Input
        label="Appointment Type Name"
        placeholder="New Patient Cleaning"
        icon={<BiRename />}
        value={appointmentTypeName}
        onChange={(e) => setAppointmentTypeName(e.target.value)}
      />
      <div className="mt-2 flex justify-end items-center gap-2">
        <Button label="Save" style="primary" onClick={continueProcess} />
      </div>
      {props.advanceResult && (
        <p className="text-red-400 w-full text-center">
          {
            errorMessages[
              props.advanceResult.error?.type as keyof typeof errorMessages
            ]
          }
        </p>
      )}
    </ProcessorSubPage>
  );
};

export default EnterAppointmentTypeName;
