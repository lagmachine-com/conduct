
import { Combobox } from '@kobalte/core/*';
import { useSearchParams } from '@solidjs/router';
import { createResource, createSignal, For, Show, type Component } from 'solid-js';
import { doIngest, exitDialog, getSummary, listElements, listExportFormats } from '~/api';
import { Button } from '~/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '~/components/ui/card';
import { Label } from '~/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '~/components/ui/select';
import { TextField, TextFieldInput } from '~/components/ui/text-field';

const DialogIngest: Component = () => {
    const [files, setFiles] = createSignal<string[]>([])
    const [selectedDepartment, setSelectedDepartment] = createSignal<string | null>()

    const [info] = createResource(getSummary);

    const [license, setLicenseFiles] = createSignal<string[]>([])

    const [step, setStep] = createSignal<string>("select_files")

    const [searchParams, setSearchParams] = useSearchParams();
    const targetAsset = () => searchParams.asset;

    const [elements] = createResource(() => listElements(targetAsset() as string, selectedDepartment()));
    const [elementSelections, setElementSelections] = createSignal<any>({});

    const [formats] = createResource(selectedDepartment, (department) => listExportFormats(department, "ingest"));
    const [formatSelections, setFormatSelections] = createSignal<any>({});

    const [source, setSource] = createSignal("");

    const [isIngesting, setIsIngesting] = createSignal(false);
    const [ingestFinished, setIsIngestFinished] = createSignal(false);
    const [ingestStatus, setIngestStatus] = createSignal("");


    addEventListener("message", (event) => {
        console.log("Received message!")
        console.log(event)

        if (event.data.type == "files_dropped") {

            if (step() == "select_files") {
                setFiles(event.data.data)
                setStep("select_license")
            } else if (step() == "select_license") {
                setLicenseFiles([event.data.data[0]])
            }

            if (license().length > 0) {
                setStep("add_source")
            }
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
                let element = elementSelections()[file]
                let asset = targetAsset() as string
                setIngestStatus(file + " -> " + asset + " " + "(" + element + ")")
                let format = formatSelections()[file]
                let dept = selectedDepartment()
                let license_file = license()[0];
                let result = await doIngest(targetAsset() as string, element, dept!, file, format, license_file, source())


                if (result['script'] != null) {
                    setIngestStatus("Running user script")
                    let data = {
                        ...result,
                        "element": element,
                        "asset": asset,
                        "department": dept,
                        "format": format
                    }

                    let obj = eval(result['script'])

                    console.log(obj)
                    obj(data)
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
                    <Show when={elements() != undefined && formats() != undefined && files().length > 0}>
                        <Card>
                            <CardHeader>
                                <CardTitle>Selected Files</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <For each={files()}>
                                    {
                                        (e) => {
                                            return <div class=' border- text-xs text-muted-foreground flex items-center'>
                                                <div class='w-9/12'>
                                                    <TextField disabled class='w-full text-xs ' defaultValue={e}>
                                                        <TextFieldInput class='text-xs' />
                                                    </TextField></div>
                                                <div class='flex items-center w-3/12'>

                                                    <Select class='w-full p-1' id={e} value={elementSelections()[e]} options={elements()!.elements} onChange={(selected) => {
                                                        console.log(selected)
                                                        let selections = {
                                                            ...elementSelections()
                                                        }

                                                        selections[e] = selected
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


                                                    <Select class='justify-end w-full'
                                                        id={"file_format_" + e}
                                                        options={formats()!.formats}
                                                        placeholder="Format"
                                                        onChange={(selected) => {

                                                            console.log(selected)
                                                            let selections = {
                                                                ...formatSelections()
                                                            }

                                                            selections[e] = selected
                                                            setFormatSelections(selections)

                                                            console.log(formatSelections())
                                                        }}
                                                        itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                                                        <SelectTrigger aria-label="Asset">
                                                            <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                                                        </SelectTrigger>
                                                        <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                                                    </Select>
                                                </div>
                                            </div>
                                        }
                                    }
                                </For>
                                <Show when={files().length > 0}>
                                    <div class='flex justify-end'>
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
                                        <CardDescription>Please enter where these files were sourced from</CardDescription>
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
