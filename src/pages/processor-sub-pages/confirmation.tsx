import Button from '../../components/button';
import ProcessorSubPage from './processor-sub-page';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { errorMessages } from '../../types/processor-error';
import { snakeCaseToTitleCase } from '../../utils/string-helper';

const Confirmation = (props: ProcessSubPageProps) => {
  const continueProcess = async () => {
    await props.appActions.updateProcessorData({ confirmed: true });
    await props.appActions.advanceProcessor();
  };

  const confirmationData =
    props.advanceResult?.error?.resolutionData?.type === 'CONFIRMATION'
      ? props.advanceResult.error.resolutionData.payload
      : null;

  console.log(confirmationData);

  return (
    <ProcessorSubPage title="Confirmation">
      <p>
        Please confirm that all of the entered information is correct before
        proceeding.
      </p>
      <div className="bg-sandstone-200 rounded-lg p-4">
        <ul>
          {confirmationData &&
            Object.entries(confirmationData).map(([key, value]) => (
              <li className="flex justify-between">
                <span className="text-sandstone-600">
                  {snakeCaseToTitleCase(key)}:{' '}
                </span>
                {value}
              </li>
            ))}
        </ul>
      </div>
      <div className="mt-2 flex justify-end items-center gap-2">
        <Button label="Confirm" style="primary" onClick={continueProcess} />
        <Button label="Cancel" style="tertiary" />
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

export default Confirmation;
