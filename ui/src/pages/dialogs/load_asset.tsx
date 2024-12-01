import { createResource, type Component, Show, Switch, Match, createSignal, For } from 'solid-js';

import { get, getSummary, doExport, doCreate, exitDialog, listAssets, cancelDialog, create_setup as createSetup, listShots, ErrorResponse, isError, getAssetTree } from '../../api';

import { Button } from '../../components/ui/button';
import { ColorModeProvider } from '@kobalte/core/color-mode';
import { Separator } from '../../components/ui/separator';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '~/components/ui/select';
import { Combobox, ComboboxContent, ComboboxControl, ComboboxInput, ComboboxItem, ComboboxItemIndicator, ComboboxItemLabel, ComboboxSection, ComboboxTrigger } from '~/components/ui/combobox';
import { Callout, CalloutContent, CalloutTitle } from '~/components/ui/callout';
import { AssetTreeEntry, SetupResult } from '~/bindings/bindings_gen';
import { Checkbox } from '~/components/ui/checkbox';
import { Label } from '~/components/ui/label';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '~/components/ui/card';
import { ToggleGroup } from '@kobalte/core/toggle-group';
import { ToggleGroupItem } from '~/components/ui/toggle-group';
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '~/components/ui/accordion';

const DialogLoadAsset: Component = () => {
    const [selectedAssets, setSelectedAssets] = createSignal<string[]>([])

    const [info] = createResource(getSummary);
    const [assets] = createResource(() => getAssetTree(null));

    let closedPaths: string[] = [];

    const loadedAssetsList = () => selectedAssets().join(",");

    const assetEntry: Component<{ entry_name: string, entry: AssetTreeEntry, current_path: string }> = (props) => {
        let path = props.current_path + "/" + props.entry_name;
        if (props.entry.type == "Asset") {
            return (
                <div class='flex-row flex p-1'>
                    <Checkbox defaultChecked={selectedAssets().indexOf(props.entry_name) != -1} onChange={(selected) => {
                        let selection = Array.from(selectedAssets());
                        if (selected) {
                            selection.push(props.entry_name)
                        } else {
                            let idx = selection.indexOf(props.entry_name)
                            if (idx != -1) {
                                selection.splice(idx, 1);
                            }
                        }

                        console.log(selection)

                        setSelectedAssets(selection)
                    }} id={path} />
                    <Label class='ml-2' for={`${path}-input`}>{props.entry_name}</Label>

                </div>
            )
        }

        return (
            <Accordion class='py-0' defaultValue={closedPaths.indexOf(path) == -1 ? [path] : []} multiple collapsible onChange={(selection) => {
                if (selection.includes(path)) {
                    let idx = closedPaths.indexOf(path)
                    if (idx != -1) {
                        closedPaths.splice(idx, 1);
                    }
                } else {
                    closedPaths.push(path)
                }
            }} >
                <AccordionItem value={path}>
                    <AccordionTrigger>{props.entry_name}</AccordionTrigger>
                    <AccordionContent class='pl-2'>
                        <For each={Object.entries(props.entry.children)}>
                            {(item) =>
                                assetEntry({ entry_name: item[0], entry: item[1]!, current_path: path })
                            }
                        </For>
                    </AccordionContent>
                </AccordionItem>
            </Accordion>
        )
    }



    return (
        <div class='h-lvh p-3 '>

            <div class='h-full flex flex-col '>


                <div class='flex-1 text-left p-1 overflow-x-clip overflow-y-scroll w-full '>
                    <Show when={assets() != null}>
                        <ToggleGroup multiple>
                            <For each={Object.entries(assets()!.children)}>
                                {(item) => item[1]!.type == 'Asset' ? (
                                    <div>
                                        {item[0]}
                                    </div>
                                ) : assetEntry({
                                    entry_name: item[0]!,
                                    entry: item[1]!,
                                    current_path: ""
                                })}
                            </For>
                        </ToggleGroup>
                    </Show>
                </div>

                <div class='text-xs break-all'>
                    {loadedAssetsList()}
                </div>

                <div class=' flex-row flex mt-2'>
                    <Button disabled={selectedAssets().length == 0} class="w-full mx-1"> {
                        selectedAssets().length > 0 ? `Import ${selectedAssets().length} Asset(s)` : "Import Assets"
                    }
                    </Button>
                    <Button class="w-full mx-1 " variant="secondary" on:click={() => cancelDialog()}>
                        Cancel
                    </Button>
                </div>
            </div>
        </div >
    );
};


export default DialogLoadAsset;
