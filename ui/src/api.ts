import { AssetTreeCategory, IngestResult, ListAssetsResult, ListElementsResult, ListExportFormatsResult, ListShotsResult, ResolveElementsResult, SetupResult, SummaryResponse } from "./bindings/bindings_gen";


declare global {
    interface Window {
        conduct: {
            get: (path: string) => Promise<Response>,
            post: (path: string, body: string) => Promise<Response>
            api: any
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

export async function doExport(department: string, asset: string, element: string, shot: string | null | undefined, from: string, file_format: string): Promise<SummaryResponse> {
    let result = await get("api/v1/command/export", {
        "department": department,
        "asset": asset,
        "element": element,
        "shot": shot,
        "from": from,
        "file_format": file_format
    })
    return await result.json() as SummaryResponse
}

export async function doCreate(asset: string | null, category: string | null): Promise<any> {
    let result = await get("api/v1/command/create", {
        "asset": asset,
        "category": category
    })

    if (result.status == 200) {
        return true
    } else {
        return await result.json()
    }
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

export async function listElements(asset: string, department: null | string = null, load: boolean = false): Promise<ListElementsResult> {
    let result = await get("api/v1/command/list_elements", {
        "department": department,
        "asset": asset,
        "load": load,
    })

    let data = await result.json()
    console.log(data)
    return data as ListElementsResult
}

export async function listExportFormats(department: string, program: string,): Promise<ListExportFormatsResult> {
    let result = await get("api/v1/command/list_export_formats", {
        "department": department,
        "from": program
    })

    let data = await result.json()
    console.log(data)
    return data as ListExportFormatsResult
}

export async function getAssetTree(department_filter: null | string = null): Promise<AssetTreeCategory> {
    let result = await get("api/v1/command/get_asset_tree", {
        "department": department_filter
    })
    let json = await result.json()
    return json as AssetTreeCategory
}

export async function saveChanges(): Promise<any> {
    let result = await get("api/v1/command/save");
    let json = await result.json()
    return json
}

export async function loadAssets(program: string, department: string, shot: null | string = null, assets: string[]): Promise<AssetTreeCategory> {
    let result = await get("api/v1/command/load_assets", {
        "program": program,
        "department": department,
        "shot": shot,
        "assets_list": assets.join(',')
    })
    let json = await result.json()
    return json as AssetTreeCategory
}


export async function create_setup(department: string, asset: string, file_format: string, shot: null | string = null, dry_run: boolean = false): Promise<SetupResult | ErrorResponse> {
    let result = await get("api/v1/command/setup", {
        "department": department,
        "asset": asset,
        "shot": shot,
        "dry": dry_run,
        "file_format": file_format
    })
    return await result.json() as SetupResult | ErrorResponse
}


export async function listShots(): Promise<ListShotsResult> {
    let result = await get("api/v1/command/list_shots")
    return await result.json() as ListShotsResult
}

export async function resolveElements(asset: string): Promise<ResolveElementsResult> {
    let result = await get("api/v1/command/resolve_elements", {
        "asset": asset
    })
    return await result.json() as ResolveElementsResult
}

export async function doIngest(asset: string, element: string | null, department: string, shot: string | null, file: string, target_format: string | null, license: string, source: string): Promise<IngestResult> {
    let result = await get("api/v1/command/ingest", {
        "asset": asset,
        "element": element,
        "department": department,
        "file": file,
        "target_format": target_format,
        "shot": shot,
        "license": license,
        "source": source
    })
    return await result.json() as IngestResult
}

window.conduct.api = {
    doExport,
    doIngest,
    listShots
}