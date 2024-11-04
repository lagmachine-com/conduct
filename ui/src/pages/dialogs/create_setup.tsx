import { createResource, type Component, Show, Switch, Match, createSignal } from 'solid-js';

import { get, getSummary, doExport, doCreate, exitDialog, listAssets, cancelDialog } from '../../api';

import { Button } from '../../components/ui/button';
import { ColorModeProvider } from '@kobalte/core/color-mode';
import { Separator } from '../../components/ui/separator';
import { SummaryResponse } from '../../bindings/summary_response';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '~/components/ui/select';

const DialogCreateSetup: Component = () => {
    const [selectedDepartment, setSelectedDepartment] = createSignal<string | null>()
    const [selectedAsset, setSelectedAsset] = createSignal("")

    const [info] = createResource(getSummary);
    const [assets] = createResource(selectedDepartment, listAssets)


    return (
        <ColorModeProvider initialColorMode="system" >
            <div class='h-lvh border-spacing-10 p-3'>
                <div class='h-full flex flex-col gap-2'>
                    <div class='h-100 flex-1 gap-2'>
                        Create Setup

                        <div>
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


                        <Show when={selectedDepartment() && assets()}>
                            <div>
                                <Select
                                    value={selectedAsset()}
                                    onChange={setSelectedAsset}
                                    options={assets()!.assets}
                                    placeholder="Select asset"
                                    class='p-1'
                                    itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                                    <SelectTrigger aria-label="Asset">
                                        <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                                    </SelectTrigger>
                                    <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                                </Select>
                            </div>
                        </Show>

                    </div>
                    <div class='flex space-x-3 justify-center'>
                        <Button class='w-full' on: click={() => exitDialog()}>Done</Button>
                        <Button variant="outline" class='w-full' on: click={() => cancelDialog()}>Cancel</Button>
                    </div>
                </div>
            </div>
        </ColorModeProvider >
    );
};


export default DialogCreateSetup;
