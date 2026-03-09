import { Suspense, useState } from 'react';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import ProcessorSubPage from './processor-sub-page';
import LocationSelect from './components/location-select';
import Button from '../../components/button';
import Spinner from '../../components/spinner';

const SelectLocations = (props: ProcessSubPageProps) => {
  const [locationSelection, setLocationSelection] = useState<
    Record<string, boolean>
  >({});

  const continueProcess = async () => {
    // await props.appActions.updateAppData({ subdomain: subdomainInput });
    props.appActions.advanceProcessor();
  };

  return (
    <ProcessorSubPage title="Select Locations">
      <Suspense
        fallback={
          <div className="h-full grid place-items-center">
            <div className="flex flex-col justify-center items-center gap-2">
              <Spinner />
              <p className="text-sandstone-400">
                Please wait while we load your locations...
              </p>
            </div>
          </div>
        }
      >
        <div className="space-y-2">
          <LocationSelect
            value={locationSelection}
            onChange={setLocationSelection}
          />
          <Button label="Save" style="primary" onClick={continueProcess} />
        </div>
      </Suspense>
    </ProcessorSubPage>
  );
};

export default SelectLocations;
