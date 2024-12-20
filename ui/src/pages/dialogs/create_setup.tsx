import { createResource, type Component, Show, Switch, Match, createSignal } from 'solid-js';

import { get, getSummary, doExport, doCreate, exitDialog, listAssets, cancelDialog, create_setup as createSetup, listShots, ErrorResponse, isError } from '../../api';

import { Button } from '../../components/ui/button';
import { ColorModeProvider } from '@kobalte/core/color-mode';
import { Separator } from '../../components/ui/separator';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '~/components/ui/select';
import { Combobox, ComboboxContent, ComboboxControl, ComboboxInput, ComboboxItem, ComboboxItemIndicator, ComboboxItemLabel, ComboboxSection, ComboboxTrigger } from '~/components/ui/combobox';
import { Callout, CalloutContent, CalloutTitle } from '~/components/ui/callout';
import { SetupResult } from '~/bindings/bindings_gen';
import { useSearchParams } from '@solidjs/router';

const DialogCreateSetup: Component = () => {
    const [selectedDepartment, setSelectedDepartment] = createSignal<string | null>()
    const [selectedAsset, setSelectedAsset] = createSignal<string | null>()
    const [selectedShot, setSelectedShot] = createSignal<string | null>()

    const [info] = createResource(getSummary);
    const [assets] = createResource(selectedDepartment, listAssets)
    const [shots] = createResource(listShots);

    const [searchParams, setSearchParams] = useSearchParams();
    const fileFormat = () => searchParams.file_format;

    const derivedState = () => {
        return { department: selectedDepartment(), asset: selectedAsset(), shot: selectedShot() }
    }


    const [result] = createResource(derivedState, dryRun);

    async function dryRun() {
        console.log("Doing dry run!");
        let department = selectedDepartment();
        let asset = selectedAsset();

        if (department == null || asset == null) {
            return
        }

        return createSetup(department!, asset!, fileFormat() as string, selectedShot(), true);
    }

    async function done() {
        console.log("done!")

        let department = selectedDepartment();
        let asset = selectedAsset();

        if (department == null || asset == null) {
            return
        }

        let result = await createSetup(department!, asset!, fileFormat() as string, selectedShot(), false);

        await exitDialog(result);
    }


    return (
        <div class='h-lvh border-spacing-10 p-3'>
            <div class='h-full flex flex-col gap-2'>
                <div class='h-100 flex-1 gap-2'>
                    Create Setup

                    <div class='flex justify-center'>
                        <div class='w-full'>
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
                        </div>

                        <div class='w-full'>
                            <Select disabled={selectedDepartment() == null}
                                value={selectedAsset()}
                                onChange={setSelectedAsset}
                                options={assets()?.assets ?? []}
                                placeholder="Select asset"
                                class='p-1'
                                itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                                <SelectTrigger aria-label="Asset">
                                    <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                                </SelectTrigger>
                                <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                            </Select>
                        </div>
                    </div>

                    <Show when={shots()}>
                        <Combobox class='p-1' options={shots()!.shots}
                            itemComponent={(props) => (
                                <ComboboxItem item={props.item}>
                                    <ComboboxItemLabel>{props.item.textValue}</ComboboxItemLabel>
                                    <ComboboxItemIndicator />
                                </ComboboxItem>
                            )}
                            sectionComponent={(props) => (
                                <ComboboxSection>{props.section.rawValue}</ComboboxSection>
                            )}
                            placeholder="Select a shot (optional)"

                            onChange={setSelectedShot}>

                            <ComboboxControl aria-label="Food">
                                <ComboboxInput />
                                <ComboboxTrigger />
                            </ComboboxControl>
                            <ComboboxContent class=' overflow-y-auto max-h-[50vh]' />
                        </Combobox>
                    </Show>

                </div>

                <div class="flex flex-col">
                    <Show when={isError(result())} >
                        <Callout variant="warning">
                            <CalloutTitle>Warning</CalloutTitle>
                            <CalloutContent>
                                {(result() as ErrorResponse).error}
                            </CalloutContent>
                        </Callout>
                    </Show>

                    <Show when={result() != null && (isError(result()) == false)} >
                        <Callout variant={null}>
                            <CalloutTitle>{(result() as SetupResult).file_name}</CalloutTitle>
                            <CalloutContent>
                                <div class=' text-xs break-words ' >
                                    {(result() as SetupResult).path}
                                </div>

                            </CalloutContent>
                        </Callout>
                    </Show>
                </div>

                <div class='flex space-x-3 justify-center'>
                    <Button class='w-full' on:click={done} disabled={selectedDepartment() == null || selectedAsset() == null} >Done</Button>
                    <Button variant="outline" class='w-full' on:click={() => cancelDialog()}>Cancel</Button>
                </div>
            </div>
        </div>
    );
};


export default DialogCreateSetup;
