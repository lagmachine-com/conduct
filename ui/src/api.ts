import { SummaryResponse } from "./bindings/summary_response"

declare global {
    interface Window {
        conduct: {
            get: (path: string) => Promise<Response>
        }
    }
}

export function get(path: string) {
    return window.conduct.get(path)
}

export async function getSummary(): Promise<SummaryResponse> {
    let result = await window.conduct.get("api/command/summary")
    return await result.json() as SummaryResponse
}