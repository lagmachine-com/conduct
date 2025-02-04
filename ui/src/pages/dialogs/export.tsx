import { createResource, type Component, Show, Switch, Match, createSignal, For } from 'solid-js';

import { cancelDialog, doExport, exitDialog, listElements, listExportFormats } from '../../api';

import { Button } from '../../components/ui/button';
import { ColorModeProvider } from '@kobalte/core/color-mode';
import { Separator } from '../../components/ui/separator';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '~/components/ui/select';
import { Combobox, ComboboxContent, ComboboxControl, ComboboxInput, ComboboxItem, ComboboxItemIndicator, ComboboxItemLabel, ComboboxSection, ComboboxTrigger } from '~/components/ui/combobox';
import { Callout, CalloutContent, CalloutTitle } from '~/components/ui/callout';
import { SetupResult } from '~/bindings/bindings_gen';
import { useSearchParams } from '@solidjs/router';
import { Label } from '~/components/ui/label';
import { Checkbox } from '~/components/ui/checkbox';
import { ToggleGroup } from '~/components/ui/toggle-group';
import { createStore } from 'solid-js/store';

type ElementState = {
    enabled: boolean,
    fileFormat: string | null,
    items: string[]
}

const DialogExport: Component = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    const department = () => searchParams.department;
    const shot = () => searchParams.shot;
    const program = () => searchParams.program;
    const asset = () => searchParams.asset;
    const items = () => (searchParams.items as string).split(',')

    var defaultState = new Map()
    let prev = searchParams['prev-state']
    if (prev != undefined) {
        console.log(searchParams['prev-state'])
        let stateObject = JSON.parse(searchParams['prev-state'] as string)
        console.log(stateObject)

        defaultState = new Map(Object.entries(stateObject));
    }

    const [data, setData] = createSignal<Map<string, ElementState>>(defaultState)

    const [elements] = createResource(() => listElements(asset() as string, department() as any));
    const [formats] = createResource(() => listExportFormats(department() as string, program() as string));

    const contextLabel = () => {
        return [shot(), department(), asset()].filter((x) => !!x).join(' / ')
    }

    async function done() {
        let export_dept = department() as string
        let export_shot = shot()
        let export_asset = asset() as string
        let export_program = program() as string

        let export_results = []

        for (let [key, value] of data()) {
            if (!value.enabled) {
                continue;
            }
            console.log(value)
            let export_result = await doExport(
                export_dept,
                export_asset,
                key,
                export_shot != undefined ? export_shot as string : null,
                export_program,
                value.fileFormat!
            )

            export_results.push({
                items: value.items,
                result: export_result
            })
            console.log(export_result)

        }

        const obj = Object.fromEntries(data());
        const json = JSON.stringify(obj);
        const save_state = JSON.stringify(json)


        let response = {
            exports: export_results,
            save_state: save_state
        };

        await exitDialog(response);
    }

    var elementEntry: Component<{ element: string, }> = (props) => {
        let s = data()
        let state = s.get(props.element)

        let item: ElementState = state ?? {
            fileFormat: null,
            enabled: false,
            items: []
        }

        return (
            <>
                <Callout variant={null} class='mt-2'>
                    <div class='flex-row flex p-1 content-around items-center'>
                        <Checkbox defaultChecked={item.enabled} onChange={(selected) => {
                            let elementState = {
                                ...item,
                                enabled: selected
                            }

                            let result = new Map(s)
                            result.set(props.element, elementState);

                            setData(
                                result
                            )

                        }} id={props.element} />
                        <Label class='ml-2' for={`${props.element}-input`}>{props.element}</Label>
                    </div>



                    <Select class='p-1 justify-end'
                        disabled={item.enabled != true}
                        id={props.element}
                        value={item.fileFormat}
                        options={formats()!.formats}
                        placeholder="File Format"
                        onChange={(selected) => {
                            let elementState = {
                                ...item!,
                                fileFormat: selected
                            }

                            let result = new Map(s)
                            result.set(props.element, elementState);

                            setData(
                                result
                            )

                        }}
                        itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                        <SelectTrigger aria-label="Asset" class={(item.fileFormat == null && item.enabled) ? 'bg-error-foreground' : ''}>
                            <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                        </SelectTrigger>
                        <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                    </Select>

                    <div class='flex-row flex-wrap flex'>

                        {
                            item.items.map((val, index) => {
                                return <>
                                    <Select class='p-1 justify-end'
                                        value={val}
                                        disabled={state?.enabled != true}
                                        id={props.element}
                                        options={['none', ...items().filter((e) => e == val || !item.items.includes(e))]}
                                        placeholder="add item...."
                                        onChange={(selected) => {
                                            let elementState = {
                                                ...item!,
                                            }

                                            if (selected == 'none') {
                                                elementState.items.splice(index, 1)
                                            } else {
                                                elementState.items[index] = selected!
                                            }
                                            let result = new Map(s)
                                            result.set(props.element, elementState);

                                            setData(
                                                result
                                            )

                                        }}
                                        itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                                        <SelectTrigger aria-label="Asset">
                                            <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                                        </SelectTrigger>
                                        <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                                    </Select>
                                </>
                            })
                        }

                        <Select class='p-1 justify-end'
                            disabled={state?.enabled != true}
                            id={props.element}
                            options={items().filter((e) => !item.items.includes(e))}
                            placeholder="add item...."
                            onChange={(selected) => {
                                let elementState = {
                                    ...item!,
                                }

                                if (!elementState.items.includes(selected!)) {
                                    elementState.items.push(selected!)
                                }

                                let result = new Map(s)
                                result.set(props.element, elementState);

                                setData(
                                    result
                                )

                            }}
                            itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                            <SelectTrigger aria-label="Asset" class={(item.items.length == 0 && item.enabled) ? 'bg-error-foreground' : ''}>
                                <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                            </SelectTrigger>
                            <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                        </Select>
                    </div>

                </Callout>
            </>
        )
    }



    return (
        <div class='h-lvh p-3 '>

            <div class='h-full flex flex-col '>
                <div>
                    Export
                </div>
                <Label class='text-xs'>{program()}: {contextLabel()}</Label>

                <div class='flex-1 text-left p-1 overflow-x-clip overflow-y-scroll w-full'>
                    <Show when={elements() != null && formats() != null}>
                        <ToggleGroup multiple class='items-start flex flex-col justify-start p-3'>
                            {
                                Object.entries(elements()!.elements).map(item => {
                                    return elementEntry({ element: item[1] })
                                })
                            }
                        </ToggleGroup>
                    </Show>

                </div>

                <div class='text-xs break-all'>

                </div>

                <div class=' flex-row flex mt-2'>
                    <Button class="w-full mx-1 " onclick={done}>
                        Export
                    </Button>
                    <Button class="w-full mx-1 " variant="secondary" on:click={() => cancelDialog()}>
                        Cancel
                    </Button>
                </div>
            </div>

        </div >
    );
};


export default DialogExport;
