import { createResource, type Component, Show, Switch, Match, createSignal, For } from 'solid-js';

import { cancelDialog, listElements, listExportFormats } from '../../api';

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

const DialogExport: Component = () => {


    const [searchParams, setSearchParams] = useSearchParams();


    const department = () => searchParams.department;
    const shot = () => searchParams.shot;
    const program = () => searchParams.program;
    const asset = () => searchParams.asset;

    const [elements] = createResource(() => listElements(asset() as string, department() as any));
    const [formats] = createResource(() => listExportFormats(department() as string, program() as string));

    const contextLabel = () => {
        return [shot(), department(), asset()].filter((x) => !!x).join(' / ')
    }

    const elementEntry: Component<{ element: string }> = (props) => {

        return (
            <div class='flex-row flex p-1 content-around items-center'>

                <Select class='p-1 justify-end'
                    value={formats()!.formats[0]}
                    options={formats()!.formats}
                    placeholder="Select department"
                    itemComponent={(props) => <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>}>
                    <SelectTrigger aria-label="Asset">
                        <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
                    </SelectTrigger>
                    <SelectContent class=' overflow-y-auto max-h-[50vh]' />
                </Select>

                <Checkbox onChange={(selected) => {
                }} id={props.element} />
                <Label class='ml-2' for={`${props.element}-input`}>{props.element}</Label>
            </div>
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
                            <For each={Object.entries(elements()!.elements)}>

                                {(item) => elementEntry({ element: item[1] })}
                            </For>
                        </ToggleGroup>
                    </Show>

                </div>

                <div class='text-xs break-all'>

                </div>

                <div class=' flex-row flex mt-2'>
                    <Button class="w-full mx-1 ">
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
