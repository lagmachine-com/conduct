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
    let result = await window.conduct.get("api/v1/command/summary")
    return await result.json() as SummaryResponse
}

export async function doExport(): Promise<SummaryResponse> {
    let result = await window.conduct.get("api/v1/command/export?asset=suzanneA&department=model")
    return await result.json() as SummaryResponse
}

export async function doCreate(): Promise<SummaryResponse> {
    let result = await window.conduct.get("api/v1/command/create?asset=suzanneA&department=model")
    return await result.json() as SummaryResponse
}

export async function exitDialog(): Promise<SummaryResponse> {
    let result = await window.conduct.get("api/v1/dialog/exit")
    return await result.json() as SummaryResponse
}