import { createResource, type Component, Show, Switch, Match, createSignal, For } from 'solid-js';

import { getSummary, getAssetTree, doCreate, get, saveChanges } from './api';

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
        <Menubar class='m-1'>
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
        <div class='border-spacing-10 p-3 pb-12'>
          <div class='w-full'>

            <AssetTree assets={assets} contextMenuBuilder={(path, entry) => (
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



      </Show>
    </ColorModeProvider>
  );
};


export default App;
