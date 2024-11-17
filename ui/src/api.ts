import { ListAssetsResult, ListShotsResult, SetupResult, SummaryResponse } from "./bindings/bindings_gen";


declare global {
    interface Window {
        conduct: {
            get: (path: string) => Promise<Response>,
            post: (path: string, body: string) => Promise<Response>
        }
    }
}

export interface ErrorResponse { error: string };

export function isError(object: any): object is ErrorResponse {

    if (object == undefined) {
        return false;
    }

    return object.hasOwnProperty("error")
}

export function get(path: string, args: Record<string, any> | null = null) {
    if (args != null) {
        let args_clean = Object.fromEntries(Object.entries(args).filter(([_, v]) => v != null)) as Record<string, string>;
        let params = new URLSearchParams(args_clean).toString()

        if (params != "") {
            path += `?${params}`
        }
    }

    return window.conduct.get(path)
}

export function post(path: string, body: any) {
    console.log("Sending POST: ")
    console.log(body)
    return window.conduct.post(path, JSON.stringify(body))
}

export async function getSummary(): Promise<SummaryResponse> {
    let result = await get("api/v1/command/summary")
    return await result.json() as SummaryResponse
}

export async function doExport(): Promise<SummaryResponse> {
    let result = await get("api/v1/command/export?asset=suzanneA&department=model")
    return await result.json() as SummaryResponse
}

export async function doCreate(): Promise<SummaryResponse> {
    let result = await get("api/v1/command/create?asset=suzanneA&department=model")
    return await result.json() as SummaryResponse
}

export async function exitDialog(result: any) {
    let response = await post("api/v1/dialog/exit", result)
}

export async function cancelDialog(): Promise<SummaryResponse> {
    let result = await get("api/v1/dialog/cancel")
    return await result.json() as SummaryResponse
}


export async function listAssets(department_filter: null | string = null): Promise<ListAssetsResult> {
    let result = await get("api/v1/command/list_assets", {
        "department": department_filter
    })
    return await result.json() as ListAssetsResult
}

export async function create_setup(department: string, asset: string, shot: null | string = null, dry_run: boolean = false): Promise<SetupResult | ErrorResponse> {
    let result = await get("api/v1/command/setup", {
        "department": department,
        "asset": asset,
        "shot": shot,
        "dry": dry_run,
    })
    return await result.json() as SetupResult | ErrorResponse
}


export async function listShots(): Promise<ListShotsResult> {
    let result = await get("api/v1/command/list_shots")
    return await result.json() as ListShotsResult
}
