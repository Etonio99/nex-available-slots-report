import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react"

export const useProcessor = (autoAdvance: boolean) => {
    const [processorResponse, setProcessorResponse] = useState<any>(null);
    
    const advanceProcessor = async () => {
        const response = await invoke("advance_processor");
    }

    useEffect(() => {
        if (autoAdvance) {
            advanceProcessor();
        }
    }, []);

    return { processorResponse };
}