import { Component, createResource, createSignal, For, Resource, Show } from "solid-js";
import { getAssetTree } from "~/api";
import { ToggleGroup } from "../ui/toggle-group";
import { Label } from "../ui/label";
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "../ui/accordion";
import { AssetTreeCategory, AssetTreeEntry } from "~/bindings/bindings_gen";
import { Checkbox } from "../ui/checkbox";
import { ContextMenu } from "@kobalte/core/context-menu";
import { ContextMenuContent, ContextMenuGroupLabel, ContextMenuItem, ContextMenuSeparator, ContextMenuTrigger } from "../ui/context-menu";
import { Button } from "../ui/button";

import * as AccordionPrimitive from "@kobalte/core/accordion"
import { Tooltip } from "@kobalte/core/tooltip";
import { TooltipContent, TooltipTrigger } from "../ui/tooltip";

export interface AssetTreeProps {
    contextMenuBuilder?(name: string, entry: AssetTreeEntry): any
    categoryContextMenuBuilder?(name: string): any
    assets: Resource<AssetTreeCategory>
}

const AssetTree = (props: AssetTreeProps) => {

    let closedPaths: string[] = [];
    let contextMenuBuilder = props.contextMenuBuilder;


    const assetEntry: Component<{ entry_name: string, entry: AssetTreeEntry, current_path: string }> = (props) => {
        let path = props.current_path + (props.current_path == "" ? props.entry_name : ("/" + props.entry_name));
        if (props.entry.type == "Asset") {
            return (
                <div class='flex-row flex p-1'>
                    <ContextMenu>
                        <ContextMenuTrigger>
                            <Label class='ml-2 text-muted-foreground text-sm'>{props.entry_name}</Label>
                        </ContextMenuTrigger>
                        <ContextMenuContent>
                            <Label class="text-muted-foreground text-xs">{path}</Label>
                            <ContextMenuSeparator></ContextMenuSeparator>
                            {contextMenuBuilder?.(path, props.entry)}

                        </ContextMenuContent>
                    </ContextMenu>

                </div>
            )
        }

        return (

            <Accordion class='py-0 w-full' defaultValue={closedPaths.indexOf(path) == -1 ? [path] : []} multiple collapsible onChange={(selection) => {
                if (selection.includes(path)) {
                    let idx = closedPaths.indexOf(path)
                    if (idx != -1) {
                        closedPaths.splice(idx, 1);
                    }
                } else {
                    closedPaths.push(path)
                }
            }} >
                <AccordionItem id={props.entry_name} value={path}>
                    <div>
                        <AccordionPrimitive.Trigger class="h-0 absolute"></AccordionPrimitive.Trigger>
                        <ContextMenu>
                            <ContextMenuTrigger>
                                <AccordionTrigger >
                                    <Label for={path}>{props.entry_name}</Label>
                                </AccordionTrigger>
                            </ContextMenuTrigger>
                            <ContextMenuContent>
                                <Label class="text-muted-foreground text-xs">{path}</Label>
                                <ContextMenuSeparator></ContextMenuSeparator>
                                {contextMenuBuilder?.(path, props.entry)}
                            </ContextMenuContent>
                        </ContextMenu>
                    </div>
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
        <div class='flex-1 flex-col text-left overflow-x-clip overflow-y-scroll w-full content-none '>
            <Show when={props.assets() != null}>
                <ToggleGroup class="flex-col" multiple>
                    <For each={Object.entries(props.assets()!.children)}>
                        {(item) => item[1]!.type == 'Asset' ? (
                            <div>
                                {item[0]}
                            </div>
                        ) : <div class="border-b w-full"> {
                            assetEntry({
                                entry_name: item[0]!,
                                entry: item[1]!,
                                current_path: ""
                            })
                        }
                        </div>
                        }
                    </For>
                </ToggleGroup>
            </Show>
        </div>
    );

}

export default AssetTree