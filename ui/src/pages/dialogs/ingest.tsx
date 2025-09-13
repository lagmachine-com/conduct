
import { Combobox } from '@kobalte/core/*';
import { useSearchParams } from '@solidjs/router';
import { createEffect, createResource, createSignal, For, Show, type Component } from 'solid-js';
import { build } from 'vite';
import { doIngest, exitDialog, getSummary, listElements, listExportFormats, listShots } from '~/api';
import { Button } from '~/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '~/components/ui/card';
import { Label } from '~/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '~/components/ui/select';
import { TextField, TextFieldInput } from '~/components/ui/text-field';

interface file {
    path: string;
    mime: string;
}

const DialogIngest: Component = () => {
    const [files, setFiles] = createSignal<file[]>([])
    const [blobs, setBlobs] = createSignal<Map<string, Blob>>(new Map());

    const [selectedDepartment, setSelectedDepartment] = createSignal<string | null>()
    const [selectedShot, setSelectedShot] = createSignal<string | null>()

    const [info] = createResource(getSummary);
    const [shots] = createResource(listShots);

    const [license, setLicenseFiles] = createSignal<string[]>([])

    const [step, setStep] = createSignal<string>("select_files")

    const [searchParams, setSearchParams] = useSearchParams();
    const targetAsset = () => searchParams.asset;

    const [elements] = createResource(selectedDepartment, (department) => listElements(targetAsset() as string, department));
    const [elementSelections, setElementSelections] = createSignal<any>({});

    const [formats] = createResource(selectedDepartment, (department) => listExportFormats(department, "ingest"));
    const [formatSelections, setFormatSelections] = createSignal<any>({});

    const [shotSelections, setShotSelections] = createSignal<any>({});

    const [source, setSource] = createSignal("");

    const [isIngesting, setIsIngesting] = createSignal(false);
    const [ingestFinished, setIsIngestFinished] = createSignal(false);
    const [ingestStatus, setIngestStatus] = createSignal("");

    createEffect(async () => {
        console.log("Selected files:", files())
        let f = files();

        let map = new Map();

        await Promise.all(f.map(async (file) => {
            let req = await window.os.file(file.path);
            let data = await req.arrayBuffer();
            console.log(data);
            let blob = new Blob([data]);
            map.set(file.path, blob);

        }));

        setBlobs(map);
    })

    addEventListener("drop", function () {
        console.log("File entered");
    });

    addEventListener("message", (event) => {
        console.log("Received message!")
        console.log(event)

        if (event.data.type == "drag_drop_dropped") {
            console.log("Files dropped");

            if (step() == "select_files") {
                setFiles(event.data.data)

                setStep("select_license")
            } else if (step() == "select_license") {
                setLicenseFiles([event.data.data[0].path])
            }

            if (license().length > 0) {
                setStep("add_source")
            }

        }

        if (event.data.type == "drag_drop_enter") {
            console.log("Drag drop started");
        }

        if (event.data.type == "drag_drop_leave") {
            console.log("Drag drop ended");
        }
    });

    function onSourceChanged(value: string) {
        setSource(value)
        setStep("finish")
    }


    const delay = (ms: any) => new Promise(res => setTimeout(res, ms));

    async function runIngest() {
        setIsIngesting(true)

        for (var file of files()) {
            try {
                let element = elementSelections()[file.path]
                let asset = targetAsset() as string
                setIngestStatus(file + " -> " + asset + " " + "(" + element + ")")
                let format = formatSelections()[file.path]
                let dept = selectedDepartment()
                let shot = shotSelections()[file.path]
                let license_file = license()[0];
                let result = await doIngest(targetAsset() as string, element, dept!, shot, file.path, format, license_file, source())


                if (result['script'] != null) {
                    setIngestStatus("Running user script")
                    let data = {
                        ...result,
                        "element": element,
                        "asset": asset,
                        "department": dept,
                        "shot": shot,
                        "format": format
                    }

                    let obj = eval(result['script'])

                    console.log(obj)
                    await obj(data)
                }

                console.log(result)
                setIngestStatus("Ingested: " + JSON.stringify(result))

            } catch (error) {
                setIngestStatus("Error: " + JSON.stringify(error))
                await delay(5000)
            }
        }

        setIsIngestFinished(true)
        setIngestStatus("Ingest finished")
    }

    function onFinished() {
        console.log("test")
        console.log(history.length)
        if (history.length > 1) {
            history.back()
        } else {
            exitDialog(null)
        }
    }

    function canPreviewFileType(mime: String) {
        if (mime.startsWith("audio/")) {
            return true;
        }
    }

    function buildPreview(path: string, mime: string) {
        var safe_path = encodeURIComponent(path);
        let blob = blobs().get(path);

        if (blob == null) {
            return (
                <div></div>
            )
        }

        let url = URL.createObjectURL(blob);

        if (mime.startsWith("audio")) {

            return (
                <div class='w-full' >
                    <audio controls class='w-full' src={url}>
                    </audio>
                </div>
            )
        }
        return (
            <div>
                TESTTT
            </div>
        )
    }

    return (

        <div class='m-2'>
            <Show when={isIngesting()}>
                <div class='w-screen h-screen absolute z-10' >
                    <div class='opacity-100 w-screen h-screen absolute z-20 content-center text-center'>
                        <Card>
                            <CardHeader>
                                <CardTitle>Ingesting</CardTitle>
                                <CardDescription>{ingestStatus()}</CardDescription>
                            </CardHeader>
                            <Show when={ingestFinished()}>
                                <CardContent>
                                    <Button onClick={onFinished}>Done!</Button>
                                </CardContent>
                            </Show>
                        </Card>
                    </div>
                    <div class='bg-white opacity-80 absolute w-screen h-screen overflow-clip'>
                    </div>

                </div>
            </Show>
            <Card>
                <CardHeader>
                    <CardTitle>Ingest Files to {targetAsset()}</CardTitle>
                    <Show when={step() == "select_files"}>

                        <CardDescription>Drag and drop files to ingest</CardDescription>
                    </Show>
                </CardHeader>
                <CardContent>
                    <Show when={info()}>
                        <Select class='p-1'
                            value={selectedDepartment()}
                            onChange={setSelectedDepartment}
                            options={info()!.departments}
                            placeholder="Select department"
                            itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                            <SelectTrigger aria-label="Asset">
                                <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                            </SelectTrigger>
                            <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                        </Select>
                    </Show>
                    <Show when={elements() != undefined && formats() != undefined && files().length > 0 && shots()}>
                        <Card>
                            <CardHeader>
                                <CardTitle>Selected Files</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <For each={files()}>
                                    {
                                        (e) => {
                                            return <div class=' mt-4 mb-4 text-xs text-muted-foreground'>
                                                <div class='w-full flex'>
                                                    <div class='w-full'>
                                                        <TextField disabled class='w-full text-xs ' defaultValue={e.path}>
                                                            <TextFieldInput class='text-xs' />
                                                        </TextField></div>


                                                    <div class='flex items-center w-full'>

                                                        <Select class='w-full pl-1' id={e.path} value={elementSelections()[e.path]} options={elements()!.elements} onChange={(selected) => {
                                                            console.log(selected)
                                                            let selections = {
                                                                ...elementSelections()
                                                            }

                                                            selections[e.path] = selected
                                                            setElementSelections(selections)

                                                            console.log(elementSelections())
                                                        }}
                                                            placeholder="Element"
                                                            itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                                                            <SelectTrigger aria-label="Element">
                                                                <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                                                            </SelectTrigger>
                                                            <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                                                        </Select>


                                                        <Select class='justify-end w-full pl-1'
                                                            id={"file_format_" + e}
                                                            options={formats()!.formats}
                                                            placeholder="Format"
                                                            onChange={(selected) => {

                                                                console.log(selected)
                                                                let selections = {
                                                                    ...formatSelections()
                                                                }

                                                                selections[e.path] = selected
                                                                setFormatSelections(selections)

                                                                console.log(formatSelections())
                                                            }}
                                                            itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                                                            <SelectTrigger aria-label="Asset">
                                                                <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                                                            </SelectTrigger>
                                                            <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                                                        </Select>


                                                        <Select class='justify-end w-full pl-1'
                                                            id={"shot_" + e}
                                                            options={shots()!.shots}
                                                            placeholder="Shot"
                                                            onChange={(selected) => {

                                                                console.log(selected)
                                                                let selections = {
                                                                    ...shotSelections()
                                                                }

                                                                selections[e.path] = selected
                                                                setShotSelections(selections)

                                                                console.log(shotSelections())
                                                            }}
                                                            itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                                                            <SelectTrigger aria-label="Shot">
                                                                <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                                                            </SelectTrigger>
                                                            <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                                                        </Select>
                                                    </div>
                                                </div>
                                                <div>

                                                    <Show when={canPreviewFileType(e.mime)}>
                                                        {
                                                            buildPreview(e.path, e.mime)
                                                        }
                                                    </Show>

                                                </div>
                                            </div>
                                        }
                                    }
                                </For>
                                <Show when={files().length > 0}>
                                    <div class='flex justify-end py-1'>
                                        <Button onClick={() => {
                                            setFiles([]);
                                            setStep("select_files");
                                        }} variant={"secondary"} class=''>Clear</Button>
                                    </div>
                                </Show>
                            </CardContent>
                        </Card>
                        <div class='h-2'></div>
                        <Show when={step() != "select_files"}>

                            <Card>
                                <CardHeader>
                                    <CardTitle>License</CardTitle>
                                    <Show when={step() == "select_license"}>
                                        <CardDescription>Now, drag and drop the license associated with these files</CardDescription>
                                    </Show>
                                </CardHeader>
                                <CardContent>
                                    <For each={license()}>
                                        {
                                            (e) => {
                                                return <div class=' border- text-xs text-muted-foreground'>{e}</div>
                                            }
                                        }
                                    </For>
                                </CardContent>
                            </Card>

                        </Show>
                        <div class='h-2'></div>
                        <Show when={!["select_files", "select_license"].includes(step())}>

                            <Card>
                                <CardHeader>
                                    <CardTitle>Source</CardTitle>
                                    <Show when={step() == "add_source"}>
                                        <CardDescription>Please enter the URL where these files were sourced from (Download / Store page)</CardDescription>
                                    </Show>
                                </CardHeader>
                                <CardContent>
                                    <TextField onChange={onSourceChanged} class='w-full text-xs'>
                                        <TextFieldInput class='text-xs' />
                                    </TextField>
                                </CardContent>
                            </Card>

                        </Show>
                        <div class='h-2'></div>
                        <Show when={step() == "finish"}>
                            <Card>
                                <CardHeader>
                                    <CardTitle>Import</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <Button onClick={runIngest}> Import Files</Button>
                                </CardContent>
                            </Card>
                        </Show>
                    </Show>
                </CardContent>
            </Card>
        </div>
    );
};


export default DialogIngest;
