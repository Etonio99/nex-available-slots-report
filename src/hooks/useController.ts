import { invoke } from '@tauri-apps/api/core';
import { ProcessorAdvanceResult } from '../types/processor-advance-result';
import { ProcessorDataUpdate } from '../types/processor-data-update';

export const useController = () => {
  const setSubdomain = async (subdomain: string) => {};

  const getLocations = async (processorName: string): Promise<boolean> =>
    invoke('set_processor', {
      processorName,
    })
      .then(() => true)
      .catch(() => false);

  const advanceProcessor = async (): Promise<
    ProcessorAdvanceResult | undefined
  > =>
    invoke<ProcessorAdvanceResult>('advance_processor')
      .then((response) => response)
      .catch(() => undefined);

  const updateProcessorData = async (
    data: ProcessorDataUpdate
  ): Promise<boolean> =>
    invoke('update_processor_data', { data })
      .then(() => true)
      .catch(() => false);

  return {
    setProcessor,
    advanceProcessor,
    updateProcessorData,
  };
};
