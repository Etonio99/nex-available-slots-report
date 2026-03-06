import { ProcessorAdvanceResult } from './processor-advance-result';
import { ProcessorDataUpdate } from './processor-data-update';

export interface ProcessSubPageProps {
  advance: () => Promise<boolean>;
  update: (data: ProcessorDataUpdate) => Promise<boolean>;
  advanceResult: ProcessorAdvanceResult | undefined;
}
