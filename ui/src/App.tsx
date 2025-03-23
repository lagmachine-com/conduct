import { createResource, type Component, Show, Switch, Match, createSignal, For } from 'solid-js';

import { getSummary, getAssetTree, doCreate, get, saveChanges, listElements, resolveElements } from './api';

import { Button } from './components/ui/button';
import { ColorModeProvider } from '@kobalte/core/color-mode';
import { Separator } from './components/ui/separator';
import AssetTree from './components/organisms/asset_tree';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from './components/ui/dialog';
import { ContextMenuCheckboxItem, ContextMenuGroupLabel, ContextMenuItem } from './components/ui/context-menu';
import { TextField, TextFieldInput } from './components/ui/text-field';
import { Callout, CalloutContent, CalloutTitle } from './components/ui/callout';
import { Menubar, MenubarContent, MenubarItem, MenubarItemLabel, MenubarMenu, MenubarSeparator, MenubarShortcut, MenubarSub, MenubarSubContent, MenubarSubTrigger, MenubarTrigger } from './components/ui/menubar';
import { comma } from 'postcss/lib/list';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './components/ui/card';
import { Accordion } from '@kobalte/core/accordion';
import { AccordionContent, AccordionItem, AccordionTrigger } from './components/ui/accordion';
import { Label } from './components/ui/label';
import { Tooltip, TooltipContent, TooltipTrigger } from './components/ui/tooltip';

interface CommandEntry {
  command: string,
  content: any,
  label: string
}

const App: Component = () => {
  const [info] = createResource(getSummary);
  const [assets, { mutate, refetch }] = createResource(() => getAssetTree(null));

  const [isCreateAssetDialogOpen, setOpenCreateAssetDialog] = createSignal<boolean>(false);

  const [isCreateCategoryDialogOpen, setOpenCreateCategoryDialog] = createSignal<boolean>(false);
  const [category, setCategory] = createSignal<string>("");
  const [newAssetName, setNewAssetname] = createSignal<string>("")

  const [selectedPath, setSelectedPath] = createSignal<string>("")

  const [resolvedElements] = createResource(selectedPath, resolveElements)

  const [error, setError] = createSignal<string>("")

  const newAssetPath = () => {
    let path = category()
    if (path.length > 0) path += "/"
    path += newAssetName()
    return path
  }

  const openCreateAssetDialog = (parent: string) => {
    setOpenCreateAssetDialog(true)
    setNewAssetname("")
    setError("")
    setCategory(parent)
  }

  const openCreateCategoryDialog = (parent: string) => {
    setNewAssetname("")
    setError("")
    setCategory(parent)
    setOpenCreateCategoryDialog(true);
  }


  return (
    <ColorModeProvider initialColorMode="system" >
      <Show when={info()}>
        <div class='h-screen overflow-clip p-1'>

          <Menubar>
            <div class='mr-2'>
              <h4 class="text-sm font-medium leading-none">{info()!.display_name}</h4>
              <p class="text-xs text-muted-foreground">{info()!.identifier}</p>
            </div>
            <Separator orientation="vertical" />
            <MenubarMenu>
              <MenubarTrigger>File</MenubarTrigger>
              <MenubarContent>
                <MenubarItem onClick={() => {
                  saveChanges()
                }
                }>Save Changes</MenubarItem>
              </MenubarContent>
            </MenubarMenu>
            <MenubarMenu>
              <MenubarTrigger>Edit</MenubarTrigger>
              <MenubarContent>
                <MenubarItem onClick={() => {
                  openCreateCategoryDialog("")
                }}>Create Category</MenubarItem>
                <MenubarItem onClick={() => {
                  openCreateAssetDialog("")
                }}>Create Asset</MenubarItem>
                <MenubarSeparator />
                <MenubarSub overlap>
                  <MenubarSubTrigger>Undo</MenubarSubTrigger>
                  <MenubarSubContent>


                  </MenubarSubContent>
                </MenubarSub>
              </MenubarContent>
            </MenubarMenu>
          </Menubar>
          <div class='border-spacing-10 p-3 pb-12 flex-row flex '>
            <div class={selectedPath() != "" ? 'w-1/2' : 'w-full'}>
              <div class='h-screen overflow-y-scroll'>

                <AssetTree onPathClicked={(path) => setSelectedPath(path)} assets={assets} contextMenuBuilder={(path, entry) => (
                  <>
                    <ContextMenuItem onClick={() => console.log(path)}>Inspect</ContextMenuItem>
                    {
                      entry.type == "Category" && Object.entries(entry.children).every((e) => e[1]?.type != "Category") ? <ContextMenuItem onClick={() => { openCreateAssetDialog(path) }} >Add Asset</ContextMenuItem> : <></>
                    }
                    {
                      entry.type == "Category" && Object.entries(entry.children).every((e) => e[1]?.type == "Category") ? <ContextMenuItem onClick={() => { openCreateCategoryDialog(path) }} >Add Subcategory</ContextMenuItem> : <></>
                    }
                  </>
                )}></AssetTree>
              </div>
            </div>
            <Show when={selectedPath() != "" && resolvedElements() != undefined} >
              <Card class='w-1/2 h-fit overflow-scroll' >
                <CardHeader>
                  <CardTitle>
                    {selectedPath().split("/").slice(-1)[0]}
                  </CardTitle>
                  <CardDescription>
                    {selectedPath()}
                  </CardDescription>
                  <CardContent class='p-0 overflow-y-scroll'>


                    <Accordion multiple>
                      <For each={Object.entries(resolvedElements()?.result ?? {})}
                      >{(item) => {
                        let entry = resolvedElements()?.result[item[0]]!
                        let versions = entry.versions

                        return <AccordionItem value={selectedPath() + item[0]}>
                          <AccordionTrigger disabled={versions?.length == 0} class='flex-row-reverse justify-end'>
                            {
                              <div class={`${versions?.length == 0 ? 'text-muted-foreground' : 'text-success-foreground'} content-start items-center flex-row flex`}>
                                <Label >
                                  {item[0]}
                                </Label>
                                <div class='ml-1'>
                                  <Show when={entry.info.shot_local}>
                                    <Tooltip>
                                      <TooltipTrigger>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"><path fill="currentColor" d="M7 20h8v-8H7zM3.5 8q.95-2.725 3.288-4.362T12 2q2.05 0 3.888.875T19 5.35V2h2v6h-6V6h1.9q-.975-.975-2.25-1.487T12 4Q9.975 4 8.262 5.075T5.676 8zM7 20v-8zm0 2q-.825 0-1.412-.587T5 20v-8q0-.825.588-1.412T7 10h8q.825 0 1.413.588T17 12v3l3-3v8l-3-3v3q0 .825-.587 1.413T15 22z" /></svg>
                                      </TooltipTrigger>
                                      <TooltipContent>
                                        <Label>Shot Local</Label>
                                      </TooltipContent>
                                    </Tooltip>
                                  </Show>
                                </div>
                              </div>
                            }
                          </AccordionTrigger>
                          <AccordionContent>
                            <For each={versions}>
                              {(version) =>

                                <div class='ml-2'>
                                  <div class='flex-row flex justify-items-center'>

                                    <Label class='self-center mr-2'>{version.version}</Label>
                                    <TextField disabled class='w-full text-xs ' defaultValue={version.path}>
                                      <TextFieldInput class='text-xs' />
                                    </TextField>
                                  </div>
                                </div>

                              }
                            </For>

                          </AccordionContent>
                        </AccordionItem>;
                      }
                        }</For>
                    </Accordion>
                    <div class='p-1'>
                      <Separator></Separator>
                    </div>
                    <div class='flex justify-end'>
                      <Button onClick={() => {
                        let asset = selectedPath().split("/").slice(-1)[0];
                        window.location.href = `/dialogs/ingest?asset=${asset}`
                      }} variant={"ghost"}>Ingest</Button>
                    </div>
                  </CardContent>
                </CardHeader>

              </Card>
            </Show>
          </div>
          <Dialog open={isCreateAssetDialogOpen()} onOpenChange={setOpenCreateAssetDialog}>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>{category()}</DialogTitle>
                <DialogDescription>
                  Add an asset to the category
                </DialogDescription>
              </DialogHeader>
              <div class="grid gap-4 py-4">
                <TextField value={newAssetName()} onChange={setNewAssetname}>
                  <TextFieldInput />
                </TextField>
                <div class='text-muted-foreground text-xs'>
                  {newAssetPath()}
                </div>
              </div>
              <Show when={error()}>
                <Callout variant="error">
                  <CalloutTitle>Warning</CalloutTitle>
                  <CalloutContent>
                    {error()}
                  </CalloutContent>
                </Callout>
              </Show>
              <DialogFooter>
                <Button type="submit" onClick={
                  async () => {

                    let result = await doCreate(newAssetPath(), null)
                    console.log(result)

                    if (result == true) {
                      setOpenCreateAssetDialog(false)
                      refetch()
                      setNewAssetname("")
                      setError("")
                    } else {
                      setError(result['error'])
                    }
                  }
                }>Add Asset</Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>


          <Dialog open={isCreateCategoryDialogOpen()} onOpenChange={setOpenCreateCategoryDialog}>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>{category()}</DialogTitle>
                <DialogDescription>
                  Add a new asset category
                </DialogDescription>
              </DialogHeader>
              <div class="grid gap-4 py-4">
                <TextField value={newAssetName()} onChange={setNewAssetname}>
                  <TextFieldInput />
                </TextField>
                <div class='text-muted-foreground text-xs'>
                  {newAssetPath()}
                </div>
              </div>
              <Show when={error()}>
                <Callout variant="error">
                  <CalloutTitle>Warning</CalloutTitle>
                  <CalloutContent>
                    {error()}
                  </CalloutContent>
                </Callout>
              </Show>
              <DialogFooter>
                <Button type="submit" onClick={
                  async () => {

                    let result = await doCreate(null, newAssetPath())
                    console.log(result)

                    if (result == true) {
                      setOpenCreateCategoryDialog(false)
                      refetch()
                      setNewAssetname("")

                    } else {
                      setError(result['error'])
                    }
                  }
                }>Add Category</Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>



        </div>
      </Show>
    </ColorModeProvider >
  );
};


export default App;
